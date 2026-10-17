//! A model set's grid at substation level, for the extension's grid view:
//! substations (and boundary points no substation holds) as nodes, the
//! equipment joining two of them (lines, mostly) as edges, and each
//! object's findings counted on the node or edge it belongs to. Where the
//! set has a Diagram Layout or Geographical Location profile, each diagram
//! and coordinate system is a [`Layout`]: node positions and line routes.
//! What each substation holds is its [`Inside`], for the view's drill-down.
//!
//! Built from the set's files before validation consumes them; the files are
//! read as one model (a field from whichever file writes it), without
//! merging them.

use std::collections::BTreeMap;

use cimmodel::base::FastMap;
use cimmodel::{CimDataset, Element};
use cimvalidation::Violation;
use serde_json::{Value, json};

use crate::index::mrid_of;
use crate::validate::ModelSet;

/// Where an object belongs in the view.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Place {
    Node(usize),
    Edge(usize),
}

#[derive(Debug, Default)]
pub struct Node {
    pub id: String,
    pub label: String,
    /// `Substation`, or `ConnectivityNode`/`TopologicalNode` for a boundary point.
    pub kind: &'static str,
    pub region: Option<String>,
    pub kv: Vec<f64>,
    /// Equipment the node contains, by class.
    pub equipment: BTreeMap<&'static str, usize>,
    pub findings: Counts,
}

#[derive(Debug, Default)]
pub struct Edge {
    pub id: String,
    pub label: String,
    pub class: &'static str,
    pub source: usize,
    pub target: usize,
    pub kv: Option<f64>,
    pub findings: Counts,
}

/// Findings by severity.
#[derive(Debug, Default, Clone, Copy)]
pub struct Counts {
    pub violation: usize,
    pub warning: usize,
    pub info: usize,
}

impl Counts {
    pub(crate) fn add(&mut self, severity: &str) {
        match severity {
            "sh:Violation" => self.violation += 1,
            "sh:Warning" => self.warning += 1,
            _ => self.info += 1,
        }
    }

    pub(crate) fn merge(&mut self, other: Counts) {
        self.violation += other.violation;
        self.warning += other.warning;
        self.info += other.info;
    }

    pub(crate) fn total(&self) -> usize {
        self.violation + self.warning + self.info
    }

    pub(crate) fn json(&self) -> Value {
        json!({ "violation": self.violation, "warning": self.warning, "info": self.info })
    }
}

#[derive(Debug, Default)]
pub struct Grid {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    /// Each placed object, by mRID.
    placed: FastMap<String, Place>,
    /// Findings on objects with no node or edge (headers, regions, …).
    pub elsewhere: Counts,
    /// Positions from the DL diagrams and GL coordinate systems.
    pub layouts: Vec<Layout>,
    /// Positions laid out here ([`crate::layout::force`]), for the view's
    /// automatic layout: the view's own is quadratic, and this one is the
    /// same each time.
    pub auto: Option<Layout>,
    /// What each substation holds, by node index.
    pub inside: FastMap<usize, Inside>,
    /// Each object on a substation's node that one of its items stands for:
    /// (node, item).
    within: FastMap<String, (usize, usize)>,
}

/// Inside one substation: its equipment and the connectivity nodes (else
/// topological nodes) their terminals meet at, by voltage level; equipment
/// reaching other nodes (lines, mostly) as an item with its far ends.
#[derive(Debug, Default)]
pub struct Inside {
    pub groups: Vec<Group>,
    pub items: Vec<Item>,
    pub links: Vec<Link>,
    /// Findings on the substation's node that no item stands for: the
    /// substation, its voltage levels and bays, …
    pub findings: Counts,
    /// Positions from each DL diagram that draws its items, by item index.
    pub layouts: Vec<Layout>,
}

/// A voltage level.
#[derive(Debug, Default)]
pub struct Group {
    pub id: String,
    pub label: String,
    pub kv: Option<f64>,
}

#[derive(Debug, Default)]
pub struct Item {
    pub id: String,
    pub label: String,
    pub class: &'static str,
    /// A connectivity or topological node, which the item's terminals meet at.
    pub junction: bool,
    /// Its voltage level, by index into `groups`.
    pub group: Option<usize>,
    /// A switch's state: `Switch.open`, else `Switch.normalOpen`.
    pub open: Option<bool>,
    /// `Equipment.inService`.
    pub in_service: Option<bool>,
    /// The other nodes the equipment reaches.
    pub far: Vec<usize>,
    pub findings: Counts,
}

/// A terminal: equipment item → junction item.
#[derive(Debug)]
pub struct Link {
    pub id: String,
    pub item: usize,
    pub junction: usize,
    /// `ACDCTerminal.connected`.
    pub connected: Option<bool>,
}

/// Positions of nodes and routes of edges, in one diagram (DL) or one
/// coordinate system (GL), as the data writes them.
#[derive(Debug, Default)]
pub struct Layout {
    pub id: String,
    pub name: String,
    /// `diagram` (x right, y as `flip_y` says) or `geographic` (x longitude,
    /// y latitude, in degrees).
    pub kind: &'static str,
    /// A diagram whose y grows upwards (`OrientationKind.positive`).
    pub flip_y: bool,
    /// Node index → position.
    pub nodes: BTreeMap<usize, (f64, f64)>,
    /// Edge index → its route, from its source's end to its target's.
    pub edges: BTreeMap<usize, Vec<(f64, f64)>>,
}

