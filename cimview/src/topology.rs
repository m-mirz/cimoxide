//! Topology edits inside a substation, as text edits across a model set:
//! opening and closing a switch, deleting equipment with what only exists
//! for it, wiring a terminal to another node, adding equipment with its
//! terminals and nodes, inserting a switch into a wire, and whole bays from
//! templates. Each operation gives the edits of every file it changes
//! together (EQ, SSH, and the diagram objects it deletes), to apply as one.
//!
//! New objects go into the files that already hold their container: the EQ
//! file defining it, and the SSH file that writes the most of that EQ
//! file's objects. What only the user can give (ratings, impedances,
//! setpoints) is left out: validation asks for it.
//!
//! [`apply`] takes an operation as JSON, the way an editor sends it, and
//! resolves what it leaves to the substation (the other busbar of a
//! double-busbar bay, the other voltage level of a transformer) from its
//! [`Inside`].

use std::collections::{BTreeSet, HashSet};

use cimmodel::base::ClassDef;
use cimmodel::registry::type_registry;
use serde_json::Value;

use crate::edit::{self, Edits, File};
use crate::grid::Inside;
use crate::index::{Element, IdKind, local};
use crate::text::{Range, TextEdit};
use crate::tree::Tree;

/// What an editor can add, by group, with how many terminals: by the class
/// or the first of its ancestors listed.
const KINDS: &[(&str, &str, usize)] = &[
    ("BusbarSection", "Busbars", 1),
    ("GroundDisconnector", "Switches", 1),
    ("Switch", "Switches", 2),
    ("EnergyConsumer", "Loads and generators", 1),
    ("RotatingMachine", "Loads and generators", 1),
    ("ExternalNetworkInjection", "Loads and generators", 1),
    ("EnergySource", "Loads and generators", 1),
    ("EquivalentInjection", "Loads and generators", 1),
    ("PowerElectronicsConnection", "Loads and generators", 1),
    ("ShuntCompensator", "Shunts", 1),
    ("StaticVarCompensator", "Shunts", 1),
    ("EarthFaultCompensator", "Shunts", 1),
    ("Ground", "Shunts", 1),
    ("PowerTransformer", "Transformers", 2),
    ("SeriesCompensator", "Series devices", 2),
];

/// Fields by which an object only exists for the object it names: deleting
/// that one deletes it too.
const DEPENDENT: &[&str] = &[
    "Terminal.ConductingEquipment",
    "PowerTransformerEnd.PowerTransformer",
    "RatioTapChanger.TransformerEnd",
    "PhaseTapChanger.TransformerEnd",
    "NonlinearShuntCompensatorPoint.NonlinearShuntCompensator",
    "OperationalLimitSet.Terminal",
    "OperationalLimitSet.Equipment",
    "OperationalLimit.OperationalLimitSet",
    "RegulatingControl.Terminal",
    "Measurement.PowerSystemResource",
    "Measurement.Terminal",
    "AnalogValue.Analog",
    "DiscreteValue.Discrete",
    "AccumulatorValue.Accumulator",
    "StringMeasurementValue.StringMeasurement",
    "Location.PowerSystemResources",
    "PositionPoint.Location",
    "DiagramObject.IdentifiedObject",
    "DiagramObjectPoint.DiagramObject",
    "SvPowerFlow.Terminal",
    "SvStatus.ConductingEquipment",
    "SvSwitch.Switch",
    "SvTapStep.TapChanger",
    "SvShuntCompensatorSections.ShuntCompensator",
];

/// The group a class is added under and how many terminals it takes;
/// `None` for what topology editing does not add.
pub fn kind(class: &str) -> Option<(&'static str, usize)> {
    let reg = type_registry();
    let def = reg.schema("cgmes")?.classes.iter().find(|c| c.local == class)?;
    let chain = reg.chain(def);
    KINDS.iter().find(|(k, ..)| chain.iter().any(|c| c.local == *k)).map(|(_, g, n)| (*g, *n))
}

/// What can be added in voltage level (or bay) `level`: (group, class), by
/// group and class.
pub fn palette(files: &[File], level: &str) -> Vec<(&'static str, &'static str)> {
    let mut out: Vec<(&str, &str)> = edit::addable(files, level)
        .into_iter()
        .filter_map(|a| kind(a.class.local).map(|(g, _)| (g, a.class.local)))
        .collect();
    let order = |g: &str| KINDS.iter().position(|(_, k, _)| *k == g).unwrap_or(usize::MAX);
    out.sort_by_key(|(g, c)| (order(g), *c));
    out.dedup();
    out
}

/// A bay as a template lays it out.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Template {
    /// Disconnector, breaker, disconnector from a busbar to the feeder's node.
    Feeder,
    /// A disconnector to each of two busbars, then breaker and disconnector.
    DoubleFeeder,
    /// Disconnector and breaker from a busbar to a transformer, whose other
    /// side is in another voltage level.
    Transformer,
    /// Disconnector, breaker, disconnector between two busbars.
    Coupler,
}

impl Template {
    pub fn parse(name: &str) -> Option<Self> {
        Some(match name {
            "feeder" => Self::Feeder,
            "double-feeder" => Self::DoubleFeeder,
            "transformer" => Self::Transformer,
            "coupler" => Self::Coupler,
            _ => return None,
        })
    }

    /// Every template, with its name and a label.
    pub const ALL: [(&'static str, &'static str); 4] = [
        ("feeder", "Single-busbar feeder"),
        ("double-feeder", "Double-busbar feeder"),
        ("transformer", "Transformer bay"),
        ("coupler", "Bus coupler"),
    ];
}

/// An operation done: its edits, and the object to show for it (what it
/// added), if any.
pub struct Done<'a> {
    pub edits: Edits<'a>,
    pub select: Option<String>,
}

/// Edits being gathered: replacements and deletions by file, and new
/// objects' lines by file, inserted together near an object of theirs.
struct Change<'f, 'a> {
    files: &'f [File<'a>],
    tree: &'f Tree,
    edits: Vec<Vec<TextEdit>>,
    new: Vec<Vec<String>>,
    near: Vec<Option<String>>,
    taken: HashSet<String>,
    /// Names given, so that two new objects are not named alike.
    named: HashSet<String>,
}

