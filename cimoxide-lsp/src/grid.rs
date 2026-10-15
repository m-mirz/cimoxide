//! A model set's grid at substation level, for the extension's grid view:
//! substations (and boundary points no substation holds) as nodes, the
//! equipment joining two of them (lines, mostly) as edges, and each
//! object's findings counted on the node or edge it belongs to.
//!
//! Built from the set's files before validation consumes them; the files are
//! read as one model (a field from whichever file writes it), without
//! merging them.

use std::collections::{BTreeMap, HashMap};

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
    fn add(&mut self, severity: &str) {
        match severity {
            "sh:Violation" => self.violation += 1,
            "sh:Warning" => self.warning += 1,
            _ => self.info += 1,
        }
    }

    fn json(&self) -> Value {
        json!({ "violation": self.violation, "warning": self.warning, "info": self.info })
    }
}

#[derive(Debug, Default)]
pub struct Grid {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    /// Each placed object, by mRID.
    placed: HashMap<String, Place>,
    /// Findings on objects with no node or edge (headers, regions, …).
    pub elsewhere: Counts,
}

/// The set's files read as one model: each object's elements, the defining
/// one first.
struct Model<'a> {
    objects: HashMap<&'a str, Vec<&'a Element>>,
    /// ConductingEquipment → its terminals, by sequence number.
    terminals: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> Model<'a> {
    fn new(datasets: &'a [CimDataset]) -> Self {
        let mut objects: HashMap<&str, Vec<&Element>> = HashMap::new();
        for ds in datasets {
            for (mrid, el) in &ds.entries {
                let els = objects.entry(mrid.as_str()).or_default();
                if el.defined() { els.insert(0, el) } else { els.push(el) }
            }
        }
        let mut model = Self { objects, terminals: HashMap::new() };
        let mut terminals: HashMap<&str, Vec<(i64, &str)>> = HashMap::new();
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

    fn class(&self, mrid: &str) -> Option<&'static str> {
        self.objects.get(mrid).map(|els| els[0].type_name())
    }

    fn text(&self, mrid: &str, key: &str) -> Option<&'a str> {
        self.objects.get(mrid)?.iter().find_map(|el| el.get_str(key))
    }

    fn number(&self, mrid: &str, key: &str) -> Option<f64> {
        self.text(mrid, key)?.trim().parse().ok()
    }

    /// The object a field refers to, if it is in the set.
    fn reference(&self, mrid: &str, key: &str) -> Option<&'a str> {
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
        let node = self
            .reference(terminal, "Terminal.ConnectivityNode")
            .or_else(|| self.reference(terminal, "Terminal.TopologicalNode"))?;
        let container = self
            .reference(node, "ConnectivityNode.ConnectivityNodeContainer")
            .or_else(|| self.reference(node, "TopologicalNode.ConnectivityNodeContainer"));
        match container.and_then(|c| self.substation(c)) {
            Some(sub) => Some((sub, false)),
            None => Some((node, true)),
        }
    }
}

/// The grid of a model set's files.
pub fn build(datasets: &[CimDataset]) -> Grid {
    let m = Model::new(datasets);
    let mut grid = Grid::default();
    let mut node_of: HashMap<&str, usize> = HashMap::new();

    let mut subs: Vec<&str> = m.objects.keys().copied().filter(|k| m.class(k) == Some("Substation")).collect();
    subs.sort_by_key(|s| (m.name(s), *s));
    for sub in subs {
        node_of.insert(sub, grid.nodes.len());
        let region = m.reference(sub, "Substation.Region").map(|r| m.name(r));
        grid.nodes.push(Node { id: sub.into(), label: m.name(sub), kind: "Substation", region, ..Default::default() });
    }

    // Equipment whose terminals reach two different nodes is an edge.
    let mut equipment: Vec<&str> = m.terminals.keys().copied().collect();
    equipment.sort();
    for ce in equipment {
        let mut ends: Vec<(&str, bool)> = Vec::new();
        for t in &m.terminals[ce] {
            if let Some(end) = m.terminal_end(t)
                && !ends.iter().any(|e| e.0 == end.0)
            {
                ends.push(end);
            }
        }
        if ends.len() < 2 {
            continue;
        }
        let source = node_for(&m, &mut grid, &mut node_of, ends[0]);
        for &end in &ends[1..] {
            let target = node_for(&m, &mut grid, &mut node_of, end);
            let kv = m.reference(ce, "ConductingEquipment.BaseVoltage").and_then(|b| m.number(b, "BaseVoltage.nominalVoltage"));
            let id = if ends.len() == 2 { ce.to_string() } else { format!("{ce}#{}", grid.edges.len()) };
            grid.placed.insert(ce.to_string(), Place::Edge(grid.edges.len()));
            grid.edges.push(Edge { id, label: m.name(ce), class: m.class(ce).unwrap_or(""), source, target, kv, findings: Counts::default() });
        }
    }

    // Every other object goes where its container, equipment or node is.
    let mut ids: Vec<&str> = m.objects.keys().copied().collect();
    ids.sort();
    for id in ids {
        if let Some(Place::Node(n)) = place(&m, &node_of, &mut grid.placed, id, 0)
            && let Some(container) = m.reference(id, "Equipment.EquipmentContainer")
            && m.substation(container).is_some()
        {
            *grid.nodes[n].equipment.entry(m.class(id).unwrap_or("")).or_default() += 1;
        }
    }
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
    grid
}