/// Points gathered for a layout: a node's own, and those it takes from the
/// ends of its lines when it has none of its own.
#[derive(Default)]
struct Gather {
    own: FastMap<usize, Vec<(f64, f64)>>,
    ends: FastMap<usize, Vec<(f64, f64)>>,
    routes: BTreeMap<usize, Vec<(f64, f64)>>,
}

impl Gather {
    /// The points of one located object: an edge's route (whose ends also
    /// locate its nodes), else points on the node it is in.
    fn add(&mut self, grid: &Grid, object: &str, points: Vec<(f64, f64)>) {
        if points.is_empty() {
            return;
        }
        match grid.placed.get(object) {
            Some(&Place::Edge(e)) if grid.edges[e].id == object => {
                let edge = &grid.edges[e];
                self.ends.entry(edge.source).or_default().push(points[0]);
                self.ends.entry(edge.target).or_default().push(points[points.len() - 1]);
                self.routes.insert(e, points);
            }
            Some(&Place::Node(n)) => self.own.entry(n).or_default().extend(points),
            _ => {}
        }
    }

    fn layout(self, id: String, name: String, kind: &'static str, flip_y: bool) -> Option<Layout> {
        let mean = |ps: &[(f64, f64)]| {
            let n = ps.len() as f64;
            (ps.iter().map(|p| p.0).sum::<f64>() / n, ps.iter().map(|p| p.1).sum::<f64>() / n)
        };
        let mut nodes: BTreeMap<usize, (f64, f64)> = self.ends.iter().map(|(&n, ps)| (n, mean(ps))).collect();
        nodes.extend(self.own.iter().map(|(&n, ps)| (n, mean(ps))));
        (!nodes.is_empty()).then_some(Layout { id, name, kind, flip_y, nodes, edges: self.routes })
    }
}

/// Each object's points by class, `(owner, sequence, x, y)`, sorted, and the
/// owner of each point list: a DiagramObject's or a Location's.
fn points<'a>(m: &Model<'a>, class: &str, owner_key: &str, x_key: &str, y_key: &str, seq_key: &str) -> FastMap<&'a str, Vec<(f64, f64)>> {
    let mut by_owner: FastMap<&str, Vec<(i64, &str, f64, f64)>> = FastMap::default();
    for (&id, els) in &m.objects {
        if els[0].type_name() != class {
            continue;
        }
        if let (Some(owner), Some(x), Some(y)) = (m.reference(id, owner_key), m.number(id, x_key), m.number(id, y_key)) {
            let seq = m.number(id, seq_key).map_or(i64::MAX, |n| n as i64);
            by_owner.entry(owner).or_default().push((seq, id, x, y));
        }
    }
    by_owner
        .into_iter()
        .map(|(owner, mut ps)| {
            ps.sort_by(|a, b| (a.0, a.1).cmp(&(b.0, b.1)));
            (owner, ps.into_iter().map(|(_, _, x, y)| (x, y)).collect())
        })
        .collect()
}

/// Whether a diagram's y grows upwards, and its layout's name.
fn diagram_flip(m: &Model, diagram: &str) -> (bool, String) {
    let flip_y = m.objects.get(diagram).and_then(|els| els.iter().find_map(|el| el.get_ref("Diagram.orientation"))).is_some_and(|o| o.ends_with("positive"));
    (flip_y, format!("Diagram: {}", m.name(diagram)))
}

