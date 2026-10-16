//! A model set's objects as a tree, for the extension's model view. Its
//! sections:
//! - **Geographical**: regions, their substations and lines, down through
//!   voltage levels, bays and equipment to terminals, limits and state
//!   variables; each object under the object the first of [`CONTAINED`] it
//!   writes names;
//! - **Operational**: control areas and load areas, with what names them
//!   ([`OPERATIONAL`]);
//! - **Not contained**: the rest of that containment, by class: what no
//!   container holds (base voltages, curves, diagrams, …) with what it holds;
//! - **By class**: every object, flat.
//!
//! A parent the set lacks (an IGM's region, which its boundary set names) is
//! an object of its own, labelled with its mRID. Each node counts the
//! findings of its subtree. The view asks for one node's children at a time,
//! by key; more than [`PAGE`] are grouped in ranges.

use cimmodel::base::FastMap;
use cimvalidation::Violation;
use serde_json::{Value, json};

use crate::grid::{Counts, Grid, Model};
use crate::validate::ModelSet;

/// The fields that name an object's container, the first written winning.
pub const CONTAINED: &[&str] = &[
    "SubGeographicalRegion.Region",
    "Substation.Region",
    "Line.Region",
    "DCLine.Region",
    "VoltageLevel.Substation",
    "Bay.VoltageLevel",
    "DCConverterUnit.Substation",
    "Equipment.EquipmentContainer",
    "ConnectivityNode.ConnectivityNodeContainer",
    "TopologicalNode.ConnectivityNodeContainer",
    "DCNode.DCEquipmentContainer",
    "DCTopologicalNode.DCEquipmentContainer",
    "BoundaryPoint.ConnectivityNode",
    "Terminal.ConductingEquipment",
    "DCTerminal.DCConductingEquipment",
    "ACDCConverterDCTerminal.DCConductingEquipment",
    "PowerTransformerEnd.PowerTransformer",
    "RatioTapChanger.TransformerEnd",
    "PhaseTapChanger.TransformerEnd",
    "RatioTapChangerTablePoint.RatioTapChangerTable",
    "PhaseTapChangerTablePoint.PhaseTapChangerTable",
    "OperationalLimitSet.Equipment",
    "OperationalLimitSet.Terminal",
    "OperationalLimit.OperationalLimitSet",
    "RegulatingControl.Terminal",
    "CurveData.Curve",
    "Measurement.PowerSystemResource",
    "AnalogValue.Analog",
    "DiscreteValue.Discrete",
    "AccumulatorValue.Accumulator",
    "StringMeasurementValue.StringMeasurement",
    "Location.PowerSystemResources",
    "PositionPoint.Location",
    "SvVoltage.TopologicalNode",
    "SvInjection.TopologicalNode",
    "SvPowerFlow.Terminal",
    "SvStatus.ConductingEquipment",
    "SvSwitch.Switch",
    "SvTapStep.TapChanger",
    "SvShuntCompensatorSections.ShuntCompensator",
];

/// What the Geographical section starts from: classes that contain, but
/// need not be contained in the set.
const CONTAINERS: &[&str] = &[
    "GeographicalRegion",
    "SubGeographicalRegion",
    "Substation",
    "Line",
    "DCLine",
    "VoltageLevel",
    "Bay",
    "DCConverterUnit",
    "EquivalentNetwork",
    "Plant",
];

/// The fields that name an object's area, under [`AREAS`].
const OPERATIONAL: &[&str] = &[
    "TieFlow.ControlArea",
    "ControlAreaGeneratingUnit.ControlArea",
    "SubLoadArea.LoadArea",
    "LoadGroup.SubLoadArea",
    "ConformLoad.LoadGroup",
    "NonConformLoad.LoadGroup",
];

const AREAS: &[&str] = &["ControlArea", "LoadArea"];

/// Children a node lists at once; more are grouped in ranges of this size.
const PAGE: usize = 500;

const NONE: u32 = u32::MAX;

/// The class of a group of parents the set lacks.
const MISSING: &str = "";

struct Object {
    id: String,
    /// `IdentifiedObject.name`, if written.
    label: Option<Box<str>>,
    /// Empty for a parent the set lacks.
    class: &'static str,
    findings: Counts,
    /// Its node in the containment and in the operational forest.
    contained: Option<u32>,
    operational: Option<u32>,
}