/// The node of an edge's end, added on first use: a substation, or a
/// boundary point.
fn node_for<'a>(m: &Model<'a>, grid: &mut Grid, node_of: &mut HashMap<&'a str, usize>, (id, boundary): (&'a str, bool)) -> usize {
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
/// object that has a place.
fn place<'a>(m: &Model<'a>, node_of: &HashMap<&str, usize>, placed: &mut HashMap<String, Place>, id: &'a str, depth: u8) -> Option<Place> {
    if let Some(p) = placed.get(id) {
        return Some(*p);
    }
    if depth > 3 {
        return None;
    }
    let class = m.class(id)?;
    let direct = match class {
        "Substation" | "VoltageLevel" | "Bay" => m.substation(id).and_then(|s| node_of.get(s)).map(|&n| Place::Node(n)),
        "ConnectivityNode" | "TopologicalNode" => match node_of.get(id) {
            Some(&n) => Some(Place::Node(n)),
            None => {
                let key = if class == "ConnectivityNode" { "ConnectivityNode.ConnectivityNodeContainer" } else { "TopologicalNode.ConnectivityNodeContainer" };
                m.reference(id, key).and_then(|c| m.substation(c)).and_then(|s| node_of.get(s)).map(|&n| Place::Node(n))
            }
        },
        "Terminal" => m.reference(id, "Terminal.ConductingEquipment").and_then(|ce| place(m, node_of, placed, ce, depth + 1)),
        _ => m.reference(id, "Equipment.EquipmentContainer").and_then(|c| m.substation(c)).and_then(|s| node_of.get(s)).map(|&n| Place::Node(n)),
    };
    let found = direct.or_else(|| {
        // Limits, tap changers, regulating controls, state variables, …:
        // where what they refer to is.
        let els = m.objects.get(id)?;
        let mut keys: Vec<&str> = els.iter().flat_map(|el| el.fields().keys().copied()).collect();
        keys.sort();
        keys.dedup();
        keys.into_iter()
            .filter(|k| !k.ends_with(".BaseVoltage") && !k.ends_with(".Region"))
            .find_map(|k| m.reference(id, k).filter(|r| *r != id).and_then(|r| place(m, node_of, placed, r, depth + 1)))
    })?;
    placed.insert(id.to_string(), found);
    Some(found)
}

impl Grid {
    /// Count each finding on its object's node or edge.
    pub fn add_findings(&mut self, violations: &[(Option<usize>, Violation)]) {
        for (_, v) in violations {
            match self.placed.get(v.object_id.as_str()) {
                Some(Place::Node(n)) => self.nodes[*n].findings.add(&v.severity),
                Some(Place::Edge(e)) => self.edges[*e].findings.add(&v.severity),
                None => self.elsewhere.add(&v.severity),
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
        json!({
            "files": set.files.iter().map(|f| &f.url).collect::<Vec<_>>(),
            "nodes": nodes, "edges": edges, "elsewhere": self.elsewhere.json(),
        })
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
    }
}