impl<'f, 'a> Change<'f, 'a> {
    fn new(files: &'f [File<'a>], tree: &'f Tree) -> Self {
        let n = files.len();
        Self { files, tree, edits: vec![Vec::new(); n], new: vec![Vec::new(); n], near: vec![None; n], taken: HashSet::new(), named: HashSet::new() }
    }

    /// The elements writing `id`, with their file's index.
    fn elements<'s>(&'s self, id: &'s str) -> impl Iterator<Item = (usize, &'f Element)> + 's {
        let files = self.files;
        files.iter().enumerate().flat_map(move |(i, f)| f.index.elements.iter().filter(move |e| e.mrid == id).map(move |e| (i, e)))
    }

    /// The file defining `id` (`rdf:ID`).
    fn defining(&self, id: &str) -> Option<usize> {
        self.elements(id).find(|(_, e)| e.id_kind == Some(IdKind::Id)).map(|(i, _)| i)
    }

    /// `id`'s class, as the file defining it writes it.
    fn class(&self, id: &str) -> Option<&'f str> {
        let (i, e) = self.elements(id).max_by_key(|(_, e)| e.id_kind == Some(IdKind::Id))?;
        Some(self.files[i].index.class_of(&e.qname).map_or(local(&e.qname), |c| c.local))
    }

    /// What `id` names with field `key`, in whichever file writes it.
    fn reference(&self, id: &str, key: &str) -> Option<&'f str> {
        self.elements(id).flat_map(|(_, e)| &e.fields).find(|f| f.key() == key).and_then(|f| f.resource.as_ref()).map(|(m, _)| m.as_str())
    }

    /// `id`'s name, else its mRID.
    fn name(&self, id: &str) -> String {
        self.elements(id).find_map(|(_, e)| e.name.clone()).filter(|n| !n.is_empty()).unwrap_or_else(|| id.to_string())
    }

    /// The objects naming `id` with field `key`.
    fn referrers(&self, id: &str, key: &str) -> BTreeSet<&'f str> {
        self.files
            .iter()
            .flat_map(|f| &f.index.elements)
            .filter(|e| !e.mrid.is_empty() && e.fields.iter().any(|f| f.key() == key && f.resource.as_ref().is_some_and(|(m, _)| m == id)))
            .map(|e| e.mrid.as_str())
            .collect()
    }

    /// The SSH file for what the EQ file `eq` defines: the one writing most of its objects.
    fn ssh_for(&self, eq: usize) -> Option<usize> {
        let defined: HashSet<&str> = self.files[eq].index.elements.iter().filter(|e| e.id_kind == Some(IdKind::Id)).map(|e| e.mrid.as_str()).collect();
        self.files
            .iter()
            .enumerate()
            .filter(|(_, f)| edit::profiles(f).contains(&"SSH"))
            .map(|(i, f)| (f.index.elements.iter().filter(|e| defined.contains(e.mrid.as_str())).count(), i))
            .filter(|(n, _)| *n > 0)
            .max()
            .map(|(_, i)| i)
    }

    /// A fresh mRID.
    fn id(&mut self, random: &mut dyn FnMut() -> [u8; 16]) -> String {
        let id = edit::fresh(self.files, &self.taken, random);
        self.taken.insert(id.clone());
        id
    }

    /// `base` if the set has no object of that name yet, else `base 2`, `base 3`, ….
    fn unique(&mut self, base: &str) -> String {
        let taken = |n: &str| self.named.contains(n) || self.files.iter().flat_map(|f| &f.index.elements).any(|e| e.name.as_deref() == Some(n));
        let mut name = base.to_string();
        let mut k = 2;
        while taken(&name) {
            name = format!("{base} {k}");
            k += 1;
        }
        self.named.insert(name.clone());
        name
    }

    /// A new object of `class` with `values` (field, value as
    /// [`edit::set_field`] takes it): in the EQ file `eq` what its profiles
    /// carry, in its SSH file the rest; inserted near `near`'s objects.
    fn object(&mut self, eq: usize, class: &str, near: &str, values: &[(&str, String)], random: &mut dyn FnMut() -> [u8; 16]) -> Result<String, String> {
        let reg = type_registry();
        let files = self.files;
        let file = &files[eq];
        let def: &'static ClassDef = file
            .index
            .namespaces
            .iter()
            .filter_map(|(_, ns)| reg.ns_table(ns)?.get(class).copied())
            .next()
            .ok_or_else(|| format!("{} binds no namespace with a class {class}", file.name))?;
        let id = self.id(random);
        let ssh = self.ssh_for(eq);
        let declared = edit::profiles(file);
        let in_eq = |key: &str| {
            let origins = reg.attr(def, key).map_or(&[][..], |a| a.origins);
            declared.is_empty() || origins.is_empty() || origins.iter().any(|o| declared.contains(o))
        };
        let mut eq_values: Vec<(&str, &str)> = Vec::new();
        let mut ssh_values: Vec<(&str, &str)> = Vec::new();
        if in_eq("IdentifiedObject.mRID") && reg.attr(def, "IdentifiedObject.mRID").is_some() {
            eq_values.push(("IdentifiedObject.mRID", &id[1..]));
        }
        for (key, value) in values {
            if reg.attr(def, key).is_none() {
                return Err(format!("{class} has no field {key}"));
            }
            if in_eq(key) {
                eq_values.push((key, value));
            } else {
                ssh_values.push((key, value));
            }
        }
        self.new[eq].extend(lines(file, def, &id, &eq_values, false)?);
        self.near[eq].get_or_insert_with(|| near.to_string());
        if let Some(s) = ssh
            && (!ssh_values.is_empty() || def.origins.contains(&"SSH"))
        {
            let lines = lines(&files[s], def, &id, &ssh_values, true)?;
            self.new[s].extend(lines);
            self.near[s].get_or_insert_with(|| near.to_string());
        }
        Ok(id)
    }

    /// A terminal of `equipment` (its `n`th) on `node`, connected.
    fn terminal(&mut self, eq: usize, equipment: &str, name: &str, n: usize, node: &str, random: &mut dyn FnMut() -> [u8; 16]) -> Result<String, String> {
        let values = [
            ("IdentifiedObject.name", format!("{name} T{n}")),
            ("ACDCTerminal.sequenceNumber", n.to_string()),
            ("Terminal.ConductingEquipment", equipment.to_string()),
            ("Terminal.ConnectivityNode", node.to_string()),
            ("ACDCTerminal.connected", "true".into()),
        ];
        self.object(eq, "Terminal", equipment, &values, random)
    }

    /// A new connectivity node in `container` (a voltage level or bay).
    fn node(&mut self, eq: usize, container: &str, name: &str, random: &mut dyn FnMut() -> [u8; 16]) -> Result<String, String> {
        let name = self.unique(name);
        self.object(eq, "ConnectivityNode", container, &[("IdentifiedObject.name", name), ("ConnectivityNode.ConnectivityNodeContainer", container.to_string())], random)
    }

    /// Equipment of `class` named `name` in `container`, a terminal on each of `nodes`.
    fn equipment(&mut self, eq: usize, class: &str, container: &str, name: &str, nodes: &[&str], random: &mut dyn FnMut() -> [u8; 16]) -> Result<String, String> {
        let mut values = vec![("IdentifiedObject.name", name.to_string()), ("Equipment.EquipmentContainer", container.to_string())];
        values.extend(states(class));
        let id = self.object(eq, class, container, &values, random)?;
        for (k, node) in nodes.iter().enumerate() {
            self.terminal(eq, &id, name, k + 1, node, random)?;
        }
        Ok(id)
    }

    /// A two-winding transformer in `substation`, its ends on `ends` (node,
    /// base voltage), each a terminal and a `PowerTransformerEnd`.
    fn transformer(&mut self, eq: usize, substation: &str, name: &str, ends: &[(&str, Option<&str>)], random: &mut dyn FnMut() -> [u8; 16]) -> Result<String, String> {
        let mut values = vec![("IdentifiedObject.name", name.to_string()), ("Equipment.EquipmentContainer", substation.to_string())];
        values.extend(states("PowerTransformer"));
        let id = self.object(eq, "PowerTransformer", substation, &values, random)?;
        for (k, (node, base)) in ends.iter().enumerate() {
            let t = self.terminal(eq, &id, name, k + 1, node, random)?;
            let mut values = vec![
                ("IdentifiedObject.name", format!("{name} end {}", k + 1)),
                ("TransformerEnd.endNumber", (k + 1).to_string()),
                ("TransformerEnd.Terminal", t),
                ("PowerTransformerEnd.PowerTransformer", id.clone()),
            ];
            if let Some(b) = base {
                values.push(("TransformerEnd.BaseVoltage", b.to_string()));
            }
            self.object(eq, "PowerTransformerEnd", &id, &values, random)?;
        }
        Ok(id)
    }

    /// Field `key` of `id` set to `value`, where it is written, else in the
    /// SSH file writing `id` (for an SSH field) or the file defining it.
    fn set(&mut self, id: &str, key: &str, value: &str) -> Result<(), String> {
        let ssh = self.elements(id).find(|(i, _)| edit::profiles(&self.files[*i]).contains(&"SSH")).map(|(i, _)| i);
        let prefer = ssh.or_else(|| self.defining(id)).map_or("", |i| self.files[i].name);
        for (name, edits) in edit::set_field(self.files, prefer, id, key, value)? {
            let i = self.files.iter().position(|f| f.name == name).expect("one of the files");
            self.edits[i].extend(edits);
        }
        Ok(())
    }

    /// `ids` deleted: their elements, and the references to them from what
    /// stays, but for a field an edit here already rewrites.
    fn delete(&mut self, ids: &HashSet<String>) {
        for (i, file) in self.files.iter().enumerate() {
            let mut edits = Vec::new();
            for el in &file.index.elements {
                if ids.contains(&el.mrid) {
                    edits.push(TextEdit::new(file.lines_of(el.full), String::new()));
                    continue;
                }
                for f in el.fields.iter().filter(|f| f.resource.as_ref().is_some_and(|(m, _)| ids.contains(m))) {
                    let r = file.lines_of(f.full);
                    if !self.edits[i].iter().any(|e| inside(e.range, r)) {
                        edits.push(TextEdit::new(r, String::new()));
                    }
                }
            }
            self.edits[i].extend(edits);
        }
    }

    /// What deleting `ids` takes with it: what only exists for them, and a
    /// connectivity node no remaining terminal is on, with its drawing;
    /// terminals in `moving` leave their nodes too, but stay.
    fn removed(&self, ids: &[&str], moving: &[&str]) -> HashSet<String> {
        let mut gone: HashSet<String> = ids.iter().map(|s| s.to_string()).collect();
        let all = || self.files.iter().flat_map(|f| &f.index.elements);
        let dependents = |gone: &mut HashSet<String>| loop {
            let more: Vec<String> = all()
                .filter(|e| !e.mrid.is_empty() && !gone.contains(&e.mrid))
                .filter(|e| e.fields.iter().any(|f| DEPENDENT.contains(&f.key()) && f.resource.as_ref().is_some_and(|(m, _)| gone.contains(m))))
                .map(|e| e.mrid.clone())
                .collect();
            if more.is_empty() {
                break;
            }
            gone.extend(more);
        };
        dependents(&mut gone);
        // The nodes the terminals gone or moving were on.
        let left: BTreeSet<&str> = gone.iter().map(String::as_str).chain(moving.iter().copied()).filter_map(|t| self.reference(t, "Terminal.ConnectivityNode")).collect();
        for node in left {
            if gone.contains(node) {
                continue;
            }
            let held = all().any(|e| {
                !e.mrid.is_empty()
                    && !gone.contains(&e.mrid)
                    && !moving.contains(&e.mrid.as_str())
                    && local(&e.qname) != "DiagramObject"
                    && e.mrid != node
                    && e.fields.iter().any(|f| f.resource.as_ref().is_some_and(|(m, _)| m == node))
            });
            if !held {
                gone.insert(node.to_string());
            }
        }
        dependents(&mut gone);
        gone
    }

    /// The edits gathered, new objects inserted near theirs.
    fn finish(mut self) -> Result<Edits<'a>, String> {
        for (i, lines) in self.new.iter().enumerate() {
            if lines.is_empty() {
                continue;
            }
            let near = self.near[i].as_deref().unwrap_or("");
            let ((edit, _), _) = edit::insertion(&self.files[i], self.tree, near, lines)?;
            self.edits[i].push(edit);
        }
        Ok(self.files.iter().zip(self.edits).filter(|(_, e)| !e.is_empty()).map(|(f, e)| (f.name, e)).collect())
    }

    /// The voltage level a container is or is in: itself, or a bay's.
    fn level_of(&self, container: &str) -> Option<&'f str> {
        match self.class(container)? {
            "Bay" => self.reference(container, "Bay.VoltageLevel"),
            "VoltageLevel" => self.elements(container).next().map(|(_, e)| e.mrid.as_str()),
            _ => None,
        }
    }

    /// The node `id` names: a connectivity node itself, or a busbar
    /// section's (its terminal's).
    fn node_of(&self, id: &str) -> Result<&'f str, String> {
        match self.class(id) {
            Some("ConnectivityNode") => Ok(self.elements(id).next().map(|(_, e)| e.mrid.as_str()).expect("written")),
            Some("TopologicalNode") => Err("this model is bus-branch: topology is edited on connectivity nodes".into()),
            Some(_) => self
                .referrers(id, "Terminal.ConductingEquipment")
                .into_iter()
                .find_map(|t| self.reference(t, "Terminal.ConnectivityNode"))
                .ok_or_else(|| format!("{} is on no connectivity node", self.name(id))),
            None => Err(format!("no file writes `{id}`")),
        }
    }
}