/// One layout per DL diagram and per GL coordinate system; and one per DL
/// diagram for each substation whose items it draws, by node index.
fn layouts(m: &Model, grid: &Grid) -> (Vec<Layout>, Vec<(usize, Layout)>) {
    let mut out = Vec::new();

    // DL: each DiagramObject's points, on the object it shows.
    let diagram_points = points(m, "DiagramObjectPoint", "DiagramObjectPoint.DiagramObject",
        "DiagramObjectPoint.xPosition", "DiagramObjectPoint.yPosition", "DiagramObjectPoint.sequenceNumber");
    let mut diagrams: BTreeMap<&str, Gather> = BTreeMap::new();
    // (diagram, node) → item → its points.
    type Items = BTreeMap<usize, Vec<(f64, f64)>>;
    let mut insides: BTreeMap<(&str, usize), Items> = BTreeMap::new();
    let mut objects: Vec<&&str> = diagram_points.keys().collect();
    objects.sort();
    for &object in objects {
        let (Some(diagram), Some(shown)) = (m.reference(object, "DiagramObject.Diagram"), m.reference(object, "DiagramObject.IdentifiedObject")) else { continue };
        diagrams.entry(diagram).or_default().add(grid, shown, diagram_points[object].clone());
        // An item, in each substation it is drawn in (a line in both of its ends').
        let ns: Vec<usize> = match grid.placed.get(shown) {
            Some(&Place::Edge(e)) => vec![grid.edges[e].source, grid.edges[e].target],
            Some(&Place::Node(n)) => vec![n],
            None => vec![],
        };
        for n in ns {
            if let Some(i) = grid.inside.get(&n).and_then(|inside| inside.items.iter().position(|it| it.id == shown)) {
                insides.entry((diagram, n)).or_default().entry(i).or_default().extend(&diagram_points[object]);
            }
        }
    }
    for (diagram, gather) in diagrams {
        let (flip_y, name) = diagram_flip(m, diagram);
        out.extend(gather.layout(format!("dl:{diagram}"), name, "diagram", flip_y));
    }
    let mut inside = Vec::new();
    for ((diagram, n), placed) in insides {
        if placed.len() < 2 {
            continue;
        }
        let items = &grid.inside[&n].items;
        let mean = |ps: &[(f64, f64)]| {
            let k = ps.len() as f64;
            (ps.iter().map(|p| p.0).sum::<f64>() / k, ps.iter().map(|p| p.1).sum::<f64>() / k)
        };
        let own: Vec<(f64, f64)> = placed.iter().filter(|(i, _)| items[**i].far.is_empty()).map(|(_, ps)| mean(ps)).collect();
        let centre = (!own.is_empty()).then(|| mean(&own));
        let span = own.iter().flat_map(|a| own.iter().map(move |b| (a.0 - b.0).abs().max((a.1 - b.1).abs()))).fold(0.0, f64::max);
        let nodes = placed
            .iter()
            .map(|(&i, ps)| {
                // A line drawn through the substation is at its end here, a
                // little out along the line, off the node it ends at.
                let at = match centre {
                    Some(c) if !items[i].far.is_empty() && ps.len() > 1 => {
                        let d = |p: (f64, f64)| (p.0 - c.0).hypot(p.1 - c.1);
                        let (end, next) = if d(ps[0]) <= d(ps[ps.len() - 1]) { (ps[0], ps[1]) } else { (ps[ps.len() - 1], ps[ps.len() - 2]) };
                        let k = (next.0 - end.0).hypot(next.1 - end.1);
                        if k > 0.0 && span > 0.0 {
                            let step = (0.06 * span).min(k / 2.0);
                            (end.0 + (next.0 - end.0) / k * step, end.1 + (next.1 - end.1) / k * step)
                        } else {
                            end
                        }
                    }
                    _ => mean(ps),
                };
                (i, at)
            })
            .collect();
        let (flip_y, name) = diagram_flip(m, diagram);
        inside.push((n, Layout { id: format!("dl:{diagram}"), name, kind: "diagram", flip_y, nodes, edges: BTreeMap::new() }));
    }

    // GL: each Location's points, on the resources it locates, by coordinate system.
    let location_points = points(m, "PositionPoint", "PositionPoint.Location",
        "PositionPoint.xPosition", "PositionPoint.yPosition", "PositionPoint.sequenceNumber");
    let mut systems: BTreeMap<String, Gather> = BTreeMap::new();
    let mut locations: Vec<&&str> = location_points.keys().collect();
    locations.sort();
    for &location in locations {
        let crs = m
            .reference(location, "Location.CoordinateSystem")
            .and_then(|c| m.text(c, "CoordinateSystem.crsUrn"))
            .unwrap_or("urn:ogc:def:crs:EPSG::4326")
            .to_string();
        let gather = systems.entry(crs).or_default();
        let located = m.objects.get(location).map(|els| els.iter().flat_map(|el| el.get_refs("Location.PowerSystemResources")).cloned().collect::<Vec<_>>()).unwrap_or_default();
        for psr in located {
            gather.add(grid, mrid_of(&psr), location_points[location].clone());
        }
    }
    for (crs, gather) in systems {
        let short = crs.rsplit(':').next().unwrap_or(&crs).to_string();
        out.extend(gather.layout(format!("gl:{crs}"), format!("Geographic (EPSG:{short})"), "geographic", false));
    }
    (out, inside)
}

/// The set's files read as one model: each object's elements, the defining
/// one first.
pub(crate) struct Model<'a> {
    pub(crate) objects: FastMap<&'a str, Vec<&'a Element>>,
    /// ConductingEquipment → its terminals, by sequence number.
    terminals: FastMap<&'a str, Vec<&'a str>>,
}

impl<'a> Model<'a> {
    pub(crate) fn new(datasets: &'a [CimDataset]) -> Self {
        let mut objects: FastMap<&str, Vec<&Element>> = FastMap::default();
        for ds in datasets {
            for (mrid, el) in &ds.entries {
                let els = objects.entry(mrid.as_str()).or_default();
                if el.defined() { els.insert(0, el) } else { els.push(el) }
            }
        }
        let mut model = Self { objects, terminals: FastMap::default() };
        let mut terminals: FastMap<&str, Vec<(i64, &str)>> = FastMap::default();
        for (&mrid, els) in &model.objects {
            if els[0].type_name() != "Terminal" {
                continue;
            }
            if let Some(ce) = model.reference(mrid, "Terminal.ConductingEquipment") {
                let seq = model.number(mrid, "ACDCTerminal.sequenceNumber").map_or(i64::MAX, |n| n as i64);
                terminals.entry(ce).or_default().push((seq, mrid));
            }
        }
        model.terminals = terminals
            .into_iter()
            .map(|(ce, mut ts)| {
                ts.sort();
                (ce, ts.into_iter().map(|(_, t)| t).collect())
            })
            .collect();
        model
    }

