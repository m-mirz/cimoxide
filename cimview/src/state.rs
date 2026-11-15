//! The state a model set's SV profile publishes, as a power flow's overlay of the grid: each
//! node's voltage, each terminal's flow and its loading against the lowest PATL, and each grid
//! node's range of voltages. In the shape a solved power flow overlays the grid with, so the
//! views draw either the same way.

use std::collections::BTreeMap;

use serde_json::{Map, Value, json};

use crate::grid::{Grid, Model};

/// Rounded for display, as a power flow's numbers are.
fn round(x: f64, digits: i32) -> f64 {
    let k = 10f64.powi(digits);
    (x * k).round() / k + 0.0
}

/// The objects of `class` in `m`, in id order.
fn all<'a>(m: &Model<'a>, class: &str) -> Vec<&'a str> {
    let mut ids: Vec<&'a str> = m.objects.iter().filter(|(_, els)| els[0].type_name() == class).map(|(id, _)| *id).collect();
    ids.sort_unstable();
    ids
}

/// A topological node's nominal voltage, kV: its base voltage's, else its voltage level's.
fn nominal(m: &Model<'_>, node: &str) -> Option<f64> {
    let base = m.reference(node, "TopologicalNode.BaseVoltage").or_else(|| {
        let vl = m.node_container(node).and_then(|c| m.voltage_level(c))?;
        m.reference(vl, "VoltageLevel.BaseVoltage")
    })?;
    m.number(base, "BaseVoltage.nominalVoltage").filter(|v| *v > 0.0)
}

/// Each terminal's lowest PATL, A.
fn patls<'a>(m: &Model<'a>) -> BTreeMap<&'a str, f64> {
    let mut out: BTreeMap<&str, f64> = BTreeMap::new();
    for limit in all(m, "CurrentLimit") {
        let amps = m.number(limit, "CurrentLimit.value").or_else(|| m.number(limit, "CurrentLimit.normalValue")).unwrap_or(0.0);
        let patl = m
            .reference(limit, "OperationalLimit.OperationalLimitType")
            .and_then(|t| m.target(t, "OperationalLimitType.kind").or_else(|| m.text(t, "OperationalLimitType.kind")))
            .is_some_and(|k| k.ends_with("patl"));
        let terminal = m.reference(limit, "OperationalLimit.OperationalLimitSet").and_then(|s| m.reference(s, "OperationalLimitSet.Terminal"));
        if let Some(t) = terminal.filter(|_| patl && amps > 0.0) {
            let at = out.entry(t).or_insert(amps);
            *at = at.min(amps);
        }
    }
    out
}