/// The states new equipment of `class` starts in, for the fields its class has: in service, and a
/// switch closed, neither locked nor retained. States, not parameters: what the user must give is
/// left to validation.
fn states(class: &str) -> Vec<(&'static str, String)> {
    let reg = type_registry();
    let Some(def) = reg.schema("cgmes").and_then(|s| s.classes.iter().find(|c| c.local == class)) else { return Vec::new() };
    [("Equipment.inService", "true"), ("Switch.normalOpen", "false"), ("Switch.open", "false"), ("Switch.locked", "false"), ("Switch.retained", "false")]
        .into_iter()
        .filter(|(k, _)| reg.attr(def, k).is_some_and(|a| a.used))
        .map(|(k, v)| (k, v.to_string()))
        .collect()
}

/// `inner` lies within `outer`.
fn inside(inner: Range, outer: Range) -> bool {
    outer.start <= inner.start && inner.end <= outer.end
}

/// An object's element, unindented: an `rdf:ID` (`rdf:about` with `about`)
/// and `values`.
fn lines(file: &File, class: &'static ClassDef, id: &str, values: &[(&str, &str)], about: bool) -> Result<Vec<String>, String> {
    let prefix = file.index.prefix_of(class.ns).ok_or_else(|| format!("{} binds no prefix to {}", file.name, class.ns))?;
    let head = if about { format!("rdf:about=\"#{id}\"") } else { format!("rdf:ID=\"{id}\"") };
    let mut out = vec![format!("<{prefix}:{} {head}>", class.local)];
    for (key, value) in values {
        out.push(format!("  {}", edit::field_element(file, class, key, value)?));
    }
    out.push(format!("</{prefix}:{}>", class.local));
    Ok(out)
}

/// Open or close switch `id`: `Switch.open` in the SSH file writing it.
pub fn switch<'a>(files: &[File<'a>], tree: &Tree, id: &str, open: bool) -> Result<Done<'a>, String> {
    let mut c = Change::new(files, tree);
    if !c.class(id).and_then(kind).is_some_and(|(g, _)| g == "Switches") {
        return Err(format!("{} is no switch", c.name(id)));
    }
    c.set(id, "Switch.open", if open { "true" } else { "false" })?;
    Ok(Done { edits: c.finish()?, select: None })
}

/// Delete equipment `ids`, with what only exists for them (terminals, transformer ends, limits,
/// measurements, diagram objects and their points, …) and the nodes they leave empty.
pub fn delete<'a>(files: &[File<'a>], tree: &Tree, ids: &[&str]) -> Result<Done<'a>, String> {
    let mut c = Change::new(files, tree);
    for id in ids {
        match c.class(id) {
            None => return Err(format!("no file writes `{id}`")),
            Some("ConnectivityNode" | "TopologicalNode") => return Err("a node goes with the last equipment on it: delete that".into()),
            Some(_) => {}
        }
    }
    let gone = c.removed(ids, &[]);
    c.delete(&gone);
    Ok(Done { edits: c.finish()?, select: None })
}