    pub(crate) fn class(&self, mrid: &str) -> Option<&'static str> {
        self.objects.get(mrid).map(|els| els[0].type_name())
    }

    pub(crate) fn text(&self, mrid: &str, key: &str) -> Option<&'a str> {
        self.objects.get(mrid)?.iter().find_map(|el| el.get_str(key))
    }

    fn number(&self, mrid: &str, key: &str) -> Option<f64> {
        self.text(mrid, key)?.trim().parse().ok()
    }

    fn flag(&self, mrid: &str, key: &str) -> Option<bool> {
        self.text(mrid, key).map(|t| t.trim() == "true")
    }

    /// The voltage level a container (VoltageLevel, Bay) is, or is in.
    fn voltage_level(&self, container: &'a str) -> Option<&'a str> {
        match self.class(container)? {
            "VoltageLevel" => Some(container),
            "Bay" => self.reference(container, "Bay.VoltageLevel"),
            _ => None,
        }
    }

    /// A terminal's connectivity node, else its topological node.
    fn terminal_node(&self, terminal: &str) -> Option<&'a str> {
        self.reference(terminal, "Terminal.ConnectivityNode").or_else(|| self.reference(terminal, "Terminal.TopologicalNode"))
    }

    /// The mRID a field refers to, whether or not the set has that object.
    pub(crate) fn target(&self, mrid: &str, key: &str) -> Option<&'a str> {
        self.objects.get(mrid)?.iter().find_map(|el| el.get_ref(key)).map(mrid_of)
    }

    /// The object a field refers to, if it is in the set.
    pub(crate) fn reference(&self, mrid: &str, key: &str) -> Option<&'a str> {
        let target = self.objects.get(mrid)?.iter().find_map(|el| el.get_ref(key))?;
        self.objects.get_key_value(mrid_of(target)).map(|(k, _)| *k)
    }

    fn name(&self, mrid: &str) -> String {
        self.text(mrid, "IdentifiedObject.name").filter(|n| !n.is_empty()).unwrap_or(mrid).to_string()
    }

    /// The substation a container (VoltageLevel, Bay, Substation) is in.
    fn substation(&self, container: &'a str) -> Option<&'a str> {
        let mut at = container;
        for _ in 0..4 {
            at = match self.class(at)? {
                "Substation" => return Some(at),
                "VoltageLevel" => self.reference(at, "VoltageLevel.Substation")?,
                "Bay" => self.reference(at, "Bay.VoltageLevel")?,
                _ => return None,
            };
        }
        None
    }

    /// The node of a terminal: its connectivity (else topological) node's
    /// substation, else that node itself, a boundary point.
    fn terminal_end(&self, terminal: &'a str) -> Option<(&'a str, bool)> {
        let node = self.terminal_node(terminal)?;
        match self.node_container(node).and_then(|c| self.substation(c)) {
            Some(sub) => Some((sub, false)),
            None => Some((node, true)),
        }
    }

    fn node_container(&self, node: &str) -> Option<&'a str> {
        self.reference(node, "ConnectivityNode.ConnectivityNodeContainer")
            .or_else(|| self.reference(node, "TopologicalNode.ConnectivityNodeContainer"))
    }
}

/// The grid of a model set's files.
pub fn build(m: &Model<'_>) -> Grid {
    let mut grid = Grid::default();
    let mut node_of: FastMap<&str, usize> = FastMap::default();

    let mut subs: Vec<&str> = m.objects.keys().copied().filter(|k| m.class(k) == Some("Substation")).collect();
    subs.sort_by_key(|s| (m.name(s), *s));
    for sub in subs {
        node_of.insert(sub, grid.nodes.len());
        let region = m.reference(sub, "Substation.Region").map(|r| m.name(r));
        grid.nodes.push(Node { id: sub.into(), label: m.name(sub), kind: "Substation", region, ..Default::default() });
    }

    // Equipment whose terminals reach two different nodes is an edge. Inside
    // a substation, each terminal ending there links its equipment to its node.
    let mut items: FastMap<(usize, &str), usize> = FastMap::default();
    let mut groups: FastMap<(usize, &str), usize> = FastMap::default();
    let mut equipment: Vec<&str> = m.terminals.keys().copied().collect();
    equipment.sort();
    for ce in equipment {
        let terminals = &m.terminals[ce];
        let ends: Vec<Option<(&str, bool)>> = terminals.iter().map(|t| m.terminal_end(t)).collect();
        let mut distinct: Vec<(&str, bool)> = Vec::new();
        for &end in ends.iter().flatten() {
            if !distinct.iter().any(|e| e.0 == end.0) {
                distinct.push(end);
            }
        }
        if distinct.len() >= 2 {
            let source = node_for(m, &mut grid, &mut node_of, distinct[0]);
            for &end in &distinct[1..] {
                let target = node_for(m, &mut grid, &mut node_of, end);
                let kv = m.reference(ce, "ConductingEquipment.BaseVoltage").and_then(|b| m.number(b, "BaseVoltage.nominalVoltage"));
                let id = if distinct.len() == 2 { ce.to_string() } else { format!("{ce}#{}", grid.edges.len()) };
                grid.placed.insert(ce.to_string(), Place::Edge(grid.edges.len()));
                grid.edges.push(Edge { id, label: m.name(ce), class: m.class(ce).unwrap_or(""), source, target, kv, findings: Counts::default() });
            }
        }
        let reached: Vec<usize> = distinct.iter().filter_map(|e| node_of.get(e.0).copied()).collect();
        for (t, end) in terminals.iter().zip(&ends) {
            let (Some(&n), Some(node)) = (end.and_then(|e| node_of.get(e.0)), m.terminal_node(t)) else { continue };
            if grid.nodes[n].kind != "Substation" {
                continue;
            }
            let far: Vec<usize> = reached.iter().copied().filter(|&r| r != n).collect();
            let item = item_for(m, &mut grid, &mut items, &mut groups, n, ce, false, far);
            let junction = item_for(m, &mut grid, &mut items, &mut groups, n, node, true, vec![]);
            grid.inside.entry(n).or_default().links.push(Link { id: t.to_string(), item, junction, connected: m.flag(t, "ACDCTerminal.connected") });
        }
    }

    // Every other object goes where its container, equipment or node is.
    let mut ids: Vec<&str> = m.objects.keys().copied().collect();
    ids.sort();
    let mut within = FastMap::default();
    for id in ids {
        if let Some(Place::Node(n)) = place(m, &node_of, &items, &mut grid.placed, &mut within, id, 0)
            && let Some(container) = m.reference(id, "Equipment.EquipmentContainer")
            && m.substation(container).is_some()
        {
            *grid.nodes[n].equipment.entry(m.class(id).unwrap_or("")).or_default() += 1;
        }
    }
    grid.within = within;
    for (&vl, els) in &m.objects {
        if els[0].type_name() != "VoltageLevel" {
            continue;
        }
        if let Some(Place::Node(n)) = grid.placed.get(vl)
            && let Some(kv) = m.reference(vl, "VoltageLevel.BaseVoltage").and_then(|b| m.number(b, "BaseVoltage.nominalVoltage"))
        {
            grid.nodes[*n].kv.push(kv);
        }
    }
    for node in &mut grid.nodes {
        node.kv.sort_by(|a, b| b.total_cmp(a));
        node.kv.dedup();
    }

    let (layouts, inside) = layouts(m, &grid);
    grid.layouts = layouts;
    if !grid.nodes.is_empty() {
        let edges: Vec<(usize, usize)> = grid.edges.iter().map(|e| (e.source, e.target)).collect();
        let pos = crate::layout::force(grid.nodes.len(), &edges);
        grid.auto = Some(Layout {
            id: "auto".into(), name: "Automatic".into(), kind: "diagram",
            nodes: pos.into_iter().enumerate().collect(), ..Default::default()
        });
    }
    for (n, l) in inside {
        grid.inside.get_mut(&n).unwrap().layouts.push(l);
    }
    grid
}