enum What {
    Section { key: &'static str, label: &'static str },
    /// Objects of a class: forest nodes in Not contained, leaf objects in By class.
    Group { section: &'static str, class: &'static str },
    /// An object in the containment (`c`) or operational (`o`) forest.
    Object { object: u32, forest: char },
}

struct Node {
    what: What,
    children: Vec<u32>,
    /// The node listing it; [`NONE`] for a section.
    parent: u32,
    findings: Counts,
}

/// A child as listed: a node, or a leaf object of By class.
#[derive(Clone, Copy)]
enum Entry {
    Node(u32),
    Leaf(u32),
}

#[derive(Default)]
pub struct Tree {
    objects: Vec<Object>,
    by_id: FastMap<String, u32>,
    nodes: Vec<Node>,
    sections: Vec<u32>,
    /// Sections and groups, by key.
    named: FastMap<String, u32>,
}

/// The tree of a model set's files.
pub fn build(m: &Model<'_>) -> Tree {
    let mut t = Tree::default();
    let mut ids: Vec<&str> = m.objects.keys().copied().collect();
    ids.sort_unstable();
    for &id in &ids {
        let label = m.text(id, "IdentifiedObject.name").filter(|n| !n.is_empty()).map(Box::from);
        t.object(id, label, m.class(id).unwrap_or(""));
    }

    let contained = forest(&mut t, m, &ids, CONTAINED, 'c');
    let mut roots: Vec<u32> = contained.roots.clone();
    let mut geo = Vec::new();
    let mut other: Vec<u32> = Vec::new();
    // A cycle has no root: start from its first object.
    for &id in &ids {
        let o = t.by_id[id];
        if t.objects[o as usize].contained.is_none() {
            roots.push(grow(&mut t, &contained.children, o, 'c'));
        }
    }
    let operational = forest(&mut t, m, &ids, OPERATIONAL, 'o');
    let areas = operational.roots;
    for n in roots {
        let class = t.class_of(n);
        let holds_container = || t.nodes[n as usize].children.iter().any(|&c| CONTAINERS.contains(&t.class_of(c)));
        if CONTAINERS.contains(&class) || (class == MISSING && holds_container()) {
            geo.push(n);
        } else {
            other.push(n);
        }
    }

    let geo = t.section("geo", "Geographical", geo);
    let op = t.section("op", "Operational", areas);
    let mut by_class: FastMap<&'static str, Vec<u32>> = FastMap::default();
    for n in other {
        by_class.entry(t.class_of(n)).or_default().push(n);
    }
    let mut groups: Vec<u32> = by_class.into_iter().map(|(class, ns)| t.group("other", class, ns)).collect();
    groups.sort_by_key(|&g| t.class_of(g));
    let other = t.section("other", "Not contained", groups);
    let mut by_class: FastMap<&'static str, Vec<u32>> = FastMap::default();
    for (o, obj) in t.objects.iter().enumerate() {
        if obj.class != MISSING {
            by_class.entry(obj.class).or_default().push(o as u32);
        }
    }
    let mut groups: Vec<u32> = by_class
        .into_iter()
        .map(|(class, mut os)| {
            os.sort_by_cached_key(|&o| t.label_of(o).to_lowercase());
            t.group("class", class, os)
        })
        .collect();
    groups.sort_by_key(|&g| t.class_of(g));
    let class = t.section("class", "By class", groups);
    t.sections = [geo, op, other, class].into_iter().filter(|&s| !t.nodes[s as usize].children.is_empty()).collect();
    t
}

/// A forest built from parent fields: its roots (in the operational forest,
/// the areas), and each object's children.
struct Forest {
    roots: Vec<u32>,
    children: FastMap<u32, Vec<u32>>,
}

/// The forest `keys` make, each object under the target of the first of them
/// it writes, its nodes added to `t`.
fn forest(t: &mut Tree, m: &Model<'_>, ids: &[&str], keys: &[&str], name: char) -> Forest {
    let rank: FastMap<&str, usize> = keys.iter().enumerate().map(|(i, &k)| (k, i)).collect();
    let mut parent: FastMap<u32, u32> = FastMap::default();
    for &id in ids {
        let Some(els) = m.objects.get(id) else { continue };
        let best = els.iter().flat_map(|el| el.fields().keys()).filter_map(|k| rank.get(k).map(|&r| (r, *k))).min();
        let Some(target) = best.and_then(|(_, k)| m.target(id, k)).filter(|p| *p != id) else { continue };
        let p = match t.by_id.get(target) {
            Some(&p) => p,
            None => t.object(target, None, MISSING),
        };
        parent.insert(t.by_id[id], p);
    }
    let mut children: FastMap<u32, Vec<u32>> = FastMap::default();
    for (&c, &p) in &parent {
        children.entry(p).or_default().push(c);
    }
    let mut roots = Vec::new();
    for o in 0..t.objects.len() as u32 {
        let is_root = !parent.contains_key(&o) && (name == 'c' || AREAS.contains(&t.objects[o as usize].class));
        if is_root {
            roots.push(grow(t, &children, o, name));
        }
    }
    Forest { roots, children }
}