/// Terminal `terminal` connected to `node` (a connectivity node, or a busbar section's): the node
/// it leaves deleted if no other terminal is on it.
pub fn connect<'a>(files: &[File<'a>], tree: &Tree, terminal: &str, node: &str) -> Result<Done<'a>, String> {
    let mut c = Change::new(files, tree);
    let node = c.node_of(node)?;
    let from = c.reference(terminal, "Terminal.ConnectivityNode");
    if from == Some(node) {
        return Err("the terminal is on that node already".into());
    }
    let level = |n: Option<&str>| n.and_then(|n| c.reference(n, "ConnectivityNode.ConnectivityNodeContainer")).and_then(|x| c.level_of(x));
    if from.is_some() && level(from) != level(Some(node)) {
        return Err("the nodes are in different voltage levels".into());
    }
    c.set(terminal, "Terminal.ConnectivityNode", node)?;
    c.set(terminal, "ACDCTerminal.connected", "true")?;
    let gone = c.removed(&[], &[terminal]);
    c.delete(&gone);
    Ok(Done { edits: c.finish()?, select: None })
}

/// Every terminal on node `from` moved to node `into` (or a busbar section's), and `from`
/// deleted with its drawing: two nodes joined.
pub fn merge<'a>(files: &[File<'a>], tree: &Tree, from: &str, into: &str) -> Result<Done<'a>, String> {
    let mut c = Change::new(files, tree);
    let (from, into) = (c.node_of(from)?, c.node_of(into)?);
    if from == into {
        return Err("that is the same node".into());
    }
    let level = |n: &str| c.reference(n, "ConnectivityNode.ConnectivityNodeContainer").and_then(|x| c.level_of(x));
    if level(from) != level(into) {
        return Err("the nodes are in different voltage levels".into());
    }
    let terminals: Vec<&str> = c.referrers(from, "Terminal.ConnectivityNode").into_iter().collect();
    for t in &terminals {
        c.set(t, "Terminal.ConnectivityNode", into)?;
    }
    let gone = c.removed(&[from], &terminals);
    c.delete(&gone);
    Ok(Done { edits: c.finish()?, select: Some(into.to_string()) })
}

/// Terminal `terminal` off its node, on a new node of its own in the same container, not
/// connected: the node it leaves deleted if no other terminal is on it.
pub fn disconnect<'a>(files: &[File<'a>], tree: &Tree, terminal: &str, random: &mut dyn FnMut() -> [u8; 16]) -> Result<Done<'a>, String> {
    let mut c = Change::new(files, tree);
    let from = c.reference(terminal, "Terminal.ConnectivityNode").ok_or("the terminal is on no connectivity node")?;
    let container = c.reference(from, "ConnectivityNode.ConnectivityNodeContainer").ok_or("its node is in no container")?;
    let eq = c.defining(from).ok_or("no file defines its node")?;
    let node = c.node(eq, container, &format!("{} node", c.name(terminal)), random)?;
    c.set(terminal, "Terminal.ConnectivityNode", &node)?;
    c.set(terminal, "ACDCTerminal.connected", "false")?;
    let gone = c.removed(&[], &[terminal]);
    c.delete(&gone);
    Ok(Done { edits: c.finish()?, select: Some(node) })
}

/// Equipment of `class` added in voltage level (or bay) `level`, named `name` (else after its
/// class), with its terminals, each on a node of its own; a transformer in the level's
/// substation, its second end in voltage level `other`.
pub fn add<'a>(
    files: &[File<'a>],
    tree: &Tree,
    level: &str,
    class: &str,
    name: Option<&str>,
    other: Option<&str>,
    random: &mut dyn FnMut() -> [u8; 16],
) -> Result<Done<'a>, String> {
    let mut c = Change::new(files, tree);
    let (_, terminals) = kind(class).ok_or_else(|| format!("a {class} is not added here"))?;
    let vl = c.level_of(level).ok_or("equipment is added in a voltage level or a bay")?;
    let eq = c.defining(level).ok_or("no file defines the voltage level")?;
    let name = c.unique(name.map(str::trim).filter(|n| !n.is_empty()).unwrap_or(class));
    let id = if class == "PowerTransformer" {
        let other = other.ok_or("a transformer needs a second voltage level in the substation")?;
        let substation = c.reference(vl, "VoltageLevel.Substation").ok_or("the voltage level is in no substation")?;
        let a = c.node(eq, level, &format!("{name} node 1"), random)?;
        let b = c.node(eq, other, &format!("{name} node 2"), random)?;
        let ends = [(a.as_str(), c.reference(vl, "VoltageLevel.BaseVoltage")), (b.as_str(), c.reference(other, "VoltageLevel.BaseVoltage"))];
        c.transformer(eq, substation, &name, &ends, random)?
    } else {
        let mut nodes = Vec::new();
        for k in 1..=terminals {
            nodes.push(c.node(eq, level, &format!("{name} node {k}"), random)?);
        }
        let nodes: Vec<&str> = nodes.iter().map(String::as_str).collect();
        c.equipment(eq, class, level, &name, &nodes, random)?
    };
    Ok(Done { edits: c.finish()?, select: Some(id) })
}

/// A switch of `class` inserted into terminal `terminal`'s wire: on the terminal's node with
/// one side, on a new node with the other, and the terminal moved to that new node.
pub fn insert<'a>(files: &[File<'a>], tree: &Tree, terminal: &str, class: &str, random: &mut dyn FnMut() -> [u8; 16]) -> Result<Done<'a>, String> {
    let mut c = Change::new(files, tree);
    if kind(class) != Some(("Switches", 2)) {
        return Err(format!("a {class} cannot be inserted into a wire"));
    }
    let node = c.reference(terminal, "Terminal.ConnectivityNode").ok_or("the terminal is on no connectivity node")?;
    let container = c.reference(node, "ConnectivityNode.ConnectivityNodeContainer").ok_or("its node is in no container")?;
    let eq = c.defining(node).ok_or("no file defines its node")?;
    let name = c.unique(class);
    let new = c.node(eq, container, &format!("{name} node"), random)?;
    let id = c.equipment(eq, class, container, &name, &[node, &new], random)?;
    c.set(terminal, "Terminal.ConnectivityNode", &new)?;
    Ok(Done { edits: c.finish()?, select: Some(id) })
}