/// The item for `id` inside node `n`'s substation, added on first use, with
/// its voltage level.
#[allow(clippy::too_many_arguments)]
fn item_for<'a>(
    m: &Model<'a>, grid: &mut Grid, items: &mut FastMap<(usize, &'a str), usize>, groups: &mut FastMap<(usize, &'a str), usize>,
    n: usize, id: &'a str, junction: bool, far: Vec<usize>,
) -> usize {
    if let Some(&i) = items.get(&(n, id)) {
        return i;
    }
    let container = if junction { m.node_container(id) } else { m.reference(id, "Equipment.EquipmentContainer") };
    let vl = container.and_then(|c| m.voltage_level(c)).filter(|vl| m.substation(vl) == Some(grid.nodes[n].id.as_str()));
    let inside = grid.inside.entry(n).or_default();
    let group = vl.map(|vl| {
        *groups.entry((n, vl)).or_insert_with(|| {
            let kv = m.reference(vl, "VoltageLevel.BaseVoltage").and_then(|b| m.number(b, "BaseVoltage.nominalVoltage"));
            inside.groups.push(Group { id: vl.into(), label: m.name(vl), kv });
            inside.groups.len() - 1
        })
    });
    let class = m.class(id).unwrap_or("");
    let switch = !junction && (m.text(id, "Switch.open").is_some() || m.text(id, "Switch.normalOpen").is_some());
    inside.items.push(Item {
        id: id.into(),
        label: m.name(id),
        class,
        junction,
        group,
        open: if switch { m.flag(id, "Switch.open").or_else(|| m.flag(id, "Switch.normalOpen")) } else { None },
        in_service: if junction { None } else { m.flag(id, "Equipment.inService") },
        far,
        findings: Counts::default(),
    });
    items.insert((n, id), inside.items.len() - 1);
    inside.items.len() - 1
}

/// The node of an edge's end, added on first use: a substation, or a
/// boundary point.
fn node_for<'a>(m: &Model<'a>, grid: &mut Grid, node_of: &mut FastMap<&'a str, usize>, (id, boundary): (&'a str, bool)) -> usize {
    *node_of.entry(id).or_insert_with(|| {
        grid.nodes.push(Node {
            id: id.into(),
            // An IGM often only refers to a boundary point the boundary set names.
            label: m.text(id, "IdentifiedObject.name").unwrap_or_default().to_string(),
            kind: if boundary { m.class(id).unwrap_or("ConnectivityNode") } else { "Substation" },
            ..Default::default()
        });
        grid.nodes.len() - 1
    })
}

/// Where `id` belongs, memoised in `placed`: a container's substation, a
/// terminal's equipment, a node's substation, else the first referenced
/// object that has a place. On a substation's node, also the item `id`
/// stands for there, memoised in `within`: an item itself, else that of the
/// object it takes its place from.
#[allow(clippy::too_many_arguments)]
fn place<'a>(
    m: &Model<'a>, node_of: &FastMap<&str, usize>, items: &FastMap<(usize, &'a str), usize>, placed: &mut FastMap<String, Place>,
    within: &mut FastMap<String, (usize, usize)>, id: &'a str, depth: u8,
) -> Option<Place> {
    if let Some(p) = placed.get(id) {
        return Some(*p);
    }
    if depth > 3 {
        return None;
    }
    let class = m.class(id)?;
    let mut via = None;
    let direct = match class {
        "Substation" | "VoltageLevel" | "Bay" => m.substation(id).and_then(|s| node_of.get(s)).map(|&n| Place::Node(n)),
        "ConnectivityNode" | "TopologicalNode" => match node_of.get(id) {
            Some(&n) => Some(Place::Node(n)),
            None => {
                let key = if class == "ConnectivityNode" { "ConnectivityNode.ConnectivityNodeContainer" } else { "TopologicalNode.ConnectivityNodeContainer" };
                m.reference(id, key).and_then(|c| m.substation(c)).and_then(|s| node_of.get(s)).map(|&n| Place::Node(n))
            }
        },
        "Terminal" => m.reference(id, "Terminal.ConductingEquipment").and_then(|ce| {
            via = Some(ce);
            place(m, node_of, items, placed, within, ce, depth + 1)
        }),
        _ => m.reference(id, "Equipment.EquipmentContainer").and_then(|c| m.substation(c)).and_then(|s| node_of.get(s)).map(|&n| Place::Node(n)),
    };
    let found = match direct {
        Some(p) => p,
        None => {
            // Limits, tap changers, regulating controls, state variables, …:
            // where what they refer to is.
            let els = m.objects.get(id)?;
            let mut keys: Vec<&str> = els.iter().flat_map(|el| el.fields().keys().copied()).collect();
            keys.sort();
            keys.dedup();
            let (r, p) = keys.into_iter().filter(|k| !k.ends_with(".BaseVoltage") && !k.ends_with(".Region")).find_map(|k| {
                let r = m.reference(id, k).filter(|r| *r != id)?;
                place(m, node_of, items, placed, within, r, depth + 1).map(|p| (r, p))
            })?;
            via = Some(r);
            p
        }
    };
    placed.insert(id.to_string(), found);
    if let Place::Node(n) = found
        && let Some(w) = items.get(&(n, id)).map(|&i| (n, i)).or_else(|| via.and_then(|r| within.get(r).copied()).filter(|w| w.0 == n))
    {
        within.insert(id.to_string(), w);
    }
    Some(found)
}