/// Add object `o` and what it holds to forest `name`; returns its node.
fn grow(t: &mut Tree, children: &FastMap<u32, Vec<u32>>, o: u32, name: char) -> u32 {
    let root = t.node(o, name);
    let mut stack = vec![(root, o)];
    while let Some((n, o)) = stack.pop() {
        let mut kids = Vec::new();
        for &c in children.get(&o).map_or(&[][..], Vec::as_slice) {
            let slot = if name == 'c' { t.objects[c as usize].contained } else { t.objects[c as usize].operational };
            if slot.is_some() {
                continue;
            }
            let cn = t.node(c, name);
            kids.push(cn);
            stack.push((cn, c));
        }
        t.sort(&mut kids);
        for &k in &kids {
            t.nodes[k as usize].parent = n;
        }
        t.nodes[n as usize].children = kids;
    }
    root
}

impl Tree {
    fn object(&mut self, id: &str, label: Option<Box<str>>, class: &'static str) -> u32 {
        let o = self.objects.len() as u32;
        self.objects.push(Object { id: id.into(), label, class, findings: Counts::default(), contained: None, operational: None });
        self.by_id.insert(id.into(), o);
        o
    }

    fn node(&mut self, o: u32, forest: char) -> u32 {
        let n = self.nodes.len() as u32;
        self.nodes.push(Node { what: What::Object { object: o, forest }, children: Vec::new(), parent: NONE, findings: Counts::default() });
        let obj = &mut self.objects[o as usize];
        if forest == 'c' { obj.contained = Some(n) } else { obj.operational = Some(n) }
        n
    }

    fn section(&mut self, key: &'static str, label: &'static str, children: Vec<u32>) -> u32 {
        let n = self.nodes.len() as u32;
        for &c in &children {
            self.nodes[c as usize].parent = n;
        }
        self.nodes.push(Node { what: What::Section { key, label }, children, parent: NONE, findings: Counts::default() });
        self.named.insert(key.into(), n);
        n
    }

    fn group(&mut self, section: &'static str, class: &'static str, children: Vec<u32>) -> u32 {
        let n = self.nodes.len() as u32;
        if section != "class" {
            for &c in &children {
                self.nodes[c as usize].parent = n;
            }
        }
        self.nodes.push(Node { what: What::Group { section, class }, children, parent: NONE, findings: Counts::default() });
        self.named.insert(format!("{section}/{class}"), n);
        n
    }

    /// The key the view knows node `n` by.
    fn key_of(&self, n: u32) -> String {
        match self.nodes[n as usize].what {
            What::Section { key, .. } => key.into(),
            What::Group { section, class } => format!("{section}/{class}"),
            What::Object { object, forest } => format!("{forest}/{}", self.objects[object as usize].id),
        }
    }

    /// The objects whose name or mRID holds `query` (ignoring case), at most
    /// `limit`: a name equal to it first, then one starting with it, then one
    /// holding it, then an mRID holding it; by name within each.
    pub fn search(&self, query: &str, limit: usize) -> Vec<(&str, &str, &'static str)> {
        let q = query.trim().to_lowercase();
        if q.is_empty() {
            return Vec::new();
        }
        let mut hits: Vec<(u8, String, usize)> = self
            .objects
            .iter()
            .enumerate()
            .filter(|(_, o)| o.class != MISSING)
            .filter_map(|(i, o)| {
                let name = o.label.as_deref().map(str::to_lowercase).unwrap_or_default();
                let rank = if name == q {
                    0
                } else if name.starts_with(&q) {
                    1
                } else if name.contains(&q) {
                    2
                } else if o.id.to_lowercase().contains(&q) {
                    3
                } else {
                    return None;
                };
                Some((rank, name, i))
            })
            .collect();
        hits.sort();
        hits.into_iter()
            .take(limit)
            .map(|(_, _, i)| {
                let o = &self.objects[i];
                (o.id.as_str(), self.label_of(i as u32), o.class)
            })
            .collect()
    }