/// A bay laid out as `template` says, in a new `Bay` of busbar `bus`'s voltage level, hanging
/// from it: `second`, the other busbar (a double-busbar feeder, a coupler) or the voltage level
/// of the transformer's other side.
pub fn bay<'a>(
    files: &[File<'a>],
    tree: &Tree,
    template: Template,
    bus: &str,
    second: Option<&str>,
    random: &mut dyn FnMut() -> [u8; 16],
) -> Result<Done<'a>, String> {
    let mut c = Change::new(files, tree);
    let bus = c.node_of(bus)?;
    let container = c.reference(bus, "ConnectivityNode.ConnectivityNodeContainer").ok_or("the busbar's node is in no container")?;
    let vl = c.level_of(container).ok_or("the busbar is in no voltage level")?;
    let eq = c.defining(vl).ok_or("no file defines the voltage level")?;
    let named = format!("{} bay", c.name(vl));
    let base = c.unique(&named);
    let bay = c.object(eq, "Bay", vl, &[("IdentifiedObject.name", base.clone()), ("Bay.VoltageLevel", vl.to_string())], random)?;
    let label = |q: &str| format!("{base} {q}");
    let names = [label("N1"), label("N2"), label("N3")];
    let n = |c: &mut Change<'_, 'a>, k: usize, random: &mut dyn FnMut() -> [u8; 16]| c.node(eq, &bay, &names[k - 1], random);
    match template {
        Template::Feeder => {
            let (n1, n2, n3) = (n(&mut c, 1, random)?, n(&mut c, 2, random)?, n(&mut c, 3, random)?);
            c.equipment(eq, "Disconnector", &bay, &label("Q1"), &[bus, &n1], random)?;
            c.equipment(eq, "Breaker", &bay, &label("Q0"), &[&n1, &n2], random)?;
            c.equipment(eq, "Disconnector", &bay, &label("Q9"), &[&n2, &n3], random)?;
        }
        Template::DoubleFeeder => {
            let other = c.node_of(second.ok_or("a double-busbar feeder needs a second busbar in the voltage level")?)?;
            let (n1, n2, n3) = (n(&mut c, 1, random)?, n(&mut c, 2, random)?, n(&mut c, 3, random)?);
            c.equipment(eq, "Disconnector", &bay, &label("Q1"), &[bus, &n1], random)?;
            c.equipment(eq, "Disconnector", &bay, &label("Q2"), &[other, &n1], random)?;
            c.equipment(eq, "Breaker", &bay, &label("Q0"), &[&n1, &n2], random)?;
            c.equipment(eq, "Disconnector", &bay, &label("Q9"), &[&n2, &n3], random)?;
        }
        Template::Coupler => {
            let other = c.node_of(second.ok_or("a coupler needs a second busbar in the voltage level")?)?;
            let (n1, n2) = (n(&mut c, 1, random)?, n(&mut c, 2, random)?);
            c.equipment(eq, "Disconnector", &bay, &label("Q1"), &[bus, &n1], random)?;
            c.equipment(eq, "Breaker", &bay, &label("Q0"), &[&n1, &n2], random)?;
            c.equipment(eq, "Disconnector", &bay, &label("Q2"), &[&n2, other], random)?;
        }
        Template::Transformer => {
            let other = second.ok_or("a transformer bay needs a second voltage level in the substation")?;
            let substation = c.reference(vl, "VoltageLevel.Substation").ok_or("the voltage level is in no substation")?;
            let (n1, n2) = (n(&mut c, 1, random)?, n(&mut c, 2, random)?);
            let far = c.node(eq, other, &label("T1 node 2"), random)?;
            c.equipment(eq, "Disconnector", &bay, &label("Q1"), &[bus, &n1], random)?;
            c.equipment(eq, "Breaker", &bay, &label("Q0"), &[&n1, &n2], random)?;
            let ends = [(n2.as_str(), c.reference(vl, "VoltageLevel.BaseVoltage")), (far.as_str(), c.reference(other, "VoltageLevel.BaseVoltage"))];
            c.transformer(eq, substation, &label("T1"), &ends, random)?;
        }
    }
    Ok(Done { edits: c.finish()?, select: Some(bay) })
}

/// An operation as an editor sends it, done on substation `inside`:
///
/// - `{"op": "switch", "id", "open"}`
/// - `{"op": "delete", "ids": […]}`
/// - `{"op": "connect", "terminal", "node"}`, `{"op": "merge", "from", "into"}`,
///   `{"op": "disconnect", "terminal"}`
/// - `{"op": "add", "class", "level", "name"?}`
/// - `{"op": "insert", "class", "terminal"}`
/// - `{"op": "bay", "template", "bus"}`
///
/// What it leaves open comes from the substation: a transformer's other voltage level (the
/// nearest in voltage, lower first), a bay's second busbar (the voltage level's other bus).
pub fn apply<'a>(files: &[File<'a>], tree: &Tree, inside: &Inside, op: &Value, random: &mut dyn FnMut() -> [u8; 16]) -> Result<Done<'a>, String> {
    let s = |key: &str| op[key].as_str().ok_or_else(|| format!("the operation has no `{key}`"));
    match s("op")? {
        "switch" => switch(files, tree, s("id")?, op["open"].as_bool().ok_or("the operation has no `open`")?),
        "delete" => {
            let ids: Vec<&str> = op["ids"].as_array().into_iter().flatten().filter_map(Value::as_str).collect();
            if ids.is_empty() {
                return Err("nothing to delete".into());
            }
            delete(files, tree, &ids)
        }
        "connect" => connect(files, tree, s("terminal")?, s("node")?),
        "merge" => merge(files, tree, s("from")?, s("into")?),
        "disconnect" => disconnect(files, tree, s("terminal")?, random),
        "add" => {
            let (class, level) = (s("class")?, s("level")?);
            let other = if class == "PowerTransformer" { other_level(inside, level) } else { None };
            add(files, tree, level, class, op["name"].as_str(), other, random)
        }
        "insert" => insert(files, tree, s("terminal")?, s("class")?, random),
        "bay" => {
            let template = Template::parse(s("template")?).ok_or("no such bay template")?;
            let bus = s("bus")?;
            let second = match template {
                Template::Feeder => None,
                Template::Transformer => {
                    let c = Change::new(files, tree);
                    let node = c.node_of(bus)?;
                    let level = c.reference(node, "ConnectivityNode.ConnectivityNodeContainer").and_then(|x| c.level_of(x)).unwrap_or_default();
                    other_level(inside, level)
                }
                Template::DoubleFeeder | Template::Coupler => other_bus(inside, bus),
            };
            bay(files, tree, template, bus, second, random)
        }
        other => Err(format!("no operation `{other}`")),
    }
}

/// The texts of `files` after `edits`, every file, by name.
pub fn after(files: &[File], edits: &Edits) -> Vec<(String, String)> {
    files
        .iter()
        .map(|f| {
            let mine: Vec<TextEdit> = edits.iter().filter(|(n, _)| *n == f.name).flat_map(|(_, e)| e.iter().cloned()).collect();
            let text = if mine.is_empty() { f.text.to_string() } else { crate::text::apply(f.text, &mine) };
            (f.name.to_string(), text)
        })
        .collect()
}

/// Edits by file name, owned.
pub type Owned = Vec<(String, Vec<TextEdit>)>;

/// The DL diagram `diagram` drawn anew for substation `substation` of the set `texts` (its
/// files by name, as an edit left them): what it draws where it is, what it does not draw yet
/// placed ([`crate::single_line::complete`], the first `at`), every terminal routed anew; the
/// lines leaving the substation as the diagram draws them. The edits to `texts`, by name; none
/// if the diagram does not draw the substation.
pub fn redraw(texts: &[(String, String)], substation: &str, diagram: &str, at: Option<(f64, f64)>, random: &mut dyn FnMut() -> [u8; 16]) -> Result<Owned, String> {
    let datasets: Vec<cimmodel::CimDataset> = texts.iter().filter_map(|(_, t)| cimmodel::CimDataset::decode_str(t).ok()).collect();
    let grid = crate::grid::build(&crate::grid::Model::new(&datasets));
    let Some(inside) = grid.inside_of(substation) else { return Ok(Vec::new()) };
    let id = format!("dl:{diagram}");
    let Some(shown) = inside.layouts.iter().find(|l| l.id == id) else { return Ok(Vec::new()) };
    let done = crate::single_line::complete(inside, shown, at);
    let objects: Vec<edit::Drawn> = crate::single_line::diagram_objects(inside, &done)
        .into_iter()
        .filter(|(m, _, _)| !inside.items.iter().any(|it| it.id == *m && !it.far.is_empty()))
        .collect();
    if objects.is_empty() {
        return Ok(Vec::new());
    }
    let lines: Vec<crate::text::LineIndex> = texts.iter().map(|(_, t)| crate::text::LineIndex::new(t)).collect();
    let indexes: Vec<crate::index::Index> = texts.iter().map(|(_, t)| crate::index::Index::build(t)).collect();
    let files: Vec<File> = texts.iter().zip(&lines).zip(&indexes).map(|(((n, t), l), i)| File { name: n, text: t, lines: l, index: i }).collect();
    let written = edit::diagram(&files, "", Some(diagram), "", &objects, random)?;
    Ok(written.edits.into_iter().map(|(n, e)| (n.to_string(), e)).collect())
}