impl Grid {
    /// Count each finding on its object's node or edge.
    pub fn add_findings(&mut self, violations: &[(Option<usize>, Violation)]) {
        for (_, v) in violations {
            match self.placed.get(v.object_id.as_str()) {
                Some(Place::Node(n)) => {
                    self.nodes[*n].findings.add(&v.severity);
                    if let Some(inside) = self.inside.get_mut(n) {
                        match self.within.get(v.object_id.as_str()) {
                            Some(&(_, i)) => inside.items[i].findings.add(&v.severity),
                            None => inside.findings.add(&v.severity),
                        }
                    }
                }
                Some(Place::Edge(e)) => self.edges[*e].findings.add(&v.severity),
                None => self.elsewhere.add(&v.severity),
            }
        }
    }

    /// Where the view shows object `id`: `select`, the node or edge it is
    /// on, else the item it stands for inside `enter`, its substation; and
    /// `inside`, a substation's own id. `None` if it is on neither.
    pub fn reveal(&self, id: &str) -> Option<Value> {
        match *self.placed.get(id)? {
            Place::Edge(e) => Some(json!({ "select": format!("e:{}", self.edges[e].id) })),
            Place::Node(n) => {
                let node = &self.nodes[n];
                if node.kind != "Substation" || node.id == id {
                    let inside = (node.kind == "Substation").then_some(&node.id);
                    return Some(json!({ "select": node.id, "inside": inside }));
                }
                let item = self.within.get(id).map(|&(_, i)| &self.inside[&n].items[i].id);
                Some(json!({ "enter": node.id, "select": item }))
            }
        }
    }

    /// The view's data: nodes and edges, each with where it is written.
    pub fn json(&self, set: &ModelSet) -> Value {
        let location = |id: &str| -> Value {
            let id = id.split('#').next().unwrap_or(id);
            match set.definition(id) {
                Some((file, el)) => json!({ "uri": file.url, "range": el.head }),
                None => Value::Null,
            }
        };
        let nodes: Vec<Value> = self
            .nodes
            .iter()
            .map(|n| {
                json!({
                    "id": n.id, "label": n.label, "kind": n.kind, "region": n.region, "kV": n.kv,
                    "equipment": n.equipment, "findings": n.findings.json(), "location": location(&n.id),
                })
            })
            .collect();
        let edges: Vec<Value> = self
            .edges
            .iter()
            .map(|e| {
                json!({
                    "id": e.id, "label": e.label, "class": e.class, "source": self.nodes[e.source].id,
                    "target": self.nodes[e.target].id, "kV": e.kv, "findings": e.findings.json(), "location": location(&e.id),
                })
            })
            .collect();
        let layout = |l: &Layout| {
                let nodes: serde_json::Map<String, Value> = l.nodes.iter().map(|(&n, &(x, y))| (self.nodes[n].id.clone(), json!([x, y]))).collect();
                let edges: serde_json::Map<String, Value> = l
                    .edges
                    .iter()
                    .map(|(&e, ps)| (self.edges[e].id.clone(), json!(ps.iter().map(|&(x, y)| [x, y]).collect::<Vec<_>>())))
                    .collect();
                json!({ "id": l.id, "name": l.name, "kind": l.kind, "flipY": l.flip_y, "nodes": nodes, "edges": edges })
        };
        json!({
            "files": set.files.iter().map(|f| &f.url).collect::<Vec<_>>(),
            "nodes": nodes, "edges": edges, "elsewhere": self.elsewhere.json(),
            "layouts": self.layouts.iter().map(layout).collect::<Vec<_>>(), "auto": self.auto.as_ref().map(layout),
        })
    }