    /// Object `id`'s label and class.
    pub fn describe(&self, id: &str) -> Option<(&str, &'static str)> {
        let &o = self.by_id.get(id)?;
        Some((self.label_of(o), self.objects[o as usize].class))
    }

    /// What object `id` holds directly.
    pub fn children(&self, id: &str) -> Vec<&str> {
        let Some(n) = self.by_id.get(id).and_then(|&o| self.objects[o as usize].contained) else { return Vec::new() };
        self.nodes[n as usize]
            .children
            .iter()
            .filter_map(|&c| match self.nodes[c as usize].what {
                What::Object { object, .. } => Some(self.objects[object as usize].id.as_str()),
                _ => None,
            })
            .collect()
    }

    /// What object `id` holds, at any depth: its subtree in the containment.
    pub fn descendants(&self, id: &str) -> Vec<&str> {
        let Some(root) = self.by_id.get(id).and_then(|&o| self.objects[o as usize].contained) else { return Vec::new() };
        let mut out = Vec::new();
        let mut stack = self.nodes[root as usize].children.clone();
        while let Some(n) = stack.pop() {
            let node = &self.nodes[n as usize];
            if let What::Object { object, .. } = node.what {
                out.push(self.objects[object as usize].id.as_str());
            }
            stack.extend(&node.children);
        }
        out
    }

    /// Object `id`'s node in the containment and its containers' before it,
    /// outermost first, as node numbers; empty if it is not in the tree.
    pub fn chain(&self, id: &str) -> Vec<u32> {
        let Some(mut n) = self.by_id.get(id).and_then(|&o| self.objects[o as usize].contained) else { return Vec::new() };
        let mut out = vec![n];
        while let Some(What::Object { .. }) = self.nodes.get(self.nodes[n as usize].parent as usize).map(|p| &p.what) {
            n = self.nodes[n as usize].parent;
            out.push(n);
        }
        out.reverse();
        out
    }

    /// The labels of the containers of object `id`, outermost first; the
    /// regions left out below a substation or line.
    pub fn containers(&self, id: &str) -> Vec<&str> {
        let mut out = self.chain(id);
        out.pop();
        let region = |&n: &u32| matches!(self.class_of(n), "GeographicalRegion" | "SubGeographicalRegion");
        if out.iter().any(|n| !region(n)) {
            out.retain(|n| !region(n));
        }
        out.into_iter().map(|n| self.node_label(n)).collect()
    }

    /// The keys the view expands to reach object `id` in Geographical or Not
    /// contained, from its section on, the class groups and pages a long
    /// level is split into included; `None` if it is not listed (filtered out).
    pub fn path(&self, id: &str, only_findings: bool) -> Option<Vec<String>> {
        let mut chain = vec![self.objects[*self.by_id.get(id)? as usize].contained?];
        while let Some(&p) = chain.last().map(|&n| &self.nodes[n as usize].parent).filter(|&&p| p != NONE) {
            chain.push(p);
        }
        chain.reverse();
        let mut keys = vec![self.key_of(chain[0])];
        for pair in chain.windows(2) {
            let (parent, child) = (self.key_of(pair[0]), pair[1]);
            let entries = self.entries(&parent, only_findings)?;
            let at = entries.iter().position(|e| matches!(e, Entry::Node(c) if *c == child))?;
            // As `children_json` lists a long level.
            if entries.len() > PAGE {
                let class = self.class_of(child);
                if entries.iter().any(|&e| self.entry_class(e) != class) {
                    let within = entries.iter().take(at).filter(|&&e| self.entry_class(e) == class).count();
                    keys.push(format!("{parent}|{class}"));
                    if entries.iter().filter(|&&e| self.entry_class(e) == class).count() > PAGE {
                        keys.push(format!("{parent}|{class}~{}", within / PAGE * PAGE));
                    }
                } else {
                    keys.push(format!("{parent}~{}", at / PAGE * PAGE));
                }
            }
            keys.push(self.key_of(child));
        }
        Some(keys)
    }