/// The voltage level of `inside` nearest in voltage to `level`, other than it: a lower one first.
pub fn other_level<'i>(inside: &'i Inside, level: &str) -> Option<&'i str> {
    let kv = inside.groups.iter().find(|g| g.id == level).and_then(|g| g.kv).unwrap_or(0.0);
    inside
        .groups
        .iter()
        .filter(|g| g.id != level)
        .min_by(|a, b| {
            let d = |g: &crate::grid::Group| {
                let v = g.kv.unwrap_or(0.0);
                ((v - kv).abs(), v > kv)
            };
            let (x, y) = (d(a), d(b));
            x.1.cmp(&y.1).then(x.0.total_cmp(&y.0))
        })
        .map(|g| g.id.as_str())
}

/// Another bus of the voltage level of bus `bus` (a node, or a busbar section's item).
pub fn other_bus<'i>(inside: &'i Inside, bus: &str) -> Option<&'i str> {
    let buses = crate::single_line::buses(inside);
    let junction = |i: usize| if inside.items[i].junction { Some(i) } else { inside.links.iter().find(|l| l.item == i).map(|l| l.junction) };
    let at = inside.items.iter().position(|it| it.id == bus).and_then(junction)?;
    let group = inside.items[at].group;
    let mut others: Vec<usize> = (0..inside.items.len()).filter(|&i| i != at && inside.items[i].junction && buses[i] && inside.items[i].group == group).collect();
    others.sort_by_key(|&i| &inside.items[i].label);
    others.first().map(|&i| inside.items[i].id.as_str())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::path::{Path, PathBuf};
    use std::sync::Arc;

    use super::*;
    use crate::index::Index;
    use crate::testing::{Set, collect, validate};
    use crate::text::LineIndex;

    type Read = Vec<(PathBuf, Arc<str>)>;

    /// MicroGrid's Type1 files (BE and NL, with SSH, TP, SV and a DL file).
    fn micro_grid() -> Read {
        collect(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../CGMES-Test-Configurations/v3.0/MicroGrid/MicroGrid-Type1/MicroGrid-Type1-Merged"))
    }

    /// The violations of `read`, as `class property: message`, for a failing test to say.
    fn found(read: &Read) -> Vec<String> {
        let datasets: Vec<cimmodel::CimDataset> = read.iter().map(|(_, t)| cimmodel::CimDataset::decode_str(t).unwrap()).collect();
        let cfg = cimvalidation::combined_config(&datasets, None, None, true, true, Vec::new());
        let mut out: Vec<String> = cimvalidation::validate_files_traced(datasets, &cfg)
            .into_iter()
            .filter(|(_, v)| v.severity.to_lowercase().contains("violation"))
            .map(|(_, v)| format!("{} {} {}: {}", v.rule_id, v.class, v.property, v.message))
            .collect();
        out.sort();
        out
    }

    /// The violations `after` has that `before` has not, but for the state solved (SV) and the
    /// topology (TP), stale after a topology edit, and `blank`: parameters left to the user.
    fn more(before: &Read, after: &Read, blank: &[&str]) -> Vec<String> {
        let was = found(before);
        let stale = |v: &str| v.starts_with("sv") || v.starts_with("tp");
        found(after).into_iter().filter(|v| !was.contains(v) && !stale(v) && !blank.iter().any(|b| v.contains(&format!(" {b}: Missing")))).collect()
    }

    /// No violation more than before, as [`more`] counts them.
    fn none_more(before: &Read, after: &Read, blank: &[&str]) {
        let more = more(before, after, blank);
        assert!(more.is_empty(), "{more:#?}");
    }

    /// The objects the files define, by class.
    fn count(read: &Read, class: &str) -> usize {
        read.iter().flat_map(|(_, t)| Index::build(t).elements).filter(|e| e.id_kind == Some(IdKind::Id) && local(&e.qname) == class).count()
    }

    fn random() -> impl FnMut() -> [u8; 16] {
        let mut counter = 0u64;
        move || {
            counter += 1;
            let mut b = [0u8; 16];
            b[..8].copy_from_slice(&counter.to_le_bytes());
            b
        }
    }

    /// What an operation did: the set before and after, the files after, what it selects.
    struct Ran {
        before: Set,
        after: Set,
        read: Read,
        select: Option<String>,
    }

    /// The operation `op` done on substation `substation`, the files edited and read again.
    fn run(read: &Read, substation: &str, op: impl Fn(&[File], &Inside) -> Value) -> Ran {
        let set = validate(read);
        let names: Vec<String> = read.iter().map(|(p, _)| p.to_string_lossy().into_owned()).collect();
        let lines: Vec<LineIndex> = read.iter().map(|(_, t)| LineIndex::new(t)).collect();
        let indexes: Vec<Index> = read.iter().map(|(_, t)| Index::build(t)).collect();
        let files: Vec<File> = (0..read.len()).map(|i| File { name: &names[i], text: &read[i].1, lines: &lines[i], index: &indexes[i] }).collect();
        let inside = inside_of(&set, substation);
        let mut random = random();
        let op = op(&files, inside);
        let done = apply(&files, &set.tree, inside, &op, &mut random).unwrap_or_else(|e| panic!("{op}: {e}"));
        let edited: Read = read
            .iter()
            .zip(&names)
            .map(|((p, t), name)| {
                let edits: Vec<_> = done.edits.iter().filter(|(n, _)| n == name).flat_map(|(_, e)| e.clone()).collect();
                (p.clone(), Arc::from(crate::text::apply(t, &edits)))
            })
            .collect();
        let after = validate(&edited);
        Ran { before: set, after, read: edited, select: done.select }
    }

    fn inside_of<'s>(set: &'s Set, substation: &str) -> &'s Inside {
        let n = set.grid.nodes.iter().position(|n| n.label == substation).unwrap();
        &set.grid.inside[&n]
    }

    /// The terminals of item `id`, as (terminal, node's id).
    fn wires(inside: &Inside, id: &str) -> Vec<(String, String)> {
        let i = inside.items.iter().position(|it| it.id == id).unwrap_or_else(|| panic!("no {id}"));
        inside.links.iter().filter(|l| l.item == i).map(|l| (l.id.clone(), inside.items[l.junction].id.clone())).collect()
    }

    fn switches(set: &Set, substation: &str) -> (usize, usize) {
        let items = &inside_of(set, substation).items;
        (items.iter().filter(|it| it.open == Some(false)).count(), items.iter().filter(|it| it.open == Some(true)).count())
    }

    #[test]
    fn a_switch_opens_and_closes() {
        let read = micro_grid();
        let ran = run(&read, "Anvers", |_, inside| {
            let b = inside.items.iter().find(|it| it.open == Some(false)).expect("a closed switch");
            serde_json::json!({ "op": "switch", "id": b.id, "open": true })
        });
        let ((c0, o0), (c1, o1)) = (switches(&ran.before, "Anvers"), switches(&ran.after, "Anvers"));
        assert_eq!((c1, o1), (c0 - 1, o0 + 1));
        none_more(&read, &ran.read, &[]);
    }

    #[test]
    fn deleting_equipment_takes_its_terminals_and_drawing() {
        let read = micro_grid();
        let ran = run(&read, "Anvers", |_, inside| {
            let b = inside.items.iter().find(|it| it.class == "Breaker").unwrap();
            serde_json::json!({ "op": "delete", "ids": [b.id] })
        });
        let was = inside_of(&ran.before, "Anvers");
        let b = was.items.iter().find(|it| it.class == "Breaker").unwrap();
        for (_, text) in &ran.read {
            assert!(!text.contains(&format!("#{}\"", b.id)), "no reference to it is left");
        }
        assert_eq!(count(&ran.read, "Terminal"), count(&read, "Terminal") - 2);
        assert!(count(&ran.read, "DiagramObject") < count(&read, "DiagramObject"), "its drawing goes");
        let now = inside_of(&ran.after, "Anvers");
        assert_eq!(now.items.iter().filter(|it| !it.junction).count(), was.items.iter().filter(|it| !it.junction).count() - 1);
        none_more(&read, &ran.read, &[]);
    }

    #[test]
    fn a_node_left_empty_goes() {
        let read = micro_grid();
        // A load added, on a node of its own, and deleted again: its node goes with it.
        let added = run(&read, "Anvers", |_, inside| serde_json::json!({ "op": "add", "class": "EnergyConsumer", "level": inside.groups[0].id }));
        let id = added.select.clone().unwrap();
        let gone = run(&added.read, "Anvers", |_, _| serde_json::json!({ "op": "delete", "ids": [id] }));
        for class in ["ConnectivityNode", "Terminal", "EnergyConsumer"] {
            assert_eq!(count(&gone.read, class), count(&read, class), "{class}");
        }
        let texts = |r: &Read| r.iter().map(|(_, t)| t.to_string()).collect::<Vec<_>>();
        assert_eq!(texts(&gone.read), texts(&read), "the files as they were");
    }

    #[test]
    fn a_node_is_not_deleted_alone() {
        let read = micro_grid();
        let set = validate(&read);
        let names: Vec<String> = read.iter().map(|(p, _)| p.to_string_lossy().into_owned()).collect();
        let lines: Vec<LineIndex> = read.iter().map(|(_, t)| LineIndex::new(t)).collect();
        let indexes: Vec<Index> = read.iter().map(|(_, t)| Index::build(t)).collect();
        let files: Vec<File> = (0..read.len()).map(|i| File { name: &names[i], text: &read[i].1, lines: &lines[i], index: &indexes[i] }).collect();
        let node = inside_of(&set, "Anvers").items.iter().find(|it| it.junction).unwrap();
        assert!(delete(&files, &set.tree, &[&node.id]).is_err());
    }

    #[test]
    fn equipment_added_comes_with_terminals_and_nodes() {
        let read = micro_grid();
        let ran = run(&read, "Anvers", |_, inside| serde_json::json!({ "op": "add", "class": "Breaker", "level": inside.groups[0].id }));
        let id = ran.select.clone().unwrap();
        let now = inside_of(&ran.after, "Anvers");
        let added = now.items.iter().find(|it| it.id == id).expect("the breaker, drawn");
        assert_eq!((added.class, added.open, added.label.as_str()), ("Breaker", Some(false), "Breaker"));
        let w = wires(now, &id);
        assert_eq!(w.len(), 2);
        assert_ne!(w[0].1, w[1].1, "each terminal on a node of its own");
        assert_eq!(count(&ran.read, "Terminal"), count(&read, "Terminal") + 2);
        // Its state in the SSH file of its EQ file.
        let ssh = ran.read.iter().find(|(p, t)| p.to_string_lossy().contains("_SSH_") && t.contains(&format!("rdf:about=\"#{id}\""))).expect("in an SSH file");
        assert!(ssh.0.to_string_lossy().contains("_BE_"));
        none_more(&read, &ran.read, &[]);
    }

    #[test]
    fn rewiring_moves_a_terminal_and_drops_the_node_it_left() {
        let read = micro_grid();
        // A breaker added, then its first node joined to a busbar.
        let added = run(&read, "Anvers", |_, inside| serde_json::json!({ "op": "add", "class": "Breaker", "level": inside.groups[0].id }));
        let id = added.select.clone().unwrap();
        let now = inside_of(&added.after, "Anvers");
        let level = now.items.iter().find(|it| it.id == id).unwrap().group;
        let bus = now.items.iter().find(|it| it.class == "BusbarSection" && it.group == level).expect("a busbar").id.clone();
        let bar_node = wires(now, &bus)[0].1.clone();
        let from = wires(now, &id)[0].1.clone();
        let merged = run(&added.read, "Anvers", |_, _| serde_json::json!({ "op": "merge", "from": from, "into": bus }));
        let now = inside_of(&merged.after, "Anvers");
        let on: Vec<String> = wires(now, &id).into_iter().map(|(_, n)| n).collect();
        assert!(on.contains(&bar_node), "{on:?} on {bar_node}");
        assert_eq!(count(&merged.read, "ConnectivityNode"), count(&added.read, "ConnectivityNode") - 1);
        none_more(&added.read, &merged.read, &[]);

        // Off again, onto a node of its own, not connected.
        let t = wires(now, &id).into_iter().find(|(_, n)| *n == bar_node).unwrap().0;
        let off = run(&merged.read, "Anvers", |_, _| serde_json::json!({ "op": "disconnect", "terminal": t }));
        let now = inside_of(&off.after, "Anvers");
        let link = now.links.iter().find(|l| l.id == t).unwrap();
        assert_eq!(link.connected, Some(false));
        assert_eq!(Some(&now.items[link.junction].id), off.select.as_ref());
        assert_eq!(count(&off.read, "ConnectivityNode"), count(&merged.read, "ConnectivityNode") + 1);

        // And on the busbar again: the node of its own goes.
        let back = run(&off.read, "Anvers", |_, _| serde_json::json!({ "op": "connect", "terminal": t, "node": bar_node }));
        let now = inside_of(&back.after, "Anvers");
        let link = now.links.iter().find(|l| l.id == t).unwrap();
        assert_eq!((now.items[link.junction].id.as_str(), link.connected), (bar_node.as_str(), Some(true)));
        assert_eq!(count(&back.read, "ConnectivityNode"), count(&merged.read, "ConnectivityNode"));
        none_more(&merged.read, &back.read, &[]);
    }

    #[test]
    fn a_switch_is_inserted_into_a_wire() {
        let read = micro_grid();
        let ran = run(&read, "Anvers", |_, inside| {
            let i = inside.items.iter().position(|it| it.class == "ACLineSegment").unwrap();
            let t = inside.links.iter().find(|l| l.item == i).unwrap();
            serde_json::json!({ "op": "insert", "class": "Disconnector", "terminal": t.id })
        });
        let now = inside_of(&ran.after, "Anvers");
        assert_eq!(wires(now, ran.select.as_ref().unwrap()).len(), 2);
        assert_eq!(count(&ran.read, "ConnectivityNode"), count(&read, "ConnectivityNode") + 1);
        none_more(&read, &ran.read, &[]);
    }

    #[test]
    fn bays_come_from_templates() {
        let read = micro_grid();
        for (template, added, transformers, at) in [("feeder", 3, 0, "Anvers"), ("double-feeder", 4, 0, "Anvers"), ("coupler", 3, 0, "Anvers"), ("transformer", 2, 1, "PP_Brussels")] {
            let ran = run(&read, at, |_, inside| {
                let bus = inside.items.iter().find(|it| it.class == "BusbarSection").unwrap();
                serde_json::json!({ "op": "bay", "template": template, "bus": bus.id })
            });
            assert_eq!(switches(&ran.after, at).0, switches(&ran.before, at).0 + added, "{template}");
            assert_eq!(count(&ran.read, "Bay"), count(&read, "Bay") + 1);
            assert_eq!(count(&ran.read, "PowerTransformer"), count(&read, "PowerTransformer") + transformers);
            none_more(&read, &ran.read, TRANSFORMER);
        }
    }

    #[test]
    fn a_transformer_joins_two_levels() {
        let read = micro_grid();
        let ran = run(&read, "PP_Brussels", |_, inside| serde_json::json!({ "op": "add", "class": "PowerTransformer", "level": inside.groups[0].id }));
        let now = inside_of(&ran.after, "PP_Brussels");
        let levels: BTreeSet<Option<usize>> =
            wires(now, ran.select.as_ref().unwrap()).iter().map(|(_, n)| now.items.iter().find(|it| &it.id == n).unwrap().group).collect();
        assert_eq!(levels.len(), 2, "{levels:?}");
        assert_eq!(count(&ran.read, "PowerTransformerEnd"), count(&read, "PowerTransformerEnd") + 2);
        none_more(&read, &ran.read, TRANSFORMER);
    }

    /// What a transformer added leaves to the user.
    const TRANSFORMER: &[&str] = &["PowerTransformer.isPartOfGeneratorUnit", "PowerTransformerEnd.r", "PowerTransformerEnd.x", "PowerTransformerEnd.ratedU", "PowerTransformerEnd.ratedS", "PowerTransformerEnd.connectionKind", "PowerTransformerEnd.b", "PowerTransformerEnd.g", "PowerTransformerEnd.r0", "PowerTransformerEnd.x0", "PowerTransformerEnd.b0", "PowerTransformerEnd.g0", "PowerTransformerEnd.phaseAngleClock", "TransformerEnd.grounded"];

    #[test]
    fn the_palette_has_what_a_level_takes() {
        let read = micro_grid();
        let _ = run(&read, "Anvers", |files, inside| {
            let p = palette(files, &inside.groups[0].id);
            let classes: Vec<&str> = p.iter().map(|(_, c)| *c).collect();
            for c in ["Breaker", "Disconnector", "BusbarSection", "EnergyConsumer", "SynchronousMachine", "LinearShuntCompensator", "PowerTransformer"] {
                assert!(classes.contains(&c), "{c}: {classes:?}");
            }
            assert!(!classes.contains(&"ACLineSegment"));
            assert_eq!(p[0].0, "Busbars");
            let _: HashMap<(), ()> = HashMap::new();
            serde_json::json!({ "op": "switch", "id": inside.items.iter().find(|it| it.open.is_some()).unwrap().id, "open": false })
        });
    }

    #[test]
    fn a_diagram_keeps_its_positions_and_places_what_is_new() {
        let read = micro_grid();
        let ran = run(&read, "Anvers", |_, inside| {
            let bus = inside.items.iter().find(|it| it.class == "BusbarSection").unwrap();
            serde_json::json!({ "op": "bay", "template": "feeder", "bus": bus.id })
        });
        let (was, now) = (inside_of(&ran.before, "Anvers"), inside_of(&ran.after, "Anvers"));
        let old = was.layouts.iter().find(|l| l.kind == "diagram").expect("MicroGrid draws Anvers");
        let drawn = now.layouts.iter().find(|l| l.id == old.id).unwrap();
        assert!(drawn.nodes.len() < now.items.len(), "the diagram lacks the bay");
        let bar = now.items.iter().position(|it| it.class == "BusbarSection").unwrap();
        let at = drawn.nodes[&bar];
        let done = crate::single_line::complete(now, drawn, Some(at));
        assert_eq!(done.nodes.len(), now.items.len(), "everything placed");
        let index = |l: &crate::grid::Layout, inside: &Inside| -> HashMap<String, (f64, f64)> { l.nodes.iter().map(|(&i, &p)| (inside.items[i].id.clone(), p)).collect() };
        let (before, after) = (index(old, was), index(&done, now));
        for (id, p) in &before {
            // Equipment stays; junctions and bars follow what hangs from them.
            if was.items.iter().any(|it| &it.id == id && !it.junction) && !old.sizes.keys().any(|&i| &was.items[i].id == id) {
                let q = after[id];
                assert!((p.0 - q.0).abs() < 1e-6 && (p.1 - q.1).abs() < 1e-6, "{id} stays: {p:?} → {q:?}");
            }
        }
        // The bay's switches in a column, below the bar.
        let new: Vec<usize> = (0..now.items.len()).filter(|i| !before.contains_key(&now.items[*i].id) && now.items[*i].open.is_some()).collect();
        assert_eq!(new.len(), 3);
        let flip = if drawn.flip_y { -1.0 } else { 1.0 };
        for i in &new {
            assert!(flip * (done.nodes[i].1 - at.1) > 0.0, "{} below the bar", now.items[*i].label);
        }
        let xs: BTreeSet<i64> = new.iter().map(|i| done.nodes[i].0.round() as i64).collect();
        assert_eq!(xs.len(), 1, "in one column: {xs:?}");
    }

    #[test]
    fn a_diagram_is_redrawn_with_the_edit() {
        let read = micro_grid();
        let set = validate(&read);
        let names: Vec<String> = read.iter().map(|(p, _)| p.to_string_lossy().into_owned()).collect();
        let lines: Vec<LineIndex> = read.iter().map(|(_, t)| LineIndex::new(t)).collect();
        let indexes: Vec<Index> = read.iter().map(|(_, t)| Index::build(t)).collect();
        let files: Vec<File> = (0..read.len()).map(|i| File { name: &names[i], text: &read[i].1, lines: &lines[i], index: &indexes[i] }).collect();
        let inside = inside_of(&set, "Anvers");
        let substation = set.grid.nodes.iter().find(|n| n.label == "Anvers").unwrap().id.clone();
        let diagram = inside.layouts.iter().find(|l| l.kind == "diagram").unwrap().id.trim_start_matches("dl:").to_string();
        let bus = inside.items.iter().find(|it| it.class == "BusbarSection").unwrap();
        let mut random = random();
        let done = apply(&files, &set.tree, inside, &serde_json::json!({ "op": "bay", "template": "feeder", "bus": bus.id }), &mut random).unwrap();
        let texts = after(&files, &done.edits);
        let drawn = redraw(&texts, &substation, &diagram, None, &mut random).unwrap();
        assert_eq!(drawn.len(), 1, "the DL file alone");
        let edited: Read = texts
            .iter()
            .map(|(n, t)| {
                let e: Vec<TextEdit> = drawn.iter().filter(|(m, _)| m == n).flat_map(|(_, e)| e.clone()).collect();
                (PathBuf::from(n), Arc::from(crate::text::apply(t, &e)))
            })
            .collect();
        let again = validate(&edited);
        let now = inside_of(&again, "Anvers");
        let shown = now.layouts.iter().find(|l| l.id == format!("dl:{diagram}")).unwrap();
        assert_eq!(shown.nodes.len(), now.items.len(), "the bay drawn too");
        none_more(&read, &edited, &[]);
    }
}