    /// What the substation `id` holds, for the view's drill-down; `None` if
    /// no substation of the set has that mRID.
    pub fn inside_json(&self, set: &ModelSet, id: &str) -> Option<Value> {
        let n = self.nodes.iter().position(|x| x.id == id && x.kind == "Substation")?;
        let node = &self.nodes[n];
        let empty = Inside::default();
        let inside = self.inside.get(&n).unwrap_or(&empty);
        let location = |id: &str| -> Value {
            match set.definition(id) {
                Some((file, el)) => json!({ "uri": file.url, "range": el.head }),
                None => Value::Null,
            }
        };
        let groups: Vec<Value> = inside.groups.iter().map(|g| json!({ "id": g.id, "label": g.label, "kV": g.kv })).collect();
        let items: Vec<Value> = inside
            .items
            .iter()
            .map(|it| {
                let far: Vec<Value> = it.far.iter().map(|&f| json!({ "id": self.nodes[f].id, "label": self.nodes[f].label, "kind": self.nodes[f].kind })).collect();
                json!({
                    "id": it.id, "label": it.label, "class": it.class, "junction": it.junction,
                    "group": it.group.map(|g| &inside.groups[g].id), "open": it.open, "inService": it.in_service,
                    "far": far, "findings": it.findings.json(), "location": location(&it.id),
                })
            })
            .collect();
        let links: Vec<Value> = inside
            .links
            .iter()
            .map(|l| json!({ "id": l.id, "source": inside.items[l.item].id, "target": inside.items[l.junction].id, "connected": l.connected }))
            .collect();
        let layouts: Vec<Value> = inside
            .layouts
            .iter()
            .map(|l| {
                let nodes: serde_json::Map<String, Value> = l.nodes.iter().map(|(&i, &(x, y))| (inside.items[i].id.clone(), json!([x, y]))).collect();
                json!({ "id": l.id, "name": l.name, "kind": l.kind, "flipY": l.flip_y, "nodes": nodes, "edges": {} })
            })
            .collect();
        Some(json!({
            "id": node.id, "label": node.label, "region": node.region, "kV": node.kv, "findings": inside.findings.json(),
            "location": location(&node.id), "groups": groups, "items": items, "links": links, "layouts": layouts,
        }))
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::validate::{Options, collect, validate};

    /// MicroGrid: Belgium's two substations and the Netherlands' one, joined
    /// through the boundary's five border nodes; each line an edge.
    #[test]
    fn micro_grid() {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../CGMES-Test-Configurations/v3.0/MicroGrid/MicroGid-BaseCase/MicroGrid-BaseCase-Merged");
        let files = collect(&dir, &Default::default());
        assert_eq!(files.len(), 12, "missing test data — run `git submodule update --init`");
        let (set, _) = validate(&files, &Options { common: true, quality: true, silence: vec![] });
        let g = &set.grid;
        let mut subs: Vec<&str> = g.nodes.iter().filter(|n| n.kind == "Substation").map(|n| n.label.as_str()).collect();
        subs.sort();
        assert_eq!(subs, ["Anvers", "PP_Amsterdam", "PP_Brussels"]);
        let mut borders: Vec<&str> = g.nodes.iter().filter(|n| n.kind == "ConnectivityNode").map(|n| n.label.as_str()).collect();
        borders.sort();
        assert_eq!(borders, ["Border_AL11", "Border_GY11", "Border_MA11", "Border_ST23", "Border_ST24"]);
        assert_eq!(g.edges.len(), 12);
        assert!(g.edges.iter().all(|e| e.class == "ACLineSegment"));

        let node = |label: &str| g.nodes.iter().find(|n| n.label == label).unwrap();
        let between = |a: &str, b: &str| {
            g.edges.iter().filter(|e| {
                let ends = [g.nodes[e.source].label.as_str(), g.nodes[e.target].label.as_str()];
                ends.contains(&a) && ends.contains(&b)
            }).count()
        };
        assert_eq!(between("PP_Brussels", "Anvers"), 2);
        assert_eq!(between("PP_Amsterdam", "Border_AL11"), 1);
        assert_eq!(between("PP_Brussels", "Border_AL11"), 1);
        let brussels = node("PP_Brussels");
        assert_eq!(brussels.kv, [380.0, 225.0, 110.0, 21.0, 10.5]);
        assert_eq!(brussels.equipment["SynchronousMachine"], 2);
        assert_eq!(brussels.region.as_deref(), Some("ELIA-Brussels"));

        // Every finding counted once, and each node clicks through to its definition.
        let json = g.json(&set);
        let total: u64 = json["nodes"].as_array().unwrap().iter().chain(json["edges"].as_array().unwrap())
            .chain(std::iter::once(&json["elsewhere"]))
            .map(|x| { let f = if x.get("findings").is_some() { &x["findings"] } else { x }; f["violation"].as_u64().unwrap() + f["warning"].as_u64().unwrap() + f["info"].as_u64().unwrap() })
            .sum();
        let datasets: Vec<_> = files.iter().map(|(_, t)| cimmodel::CimDataset::decode_str(t).unwrap()).collect();
        let cfg = cimvalidation::combined_config(&datasets, None, None, true, true, vec![]);
        assert_eq!(total as usize, cimvalidation::validate_files(datasets, &cfg).len());
        let anvers = json["nodes"].as_array().unwrap().iter().find(|n| n["label"] == "Anvers").unwrap();
        assert!(anvers["location"]["uri"].as_str().unwrap().ends_with("_BE_EQ_001.xml"), "{anvers}");

        // The assembled DL diagram and the GL locations place every node and route every line.
        let names: Vec<&str> = g.layouts.iter().map(|l| l.name.as_str()).collect();
        assert_eq!(names, ["Diagram: Assembled", "Geographic (EPSG:4326)"]);
        for l in &g.layouts {
            assert_eq!((l.nodes.len(), l.edges.len()), (8, 12), "{}", l.name);
        }
        let index = |label: &str| g.nodes.iter().position(|n| n.label == label).unwrap();
        let (diagram, map) = (&g.layouts[0], &g.layouts[1]);
        assert!(!diagram.flip_y);
        // Brussels and Anvers left of the border points, Amsterdam right of them.
        let x = |label: &str| diagram.nodes[&index(label)].0;
        assert!(x("PP_Brussels") < x("Anvers") && x("Anvers") < x("Border_AL11") && x("Border_AL11") < x("PP_Amsterdam"));
        // Brussels in Brussels, from the ends of its lines; Amsterdam north of it.
        let (lon, lat) = map.nodes[&index("PP_Brussels")];
        assert!((lon - 4.30).abs() < 0.01 && (lat - 50.80).abs() < 0.01, "{lon} {lat}");
        assert!(map.nodes[&index("PP_Amsterdam")].1 > 52.3);
        // A route runs from its source's end, through the line's points.
        let line = g.edges.iter().position(|e| e.label == "BE-Line_2").unwrap();
        assert_eq!(map.edges[&line].len(), 5);
        let (start, source) = (map.edges[&line][0], map.nodes[&g.edges[line].source]);
        assert!((start.0 - source.0).abs() < 0.01 && (start.1 - source.1).abs() < 0.01, "{start:?} {source:?}");
        assert_eq!(json["layouts"][1]["nodes"].as_object().unwrap().len(), 8);

    }

    /// MicroGrid Type 1, node-breaker: inside a substation, its breakers,
    /// busbars and connectivity nodes by voltage level, its lines with their
    /// far ends, and its switches' states; placed on the DL diagrams.
    #[test]
    fn micro_grid_inside() {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../CGMES-Test-Configurations/v3.0/MicroGrid/MicroGrid-Type1/MicroGrid-Type1-Merged");
        let mut files = collect(&dir, &Default::default());
        assert_eq!(files.len(), 12, "missing test data — run `git submodule update --init`");
        // A finding on a breaker (no normalOpen) and one on a voltage level (no name).
        let eq = files.iter_mut().find(|(p, _)| p.to_string_lossy().ends_with("_BE_EQ_1.xml")).unwrap();
        let mut text = eq.1.to_string();
        for written in ["<cim:Switch.normalOpen>false</cim:Switch.normalOpen>", "<cim:IdentifiedObject.name>225.0</cim:IdentifiedObject.name>"] {
            let at = text.find(written).unwrap();
            text.replace_range(at..at + written.len(), "");
        }
        eq.1 = text.into();
        let (set, _) = validate(&files, &Options { common: true, quality: true, silence: vec![] });
        let g = &set.grid;
        let index = |label: &str| g.nodes.iter().position(|n| n.label == label).unwrap();

        // Each substation's findings are its items' and its own, once each.
        let total = |c: &super::Counts| c.violation + c.warning + c.info;
        for (n, inside) in &g.inside {
            let items: usize = inside.items.iter().map(|it| total(&it.findings)).sum();
            assert_eq!(items + total(&inside.findings), total(&g.nodes[*n].findings), "{}", g.nodes[*n].label);
        }
        let breaker = g.inside.values().flat_map(|i| &i.items).find(|it| it.label == "CIRCB-1230992285").unwrap();
        assert!(breaker.findings.violation > 0, "{:?}", breaker.findings);
        let vl = g.inside.values().find(|i| i.groups.iter().any(|gr| gr.id.ends_with("69ef0dbd-da79-4eef-a02f-690cb8a28361"))).unwrap();
        assert!(total(&vl.findings) > 0, "{:?}", vl.findings);

        let anvers = &g.inside[&index("Anvers")];
        let count = |class: &str| anvers.items.iter().filter(|it| it.class == class).count();
        assert_eq!((count("Breaker"), count("BusbarSection"), count("ConnectivityNode"), count("ACLineSegment")), (4, 3, 6, 4));
        assert_eq!(anvers.groups.iter().map(|gr| gr.kv).collect::<Vec<_>>(), [Some(225.0)]);
        assert_eq!(anvers.links.len(), 19);
        // Every link runs from equipment to a junction.
        assert!(anvers.links.iter().all(|l| !anvers.items[l.item].junction && anvers.items[l.junction].junction));
        let line = anvers.items.iter().find(|it| it.label == "BE-Line_2").unwrap();
        assert_eq!((line.group, line.far.iter().map(|&f| g.nodes[f].label.as_str()).collect::<Vec<_>>()), (None, vec!["PP_Brussels"]));
        assert!(anvers.items.iter().filter(|it| it.class == "Breaker").all(|it| it.open == Some(false)));

        // Amsterdam's bus coupler is open and out of service; its transformers sit between voltage levels.
        let amsterdam = &g.inside[&index("PP_Amsterdam")];
        let coupler = amsterdam.items.iter().find(|it| it.label == "busbarcoupler 1").unwrap();
        assert_eq!((coupler.open, coupler.in_service), (Some(true), Some(false)));
        assert!(amsterdam.items.iter().filter(|it| it.class == "PowerTransformer").all(|it| it.group.is_none()));
        assert_eq!(amsterdam.groups.len(), 3);

        // Both diagrams that draw Anvers place its equipment and nodes.
        let names: Vec<&str> = anvers.layouts.iter().map(|l| l.name.as_str()).collect();
        assert_eq!(names, ["Diagram: MAS BE", "Diagram: Assembled"]);
        assert!(anvers.layouts.iter().all(|l| l.nodes.len() > 10), "{:?}", anvers.layouts.iter().map(|l| l.nodes.len()).collect::<Vec<_>>());

        let json = g.inside_json(&set, &g.nodes[index("Anvers")].id).unwrap();
        assert_eq!(json["items"].as_array().unwrap().len(), 20);
        assert!(json["items"][0]["location"]["uri"].as_str().unwrap().ends_with("_BE_EQ_1.xml"));
        assert!(g.inside_json(&set, "no such substation").is_none());
    }
}