    fn class_of(&self, n: u32) -> &'static str {
        match self.nodes[n as usize].what {
            What::Object { object, .. } => self.objects[object as usize].class,
            What::Group { class, .. } => class,
            What::Section { .. } => "",
        }
    }

    fn label_of(&self, o: u32) -> &str {
        let obj = &self.objects[o as usize];
        obj.label.as_deref().unwrap_or(&obj.id)
    }

    fn node_label(&self, n: u32) -> &str {
        match self.nodes[n as usize].what {
            What::Object { object, .. } => self.label_of(object),
            What::Group { class, .. } => class,
            What::Section { label, .. } => label,
        }
    }

    /// Containers first, then by class and label.
    fn sort(&self, nodes: &mut [u32]) {
        nodes.sort_by_cached_key(|&n| {
            let class = self.class_of(n);
            (!(class == MISSING || CONTAINERS.contains(&class)), class, self.node_label(n).to_lowercase())
        });
    }

    /// Count each finding on its object, and each node's subtree.
    pub fn add_findings(&mut self, violations: &[(Option<usize>, Violation)]) {
        for (_, v) in violations {
            if let Some(&o) = self.by_id.get(v.object_id.as_str()) {
                self.objects[o as usize].findings.add(&v.severity);
            }
        }
        // A forest node comes before its children; groups and sections after
        // what they hold, sections after groups.
        for pass in 0..3 {
            for n in (0..self.nodes.len()).rev() {
                let node = &self.nodes[n];
                let mut sum = match (&node.what, pass) {
                    (What::Object { object, .. }, 0) => self.objects[*object as usize].findings,
                    (What::Group { section: "class", .. }, 1) => {
                        let mut sum = Counts::default();
                        for &o in &node.children {
                            sum.merge(self.objects[o as usize].findings);
                        }
                        self.nodes[n].findings = sum;
                        continue;
                    }
                    (What::Group { .. }, 1) | (What::Section { .. }, 2) => Counts::default(),
                    _ => continue,
                };
                for &c in &node.children {
                    sum.merge(self.nodes[c as usize].findings);
                }
                self.nodes[n].findings = sum;
            }
        }
    }

    /// What `key` lists: its children; for `<key>|<class>`, those of a
    /// class; for a range key, `…~<start>`, a page of them. `None` for an
    /// unknown key.
    fn entries(&self, key: &str, only_findings: bool) -> Option<Vec<Entry>> {
        let (key, start) = match key.rsplit_once('~') {
            Some((key, start)) if start.parse::<usize>().is_ok() => (key, start.parse::<usize>().ok()),
            _ => (key, None),
        };
        let (base, class) = match key.split_once('|') {
            Some((base, class)) => (base, Some(class)),
            None => (key, None),
        };
        let n = match base.split_once('/') {
            Some(("c", id)) => self.objects[*self.by_id.get(id)? as usize].contained?,
            Some(("o", id)) => self.objects[*self.by_id.get(id)? as usize].operational?,
            _ => *self.named.get(base)?,
        };
        let node = &self.nodes[n as usize];
        let leaves = matches!(node.what, What::Group { section: "class", .. });
        let mut out: Vec<Entry> = node.children.iter().map(|&c| if leaves { Entry::Leaf(c) } else { Entry::Node(c) }).collect();
        if only_findings {
            out.retain(|&e| self.findings(e).total() > 0);
        }
        if let Some(class) = class {
            out.retain(|&e| self.entry_class(e) == class);
        }
        if let Some(start) = start {
            out = out.into_iter().skip(start).take(PAGE).collect();
        }
        Some(out)
    }

    fn entry_class(&self, e: Entry) -> &'static str {
        match e {
            Entry::Node(n) => self.class_of(n),
            Entry::Leaf(o) => self.objects[o as usize].class,
        }
    }

    fn findings(&self, e: Entry) -> Counts {
        match e {
            Entry::Node(n) => self.nodes[n as usize].findings,
            Entry::Leaf(o) => self.objects[o as usize].findings,
        }
    }

    /// The children of node `key`, or the sections for `None`, for the view:
    /// each with its key, label, class, number of children, findings, where
    /// it is written and where the grid view shows it. `None` for a key the
    /// tree does not have (any more).
    pub fn children_json(&self, set: &ModelSet, grid: &Grid, key: Option<&str>, only_findings: bool) -> Option<Value> {
        let entries = match key {
            Some(key) => self.entries(key, only_findings)?,
            None => {
                let mut sections: Vec<Entry> = self.sections.iter().map(|&s| Entry::Node(s)).collect();
                if only_findings {
                    sections.retain(|&e| self.findings(e).total() > 0);
                }
                sections
            }
        };
        // Too many to list: by class if they differ, else in pages.
        if let Some(key) = key.filter(|k| !k.contains('~')) && entries.len() > PAGE {
            let sum = |es: &[Entry]| {
                let mut findings = Counts::default();
                for &e in es {
                    findings.merge(self.findings(e));
                }
                findings.json()
            };
            let mut classes: Vec<(&str, Vec<Entry>)> = Vec::new();
            if !key.contains('|') {
                for &e in &entries {
                    let class = self.entry_class(e);
                    match classes.iter_mut().find(|c| c.0 == class) {
                        Some(c) => c.1.push(e),
                        None => classes.push((class, vec![e])),
                    }
                }
            }
            let ranges: Vec<Value> = if classes.len() > 1 {
                classes
                    .iter()
                    .map(|(class, es)| {
                        json!({
                            "key": format!("{key}|{class}"), "kind": "group", "class": class,
                            "label": if *class == MISSING { "Not in the set" } else { class }, "children": es.len(), "findings": sum(es),
                        })
                    })
                    .collect()
            } else {
                entries
                    .chunks(PAGE)
                    .enumerate()
                    .map(|(i, chunk)| {
                        json!({
                            "key": format!("{key}~{}", i * PAGE), "kind": "range",
                            "label": format!("{}–{}", i * PAGE + 1, i * PAGE + chunk.len()), "children": chunk.len(), "findings": sum(chunk),
                        })
                    })
                    .collect()
            };
            return Some(Value::Array(ranges));
        }
        let location = |id: &str| -> Value {
            match set.definition(id) {
                Some((file, el)) => json!({ "uri": file.url, "range": el.head }),
                None => Value::Null,
            }
        };
        // A line is shown as its segments are: the first of them on the grid.
        let reveal = |o: u32, kids: &[u32]| -> Option<Value> {
            grid.reveal(&self.objects[o as usize].id).or_else(|| {
                kids.iter().find_map(|&c| match self.nodes[c as usize].what {
                    What::Object { object, .. } => grid.reveal(&self.objects[object as usize].id).filter(|r| r.get("enter").is_none()),
                    _ => None,
                })
            })
        };
        let object = |o: u32, key: String, kids: &[u32], findings: Counts| -> Value {
            let obj = &self.objects[o as usize];
            json!({
                "key": key, "kind": if obj.class == MISSING { "missing" } else { "object" }, "id": obj.id,
                "label": self.label_of(o), "class": obj.class, "children": kids.len(), "findings": findings.json(),
                "location": location(&obj.id), "grid": reveal(o, kids),
            })
        };
        let items = entries
            .into_iter()
            .map(|e| match e {
                Entry::Leaf(o) => object(o, format!("k/{}", self.objects[o as usize].id), &[], self.objects[o as usize].findings),
                Entry::Node(n) => {
                    let node = &self.nodes[n as usize];
                    match node.what {
                        What::Object { object: o, forest } => {
                            object(o, format!("{forest}/{}", self.objects[o as usize].id), &node.children, node.findings)
                        }
                        What::Group { section, class } => json!({
                            "key": format!("{section}/{class}"), "kind": "group", "class": class,
                            "label": if class == MISSING { "Not in the set" } else { class },
                            "children": node.children.len(), "findings": node.findings.json(),
                        }),
                        What::Section { key, label } => json!({
                            "key": key, "kind": "section", "label": label, "children": node.children.len(), "findings": node.findings.json(),
                        }),
                    }
                }
            })
            .collect();
        Some(Value::Array(items))
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use serde_json::Value;

    use crate::validate::{Options, collect, validate};

    fn labels(v: &Value) -> Vec<&str> {
        v.as_array().unwrap().iter().map(|x| x["label"].as_str().unwrap()).collect()
    }

    /// MicroGrid: regions down to terminals, the boundary's lines among them;
    /// each object once in the containment, findings summed up the tree.
    #[test]
    fn micro_grid_tree() {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../CGMES-Test-Configurations/v3.0/MicroGrid/MicroGid-BaseCase/MicroGrid-BaseCase-Merged");
        let files = collect(&dir, &Default::default());
        assert_eq!(files.len(), 12, "missing test data — run `git submodule update --init`");
        let (set, _) = validate(&files, &Options { common: true, quality: true, silence: vec![] });
        let t = &set.tree;
        let children = |key: Option<&str>| t.children_json(&set, &set.grid, key, false).unwrap();
        assert_eq!(labels(&children(None)), ["Geographical", "Operational", "Not contained", "By class"]);

        let geo = children(Some("geo"));
        assert!(labels(&geo).contains(&"BE"), "{:?}", labels(&geo));
        let find = |list: &Value, label: &str| list.as_array().unwrap().iter().find(|x| x["label"] == label).cloned().unwrap();
        let be = find(&geo, "BE");
        let sub_regions = children(be["key"].as_str());
        let brussels_region = find(&sub_regions, "ELIA-Brussels");
        let subs = children(brussels_region["key"].as_str());
        let brussels = find(&subs, "PP_Brussels");
        assert_eq!(brussels["class"], "Substation");
        assert_eq!(brussels["grid"]["inside"], brussels["id"]);
        // Its voltage levels first, then the transformers it holds itself.
        let held = children(brussels["key"].as_str());
        let classes: Vec<&str> = held.as_array().unwrap().iter().map(|x| x["class"].as_str().unwrap()).collect();
        assert_eq!(&classes[..5], ["VoltageLevel"; 5], "{classes:?}");
        assert!(classes.contains(&"PowerTransformer"), "{classes:?}");
        assert_eq!(t.containers(brussels["id"].as_str().unwrap()), ["BE", "ELIA-Brussels"]);

        // Search: by name, ignoring case, the exact one first; by mRID.
        assert_eq!(t.search("anvers", 5)[0].1, "Anvers");
        // Names starting with it before names holding it; objects may share a name.
        let lines: Vec<&str> = t.search("be-line_", 30).into_iter().map(|h| h.1).collect();
        let starting = lines.iter().take_while(|l| l.starts_with("BE-Line_")).count();
        assert!(starting >= 7 && lines[starting..].iter().all(|l| l.contains("BE-Line_") && !l.starts_with("BE-Line_")), "{lines:?}");
        let id = brussels["id"].as_str().unwrap();
        assert_eq!(t.search(&id[3..15], 5)[0].0, id);
        assert!(t.search("  ", 5).is_empty());
        assert_eq!(t.containers(held[0]["id"].as_str().unwrap()), ["PP_Brussels"]);

        // Each object's path leads to it, a listed level at a time.
        for group in children(Some("class")).as_array().unwrap() {
            for x in children(group["key"].as_str()).as_array().unwrap() {
                let keys = t.path(x["id"].as_str().unwrap(), false).unwrap();
                assert!(["geo", "other"].contains(&keys[0].as_str()), "{keys:?}");
                for step in keys.windows(2) {
                    let listed = children(Some(&step[0]));
                    assert!(listed.as_array().unwrap().iter().any(|y| y["key"] == step[1].as_str()), "{keys:?}");
                }
                assert_eq!(keys.last().unwrap(), &format!("c/{}", x["id"].as_str().unwrap()));
            }
        }

        // Each object of the set is once in Geographical or Not contained.
        let mut seen = 0;
        let mut stack: Vec<String> = vec!["geo".into(), "other".into()];
        while let Some(key) = stack.pop() {
            for x in children(Some(&key)).as_array().unwrap() {
                if x["kind"] == "object" {
                    seen += 1;
                }
                if x["children"].as_u64().unwrap() > 0 {
                    stack.push(x["key"].as_str().unwrap().into());
                }
            }
        }
        let by_class: u64 = children(Some("class")).as_array().unwrap().iter().map(|x| x["children"].as_u64().unwrap()).sum();
        assert_eq!(seen, by_class);

        // A section counts the findings of what it holds; By class, of every
        // object, so all the findings on objects of the set.
        let total = |x: &Value| ["violation", "warning", "info"].iter().map(|s| x["findings"][s].as_u64().unwrap()).sum::<u64>();
        let sections = children(None);
        let sum_of = |key: &str| children(Some(key)).as_array().unwrap().iter().map(total).sum::<u64>();
        for s in sections.as_array().unwrap() {
            assert_eq!(total(s), sum_of(s["key"].as_str().unwrap()), "{}", s["label"]);
        }
        let all = total(&find(&sections, "By class"));
        assert!(all > 0);
        assert_eq!(total(&find(&sections, "Geographical")) + total(&find(&sections, "Not contained")), all);
        assert!(t.children_json(&set, &set.grid, None, true).unwrap().as_array().unwrap().iter().all(|x| total(x) > 0));
    }
}