/// The SV's state of `m` over `grid`, as `{ nodes, voltages, flows }` (see the module); `None` if
/// the set publishes no voltage or flow.
pub fn overlay(m: &Model<'_>, grid: &Grid) -> Option<Value> {
    let (svv, svf) = (all(m, "SvVoltage"), all(m, "SvPowerFlow"));
    if svv.is_empty() && svf.is_empty() {
        return None;
    }
    // Each topological node's connectivity nodes.
    let mut cns: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for cn in all(m, "ConnectivityNode") {
        if let Some(tn) = m.reference(cn, "ConnectivityNode.TopologicalNode") {
            cns.entry(tn).or_default().push(cn);
        }
    }
    let place = |node: &str| -> Option<String> {
        let at = |id: &str| {
            let found = grid.reveal(id)?;
            let enter = found.get("enter").and_then(Value::as_str);
            let select = found.get("select").and_then(Value::as_str);
            enter.or(select).map(str::to_string)
        };
        at(node).or_else(|| cns.get(node)?.iter().find_map(|cn| at(cn)))
    };

    // Voltages, by topological and connectivity node; kV by topological node, for currents.
    let mut voltages = Map::new();
    let mut kv_at: BTreeMap<&str, f64> = BTreeMap::new();
    let mut on_grid: BTreeMap<String, (f64, f64)> = BTreeMap::new();
    for sv in svv {
        let (Some(tn), Some(v)) = (m.reference(sv, "SvVoltage.TopologicalNode"), m.number(sv, "SvVoltage.v")) else { continue };
        if v <= 0.0 {
            continue;
        }
        kv_at.insert(tn, v);
        let deg = m.number(sv, "SvVoltage.angle").unwrap_or(0.0);
        let Some(kv) = nominal(m, tn) else { continue };
        let pu = v / kv;
        let value = json!({ "pu": round(pu, 5), "kV": round(v, 3), "deg": round(deg, 4) });
        for cn in cns.get(tn).into_iter().flatten() {
            voltages.insert(cn.to_string(), value.clone());
        }
        voltages.insert(tn.to_string(), value);
        if let Some(at) = place(tn) {
            let range = on_grid.entry(at).or_insert((f64::INFINITY, f64::NEG_INFINITY));
            *range = (range.0.min(pu), range.1.max(pu));
        }
    }

    // A busbar section, drawn as its bar: its node's voltage.
    for bar in all(m, "BusbarSection") {
        let terminals = m.terminals.get(bar).into_iter().flatten();
        let found = terminals.filter_map(|t| m.reference(t, "Terminal.TopologicalNode").or_else(|| m.reference(t, "Terminal.ConnectivityNode"))).find_map(|n| voltages.get(n).cloned());
        if let Some(v) = found {
            voltages.insert(bar.to_string(), v);
        }
    }

    // Flows, by equipment: each terminal's, into it, and its loading.
    let limits = patls(m);
    let mut flows: BTreeMap<&str, Vec<Value>> = BTreeMap::new();
    for sv in svf {
        let Some(terminal) = m.reference(sv, "SvPowerFlow.Terminal") else { continue };
        let Some(equipment) = m.reference(terminal, "Terminal.ConductingEquipment") else { continue };
        let (p, q) = (m.number(sv, "SvPowerFlow.p").unwrap_or(0.0), m.number(sv, "SvPowerFlow.q").unwrap_or(0.0));
        let tn = m
            .reference(terminal, "Terminal.TopologicalNode")
            .or_else(|| m.reference(terminal, "Terminal.ConnectivityNode").and_then(|cn| m.reference(cn, "ConnectivityNode.TopologicalNode")));
        let amps = tn.and_then(|tn| kv_at.get(tn)).map(|kv| 1e3 * p.hypot(q) / (3f64.sqrt() * kv));
        let loading = amps.zip(limits.get(terminal)).map(|(a, l)| round(100.0 * a / l, 2));
        flows.entry(equipment).or_default().push(json!({
            "terminal": terminal, "p": round(p, 3), "q": round(q, 3),
            "amps": amps.map(|a| round(a, 2)), "loading": loading,
        }));
    }
    let flows: Map<String, Value> = flows
        .into_iter()
        .map(|(equipment, ends)| {
            let loading = ends.iter().filter_map(|e| e["loading"].as_f64()).reduce(f64::max);
            (equipment.to_string(), json!({ "ends": ends, "loading": loading }))
        })
        .collect();
    Some(json!({
        "nodes": on_grid.into_iter().map(|(at, (lo, hi))| (at, json!({ "min": round(lo, 4), "max": round(hi, 4) }))).collect::<Map<_, _>>(),
        "voltages": voltages,
        "flows": flows,
    }))
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use cimmodel::CimDataset;

    use crate::grid::{Model, build};
    use crate::testing::collect;

    fn micro_grid() -> Vec<CimDataset> {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../CGMES-Test-Configurations/v3.0/MicroGrid/MicroGrid-Type1/MicroGrid-Type1-Merged");
        collect(&dir).iter().map(|(_, t)| CimDataset::decode_str(t).unwrap()).collect()
    }

    #[test]
    fn the_sv_state_overlays_the_grid() {
        let datasets = micro_grid();
        let m = Model::new(&datasets);
        let grid = build(&m);
        let o = super::overlay(&m, &grid).expect("MicroGrid has an SV");
        // Every substation drawn has its voltages, near nominal.
        let anvers = grid.nodes.iter().find(|n| n.label == "Anvers").unwrap();
        let range = &o["nodes"][&anvers.id];
        let (lo, hi) = (range["min"].as_f64().unwrap(), range["max"].as_f64().unwrap());
        assert!(lo > 0.9 && hi < 1.1, "Anvers {lo}–{hi} pu");
        // A busbar's node, inside: a voltage, by its connectivity node too.
        let inside = grid.inside_of(&anvers.id).unwrap();
        let junctions: Vec<_> = inside.items.iter().filter(|it| it.junction).collect();
        assert!(junctions.iter().any(|j| o["voltages"][&j.id]["pu"].is_number()), "a junction's voltage");
        // And each busbar section, as its bar.
        for bar in inside.items.iter().filter(|it| it.class == "BusbarSection") {
            assert!(o["voltages"][&bar.id]["pu"].is_number(), "{}: no voltage", bar.label);
        }
        // A line's flows: its two ends, p into each, a current and a loading against its PATL.
        let line = inside.items.iter().find(|it| it.class == "ACLineSegment").unwrap();
        let f = &o["flows"][&line.id];
        let ends = f["ends"].as_array().unwrap();
        assert_eq!(ends.len(), 2, "{f}");
        assert!(ends.iter().all(|e| e["amps"].as_f64().is_some_and(|a| a > 0.0)), "{f}");
        assert!(f["loading"].as_f64().is_some_and(|l| l > 0.0 && l < 200.0), "{f}");
    }

    #[test]
    fn without_an_sv_there_is_no_state() {
        let datasets: Vec<CimDataset> = micro_grid().into_iter().filter(|d| !d.by_type.contains_key("SvVoltage") && !d.by_type.contains_key("SvPowerFlow")).collect();
        let m = Model::new(&datasets);
        assert!(super::overlay(&m, &build(&m)).is_none());
    }
}
