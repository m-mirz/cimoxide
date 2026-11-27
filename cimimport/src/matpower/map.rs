//! A parsed MATPOWER case as a CGMES model: EQ, SSH, TP and SV.
//!
//! The rules follow what MATPOWER's `runpf` reads, as gridoxide's own MATPOWER
//! importer does, so that solving the converted model gives what solving the
//! case gives:
//!
//! - **Buses**: one `BusbarSection` with its `ConnectivityNode` and one
//!   `TopologicalNode` each, in a `VoltageLevel` of its base voltage. A bus
//!   without one (`baseKV` 0, as in `case14`) gets 1 kV: per unit, any base
//!   gives the same network.
//! - **Substations**: buses joined by a transformer share one, and so do buses
//!   whose names differ only in a trailing section number (`MASON CITY 3`)
//!   where a branch joins them.
//! - **Branches**: a branch without a ratio (0 or 1), without a shift and
//!   between equal base voltages is an `ACLineSegment` in a `Line` of its own;
//!   without charging, one with a negative reactance is a `SeriesCompensator`
//!   and one with a negative resistance an `EquivalentBranch`, as CGMES has
//!   no negative line impedances. Any other branch is a two-winding
//!   `PowerTransformer`, built as MATPOWER's model is: on the from side an
//!   ideal transformer of ratio `tap` (end 2's `ratedU`) and shift (a fixed
//!   `PhaseTapChangerLinear`), then the series impedance and the charging
//!   (end 1, on the to side's base). CGMES's end 1 is the winding of the
//!   highest `ratedU`; where that is the from side, the transformer is built
//!   mirrored, the ideal transformer on the to side and the impedance it
//!   moved across scaled by `tap²`, which is the same two-port. A breaker
//!   sits at each end of each branch; an out-of-service branch has both open,
//!   its ends on topological nodes of their own.
//! - **Generators**: `SynchronousMachine`s, of the kind their `Pmin`/`Pmax`
//!   make them (generator, motor, both, or condenser). A PV or reference bus
//!   keeps its type only with an online generator, and holds the `Vg` of the
//!   last one in file order, through one `RegulatingControl` its generators
//!   share. If no reference bus remains, the first PV bus becomes the
//!   reference, as `bustypes` does; its machines are the only ones with a
//!   participation factor (`normalPF`), MATPOWER's slack being one bus.
//! - **Loads**: an `EnergyConsumer` per bus with `Pd` or `Qd`, or an
//!   `EquivalentInjection` where either is negative, which CGMES does not let
//!   a load be. `Gs`/`Bs` are a one-section `LinearShuntCompensator`.
//! - **Isolated buses** (type 4) are kept, with everything on them
//!   disconnected and the branches touching them switched out.
//! - **State**: the SV holds the case's `Vm`/`Va`, its flows where it has
//!   them, and its topological islands with their angle references.
//!
//! What the case gives that CGMES's value checks reject, a transformer with a
//! positive `b` (MATPOWER's charging) or a negative impedance, is kept as
//! given, and noted: changing it would convert a different network.

use std::collections::{BTreeMap, HashMap};

use super::parse::*;
use crate::build::{Builder, Header};
use crate::{Converted, Options};

/// `MASON CITY` for `MASON CITY 3`, `Olive` for `Olive V2`: a name without a
/// trailing section or voltage-level number. A name that is nothing but that
/// number stays whole.
pub fn station_name(name: &str) -> &str {
    let name = name.trim();
    let Some((rest, last)) = name.rsplit_once(char::is_whitespace) else {
        return name;
    };
    let digits = last
        .strip_prefix(|c: char| c.is_ascii_alphabetic())
        .unwrap_or(last);
    if !rest.trim().is_empty() && !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit()) {
        rest.trim_end()
    } else {
        name
    }
}

/// A row of `mpc.branch`, resolved.
struct Branch {
    row: usize,
    from: usize,
    to: usize,
    transformer: bool,
    out: bool,
    key: String,
    name: String,
}

struct UnionFind(Vec<usize>);

impl UnionFind {
    fn new(n: usize) -> Self {
        Self((0..n).collect())
    }
    fn find(&mut self, mut i: usize) -> usize {
        while self.0[i] != i {
            self.0[i] = self.0[self.0[i]];
            i = self.0[i];
        }
        i
    }
    /// Joins the sets of `a` and `b`, the lower root winning, so a set's root
    /// is its first bus in file order. Returns whether they were apart.
    fn union(&mut self, a: usize, b: usize) -> bool {
        let (ra, rb) = (self.find(a), self.find(b));
        if ra == rb {
            return false;
        }
        let (keep, gone) = if ra < rb { (ra, rb) } else { (rb, ra) };
        self.0[gone] = keep;
        true
    }
}

/// `230` for 230 kV, `13.8` for 13.8: how a base voltage reads in a key or a
/// name.
fn kv_text(kv: f64) -> String {
    kv.to_string()
}

/// Above zero, and so not NaN.
fn positive(v: f64) -> bool {
    v > 0.0
}

/// A column that a short row does not have reads as `default`.
fn col(row: &[f64], i: usize, default: f64) -> f64 {
    row.get(i).copied().unwrap_or(default)
}

pub fn convert(
    case: &MatpowerCase,
    stem: &str,
    options: &Options,
) -> Result<Converted, MatpowerError> {
    let n = case.bus.len();
    let base_mva = case.base_mva;
    let seed = case.name.as_deref().unwrap_or(stem);
    let mut notes: Vec<String> = Vec::new();

    // --- Buses: numbers, names, types ---
    let mut index: HashMap<u64, usize> = HashMap::with_capacity(n);
    let mut numbers = Vec::with_capacity(n);
    for (row, b) in case.bus.iter().enumerate() {
        let number = bus_number(b[BUS_I]).ok_or_else(|| MatpowerError::Parse {
            line: 0,
            message: format!("mpc.bus row {} has bus number {}", row + 1, b[BUS_I]),
        })?;
        if index.insert(number, row).is_some() {
            return Err(MatpowerError::Unsupported(format!(
                "mpc.bus defines bus {number} twice"
            )));
        }
        numbers.push(number);
    }
    let resolve = |matrix: &'static str, row: usize, value: f64| {
        bus_number(value)
            .and_then(|b| index.get(&b).copied())
            .ok_or(MatpowerError::UnknownBus {
                matrix,
                row: row + 1,
                bus: value,
            })
    };
    let names: Vec<String> = match &case.bus_name {
        Some(names) => names
            .iter()
            .zip(&numbers)
            .map(|(name, number)| match name.trim() {
                "" => format!("Bus {number}"),
                // Names padded into columns (`Bus 4     HV`) read with one blank.
                name => name.split_whitespace().collect::<Vec<_>>().join(" "),
            })
            .collect(),
        None => numbers.iter().map(|n| format!("Bus {n}")).collect(),
    };
    // A case without base voltages (`case14` has 0 throughout) is per unit
    // all the same; any base keeps its per-unit values, and 1 kV reads as the
    // placeholder it is.
    let kv: Vec<f64> = case
        .bus
        .iter()
        .map(|b| if b[BASE_KV] > 0.0 { b[BASE_KV] } else { 1.0 })
        .collect();
    let no_base = case.bus.iter().filter(|b| !positive(b[BASE_KV])).count();
    if no_base > 0 {
        notes.push(format!(
            "{no_base} bus(es) without a base voltage (baseKV 0) given 1 kV; \
             their per-unit values are kept"
        ));
    }
    let isolated: Vec<bool> = case.bus.iter().map(|b| b[BUS_TYPE] == NONE).collect();

    let gen_bus: Vec<usize> = case
        .gens
        .iter()
        .enumerate()
        .map(|(row, g)| resolve("gen", row, g[GEN_BUS]))
        .collect::<Result<_, _>>()?;
    let gen_online: Vec<bool> = case
        .gens
        .iter()
        .zip(&gen_bus)
        .map(|(g, &at)| g[GEN_STATUS] > 0.0 && !isolated[at])
        .collect();
    // The setpoint of a regulated bus: the last online generator's Vg, as
    // MATPOWER's `V0(gbus) = gen(on, VG)` assigns it.
    let mut vg: Vec<Option<f64>> = vec![None; n];
    let mut vg_conflict = 0usize;
    for (k, g) in case.gens.iter().enumerate() {
        if gen_online[k] {
            let at = gen_bus[k];
            if vg[at].is_some_and(|v| v != g[VG]) {
                vg_conflict += 1;
            }
            vg[at] = Some(g[VG]);
        }
    }
    if vg_conflict > 0 {
        notes.push(format!(
            "{vg_conflict} generator(s) share a bus with another of a different Vg; \
             the bus holds the last one's, as MATPOWER does"
        ));
    }

    #[derive(Clone, Copy, PartialEq)]
    enum Kind {
        Pq,
        Pv,
        Slack,
    }
    let mut kinds: Vec<Kind> = case
        .bus
        .iter()
        .zip(&vg)
        .map(|(b, vg)| match (b[BUS_TYPE], vg) {
            (t, Some(_)) if t == REF => Kind::Slack,
            (t, Some(_)) if t == PV => Kind::Pv,
            _ => Kind::Pq,
        })
        .collect();
    let demoted = case
        .bus
        .iter()
        .zip(&vg)
        .filter(|(b, vg)| (b[BUS_TYPE] == PV || b[BUS_TYPE] == REF) && vg.is_none())
        .count();
    if demoted > 0 {
        notes.push(format!(
            "{demoted} PV or reference bus(es) without an online generator hold no voltage"
        ));
    }
    if !kinds.contains(&Kind::Slack)
        && let Some(first_pv) = kinds.iter().position(|k| *k == Kind::Pv)
    {
        kinds[first_pv] = Kind::Slack;
        notes.push(format!(
            "no reference bus with an online generator; bus {} (the first PV bus) is the reference",
            numbers[first_pv]
        ));
    }
    let unknown_types = case
        .bus
        .iter()
        .filter(|b| ![PQ, PV, REF, NONE].contains(&b[BUS_TYPE]))
        .count();
    if unknown_types > 0 {
        notes.push(format!(
            "{unknown_types} bus(es) of an unknown type read as PQ"
        ));
    }
    let isolated_count = isolated.iter().filter(|&&i| i).count();
    if isolated_count > 0 {
        notes.push(format!(
            "{isolated_count} isolated bus(es) kept, with their equipment disconnected and \
             their branches switched out"
        ));
    }

    // --- Branches: ends, kind, status ---
    let mut parallel: HashMap<(usize, usize), usize> = HashMap::new();
    let mut ends = Vec::with_capacity(case.branch.len());
    for (row, br) in case.branch.iter().enumerate() {
        let from = resolve("branch", row, br[F_BUS])?;
        let to = resolve("branch", row, br[T_BUS])?;
        *parallel.entry((from, to)).or_default() += 1;
        ends.push((from, to));
    }
    let mut seen: HashMap<(usize, usize), usize> = HashMap::new();
    let branches: Vec<Branch> = case
        .branch
        .iter()
        .zip(&ends)
        .enumerate()
        .map(|(row, (br, &(from, to)))| {
            let k = {
                let k = seen.entry((from, to)).or_default();
                *k += 1;
                *k
            };
            let mut name = format!("{} - {}", names[from], names[to]);
            if parallel[&(from, to)] > 1 {
                name.push_str(&format!(" ({k})"));
            }
            Branch {
                row,
                from,
                to,
                // A ratio of 1 is no ratio, as 0 is (RTE's cases write 1 on
                // every line).
                transformer: (br[TAP] != 0.0 && br[TAP] != 1.0)
                    || br[SHIFT] != 0.0
                    || kv[from] != kv[to],
                out: br[BR_STATUS] == 0.0 || isolated[from] || isolated[to],
                key: format!("branch:{}-{}:{k}", numbers[from], numbers[to]),
                name,
            }
        })
        .collect();

    // --- Substations ---
    let mut groups = UnionFind::new(n);
    for b in branches.iter().filter(|b| b.transformer) {
        groups.union(b.from, b.to);
    }
    let by_transformer = (0..n).filter(|&i| groups.find(i) == i).count();
    if options.name_grouping && case.bus_name.is_some() {
        for b in &branches {
            if b.from != b.to && station_name(&names[b.from]) == station_name(&names[b.to]) {
                groups.union(b.from, b.to);
            }
        }
    }
    let group: Vec<usize> = (0..n).map(|i| groups.find(i)).collect();
    let substations = (0..n).filter(|&i| group[i] == i).count();
    if substations < by_transformer {
        notes.push(format!(
            "{n} buses in {substations} substations: grouped by transformer \
             ({by_transformer} substations), then by name where a branch joins buses"
        ));
    } else {
        notes.push(format!(
            "{n} buses in {substations} substations, grouped by transformer"
        ));
    }

    let mut b = Builder::new(seed);

    let region = b.add("GeographicalRegion", "region", seed);
    let mut areas: BTreeMap<i64, String> = BTreeMap::new();
    for bus in &case.bus {
        let area = bus[BUS_AREA] as i64;
        areas.entry(area).or_insert_with(|| {
            let id = b.add(
                "SubGeographicalRegion",
                &format!("area:{area}"),
                &format!("Area {area}"),
            );
            b.link(&id, "SubGeographicalRegion.Region", &region);
            id
        });
    }

    let mut base_voltage: BTreeMap<String, String> = BTreeMap::new();
    for &v in &kv {
        let text = kv_text(v);
        base_voltage.entry(text.clone()).or_insert_with(|| {
            let id = b.add("BaseVoltage", &format!("bv:{text}"), &format!("{text} kV"));
            b.num(&id, "BaseVoltage.nominalVoltage", v);
            id
        });
    }
    let bv = |i: usize| base_voltage[&kv_text(kv[i])].clone();

    let mut substation: HashMap<usize, String> = HashMap::new();
    for i in (0..n).filter(|&i| group[i] == i) {
        let name = if case.bus_name.is_some() {
            station_name(&names[i]).to_string()
        } else {
            format!("Substation {}", numbers[i])
        };
        let id = b.add("Substation", &format!("substation:{}", numbers[i]), &name);
        b.link(
            &id,
            "Substation.Region",
            &areas[&(case.bus[i][BUS_AREA] as i64)],
        );
        substation.insert(i, id);
    }

    // One voltage level per substation and base voltage, its limits the widest
    // its buses give.
    let mut limits: HashMap<(usize, String), (f64, f64)> = HashMap::new();
    for i in 0..n {
        let (vmax, vmin) = (col(&case.bus[i], VMAX, 0.0), col(&case.bus[i], VMIN, 0.0));
        let (high, low) = limits
            .entry((group[i], kv_text(kv[i])))
            .or_insert((0.0, f64::INFINITY));
        if vmax > 0.0 {
            *high = high.max(vmax * kv[i]);
        }
        if vmin > 0.0 {
            *low = low.min(vmin * kv[i]);
        }
    }
    let mut level: HashMap<(usize, String), String> = HashMap::new();
    for i in 0..n {
        let key = (group[i], kv_text(kv[i]));
        if level.contains_key(&key) {
            continue;
        }
        let sub_name = if case.bus_name.is_some() {
            station_name(&names[group[i]]).to_string()
        } else {
            format!("Substation {}", numbers[group[i]])
        };
        let id = b.add(
            "VoltageLevel",
            &format!("level:{}:{}", numbers[group[i]], key.1),
            &format!("{sub_name} {} kV", key.1),
        );
        b.link(&id, "VoltageLevel.Substation", &substation[&group[i]]);
        b.link(&id, "VoltageLevel.BaseVoltage", &bv(i));
        let (high, low) = limits[&key];
        if high > 0.0 {
            b.num(&id, "VoltageLevel.highVoltageLimit", high);
        }
        if low.is_finite() {
            b.num(&id, "VoltageLevel.lowVoltageLimit", low);
        }
        level.insert(key, id);
    }
    let vl = |i: usize| level[&(group[i], kv_text(kv[i]))].clone();

    // --- Buses as busbars, nodes and topological nodes ---
    let mut node = Vec::with_capacity(n); // ConnectivityNode
    let mut tnode = Vec::with_capacity(n); // TopologicalNode
    let mut busbar_terminal = Vec::with_capacity(n);
    for i in 0..n {
        let key = format!("bus:{}", numbers[i]);
        let cn = b.add("ConnectivityNode", &format!("{key}:node"), &names[i]);
        b.link(&cn, "ConnectivityNode.ConnectivityNodeContainer", &vl(i));
        let tn = b.add("TopologicalNode", &format!("{key}:tnode"), &names[i]);
        b.link(&tn, "TopologicalNode.BaseVoltage", &bv(i));
        b.link(&tn, "TopologicalNode.ConnectivityNodeContainer", &vl(i));
        b.link(&cn, "ConnectivityNode.TopologicalNode", &tn);
        let bbs = b.add("BusbarSection", &key, &names[i]);
        b.link(&bbs, "Equipment.EquipmentContainer", &vl(i));
        b.flag(&bbs, "Equipment.inService", true);
        let t = terminal(&mut b, &bbs, &key, 1, &cn, &tn, true);
        node.push(cn);
        tnode.push(tn);
        busbar_terminal.push(t);
    }

    let mut sv = SvState::default();
    for i in 0..n {
        sv.voltage
            .push((tnode[i].clone(), case.bus[i][VM] * kv[i], case.bus[i][VA]));
    }

    // --- Branches ---
    let patl = limit_type(&mut b, "PATL", None);
    let (rate_b, rate_c) = (
        case.branch
            .iter()
            .any(|r| col(r, RATE_B, 0.0) > col(r, RATE_A, 0.0)),
        case.branch
            .iter()
            .any(|r| col(r, RATE_C, 0.0) > col(r, RATE_B, 0.0).max(col(r, RATE_A, 0.0))),
    );
    let tatl_b = rate_b.then(|| limit_type(&mut b, "TATL 1200", Some(1200.0)));
    let tatl_c = rate_c.then(|| limit_type(&mut b, "TATL 60", Some(60.0)));
    if rate_b || rate_c {
        notes.push(
            "rateB and rateC written as temporary limits of 20 and 1 minutes; \
             MATPOWER gives no durations"
                .to_string(),
        );
    }
    let has_flows = case.branch.iter().all(|r| r.len() > QT);
    let mut inside: BTreeMap<usize, usize> = BTreeMap::new();

    let (mut compensators, mut equivalent_branches) = (0usize, 0usize);
    let (mut charged, mut odd_impedance) = (0usize, 0usize);
    for br in &branches {
        let row = &case.branch[br.row];
        let (r, x, bch) = (row[BR_R], row[BR_X], row[BR_B]);
        let tap = if row[TAP] == 0.0 { 1.0 } else { row[TAP] };
        // A transformer's end 1 has the highest ratedU, and carries its
        // impedance. That is MATPOWER's to side unless the from side's
        // (tap·baseKV) is higher, when the transformer is built mirrored.
        let mirrored = br.transformer && tap * kv[br.from] > kv[br.to];
        // CGMES has no negative impedances on lines: a negative reactance is a
        // series capacitor, a negative resistance an equivalent.
        let class = if br.transformer {
            "PowerTransformer"
        } else if bch == 0.0 && r < 0.0 {
            equivalent_branches += 1;
            "EquivalentBranch"
        } else if bch == 0.0 && x < 0.0 {
            compensators += 1;
            "SeriesCompensator"
        } else {
            "ACLineSegment"
        };
        let eq = b.add(class, &br.key, &br.name);
        // MATPOWER's from side, then its to side.
        let sides = [(br.from, 1usize), (br.to, 2usize)];
        let mut terminals = Vec::new();
        for (bus, side) in sides {
            let seq = if br.transformer && !mirrored {
                3 - side
            } else {
                side
            };
            let (cn, tn) = if options.breakers {
                branch_end(
                    &mut b,
                    br,
                    bus,
                    side,
                    &numbers,
                    &node,
                    &tnode,
                    &vl(bus),
                    &bv(bus),
                    &mut sv,
                )
            } else {
                (node[bus].clone(), tnode[bus].clone())
            };
            let connected = options.breakers || !br.out;
            let t = terminal(&mut b, &eq, &br.key, seq, &cn, &tn, connected);
            terminals.push((bus, t));
        }
        b.flag(&eq, "Equipment.inService", !br.out);
        sv.status.push((eq.clone(), !br.out));

        if br.transformer {
            b.link(
                &eq,
                "Equipment.EquipmentContainer",
                &substation[&group[br.from]],
            );
            // MATPOWER's model is an ideal transformer of ratio tap·e^{jθ} on
            // the from side, then the series impedance and the charging. Built
            // as it is, end 1 is the to side, and end 2's ratedU carries the
            // ratio. Mirrored, end 1 is the from side, the ideal transformer
            // moves to the to side, and the impedance it moves across scales
            // by tap² (the charging by 1/tap²).
            let (e1, e2, u1, u2, scale) = if mirrored {
                (0, 1, kv[br.from], kv[br.to] / tap, tap * tap)
            } else {
                (1, 0, kv[br.to], tap * kv[br.from], 1.0)
            };
            let z_base = u1 * u1 / base_mva;
            if bch > 0.0 {
                charged += 1;
            }
            if !positive(x) || r < 0.0 {
                odd_impedance += 1;
            }
            let mut end = |n: usize, at: usize, u: f64| {
                let (bus, t) = &terminals[at];
                let id = b.add(
                    "PowerTransformerEnd",
                    &format!("{}:end{n}", br.key),
                    &format!("{} {n}", br.name),
                );
                b.link(&id, "PowerTransformerEnd.PowerTransformer", &eq);
                b.link(&id, "TransformerEnd.Terminal", t);
                b.link(&id, "TransformerEnd.BaseVoltage", &bv(*bus));
                b.int(&id, "TransformerEnd.endNumber", n as i64);
                b.num(&id, "PowerTransformerEnd.ratedU", u);
                b.value(
                    &id,
                    "PowerTransformerEnd.connectionKind",
                    "WindingConnection.Y",
                );
                b.int(&id, "PowerTransformerEnd.phaseAngleClock", 0);
                id
            };
            let end1 = end(1, e1, u1);
            let end2 = end(2, e2, u2);
            b.num(&end1, "PowerTransformerEnd.r", r * scale * z_base);
            b.num(&end1, "PowerTransformerEnd.x", x * scale * z_base);
            b.num(&end1, "PowerTransformerEnd.b", bch / scale / z_base);
            b.num(&end1, "PowerTransformerEnd.g", 0.0);
            for attr in ["r", "x", "b", "g"] {
                b.num(&end2, &format!("PowerTransformerEnd.{attr}"), 0.0);
            }
            if row[SHIFT] != 0.0 {
                // On the end the ideal transformer is at: end 2 as built, end 1
                // mirrored.
                let (on, neutral_u, x_on) = if mirrored {
                    (&end1, u1, x * scale * z_base)
                } else {
                    (&end2, u2, 0.0)
                };
                let ptc = b.add(
                    "PhaseTapChangerLinear",
                    &format!("{}:shift", br.key),
                    &br.name,
                );
                b.link(&ptc, "PhaseTapChanger.TransformerEnd", on);
                // One step either way of |shift|, set at the case's: a range
                // whose largest angle is the shift's own, which a reader
                // scaling the reactance by the angle (as gridoxide does
                // between xMin and xMax) then leaves at its end's.
                let step = row[SHIFT].signum() as i64;
                b.int(&ptc, "TapChanger.lowStep", -1);
                b.int(&ptc, "TapChanger.highStep", 1);
                b.int(&ptc, "TapChanger.neutralStep", 0);
                b.int(&ptc, "TapChanger.normalStep", step);
                b.num(&ptc, "TapChanger.neutralU", neutral_u);
                b.flag(&ptc, "TapChanger.ltcFlag", false);
                b.num(
                    &ptc,
                    "PhaseTapChangerLinear.stepPhaseShiftIncrement",
                    row[SHIFT].abs(),
                );
                // Its end's reactance at every step: the shift moves none.
                b.num(&ptc, "PhaseTapChangerLinear.xMin", x_on);
                b.num(&ptc, "PhaseTapChangerLinear.xMax", x_on);
                b.num(&ptc, "TapChanger.step", step as f64);
                b.flag(&ptc, "TapChanger.controlEnabled", false);
                sv.taps.push((ptc, step));
            }
        } else {
            let z_base = kv[br.from] * kv[br.from] / base_mva;
            b.link(&eq, "ConductingEquipment.BaseVoltage", &bv(br.from));
            match class {
                "EquivalentBranch" => {
                    b.num(&eq, "EquivalentBranch.r", r * z_base);
                    b.num(&eq, "EquivalentBranch.x", x * z_base);
                }
                "SeriesCompensator" => {
                    b.num(&eq, "SeriesCompensator.r", r * z_base);
                    b.num(&eq, "SeriesCompensator.x", x * z_base);
                }
                _ => {
                    b.num(&eq, "ACLineSegment.r", r * z_base);
                    b.num(&eq, "ACLineSegment.x", x * z_base);
                    b.num(&eq, "ACLineSegment.bch", bch / z_base);
                    b.num(&eq, "ACLineSegment.gch", 0.0);
                    b.num(&eq, "Conductor.length", 0.0);
                }
            }
            let line = b.add("Line", &format!("{}:line", br.key), &br.name);
            b.link(
                &line,
                "Line.Region",
                &areas[&(case.bus[br.from][BUS_AREA] as i64)],
            );
            b.link(&eq, "Equipment.EquipmentContainer", &line);
            if group[br.from] == group[br.to] {
                *inside.entry(group[br.from]).or_default() += 1;
            }
        }

        // Limits at both ends, in amperes at each end's base voltage.
        for (k, (bus, t)) in terminals.iter().enumerate() {
            let rates = [
                (RATE_A, Some(&patl), "PATL"),
                (RATE_B, tatl_b.as_ref(), "TATL 1200"),
                (RATE_C, tatl_c.as_ref(), "TATL 60"),
            ];
            let mut set: Option<String> = None;
            let mut below = 0.0f64;
            for (column, kind, label) in rates {
                let (Some(kind), mva) = (kind, col(row, column, 0.0)) else {
                    continue;
                };
                if !positive(mva) || mva <= below {
                    continue;
                }
                below = mva;
                let set = set.get_or_insert_with(|| {
                    let id = b.add(
                        "OperationalLimitSet",
                        &format!("{}:limits{}", br.key, k + 1),
                        &format!("{} limits", br.name),
                    );
                    b.link(&id, "OperationalLimitSet.Terminal", t);
                    id
                });
                let set = set.clone();
                let amps = mva * 1e3 / (3f64.sqrt() * kv[*bus]);
                let limit = b.add(
                    "CurrentLimit",
                    &format!("{}:limits{}:{label}", br.key, k + 1),
                    label,
                );
                b.link(&limit, "OperationalLimit.OperationalLimitSet", &set);
                b.link(&limit, "OperationalLimit.OperationalLimitType", kind);
                b.num(&limit, "CurrentLimit.normalValue", amps);
                b.num(&limit, "CurrentLimit.value", amps);
            }
        }

        if has_flows {
            let on = !br.out;
            let (pf, qf, pt, qt) = (row[PF], row[QF], row[PT], row[QT]);
            let at = |v: f64| if on { v } else { 0.0 };
            // A transformer's terminals were made to side first.
            let (t_from, t_to) = (&terminals[0].1, &terminals[1].1);
            sv.flow.push((t_from.clone(), at(pf), at(qf)));
            sv.flow.push((t_to.clone(), at(pt), at(qt)));
        }
    }
    if compensators > 0 {
        notes.push(format!(
            "{compensators} branch(es) with a negative reactance written as SeriesCompensators"
        ));
    }
    if equivalent_branches > 0 {
        notes.push(format!(
            "{equivalent_branches} branch(es) with a negative resistance written as \
             EquivalentBranches"
        ));
    }
    if charged > 0 {
        notes.push(format!(
            "{charged} transformer(s) with a positive b (MATPOWER's charging) kept as the case \
             gives it, though CGMES expects a transformer's b to be inductive"
        ));
    }
    if odd_impedance > 0 {
        notes.push(format!(
            "{odd_impedance} transformer(s) with a negative resistance or a reactance not above \
             zero kept as the case gives them"
        ));
    }
    for (root, count) in &inside {
        notes.push(format!(
            "{count} line(s) inside substation {}",
            if case.bus_name.is_some() {
                station_name(&names[*root]).to_string()
            } else {
                format!("Substation {}", numbers[*root])
            }
        ));
    }

    // --- Generators ---
    let slack: Vec<bool> = kinds.iter().map(|k| *k == Kind::Slack).collect();
    let mut control: HashMap<usize, String> = HashMap::new();
    for i in (0..n).filter(|&i| kinds[i] != Kind::Pq) {
        let id = b.add(
            "RegulatingControl",
            &format!("bus:{}:control", numbers[i]),
            &format!("{} voltage", names[i]),
        );
        b.value(
            &id,
            "RegulatingControl.mode",
            "RegulatingControlModeKind.voltage",
        );
        b.link(&id, "RegulatingControl.Terminal", &busbar_terminal[i]);
        b.flag(&id, "RegulatingControl.enabled", true);
        b.flag(&id, "RegulatingControl.discrete", false);
        b.num(
            &id,
            "RegulatingControl.targetValue",
            vg[i].expect("a regulated bus has an online generator") * kv[i],
        );
        b.value(
            &id,
            "RegulatingControl.targetValueUnitMultiplier",
            "UnitMultiplier.k",
        );
        control.insert(i, id);
    }
    let mut raised = 0usize;
    for (k, g) in case.gens.iter().enumerate() {
        let at = gen_bus[k];
        let key = format!("gen:{}", k + 1);
        let fuel = case
            .genfuel
            .as_ref()
            .map(|f| f[k].trim().to_ascii_lowercase());
        let name = format!("{} G{}", names[at], k + 1);
        let unit_class = match fuel.as_deref() {
            Some("hydro") => "HydroGeneratingUnit",
            Some("wind") => "WindGeneratingUnit",
            Some("solar") => "SolarGeneratingUnit",
            Some("nuclear") => "NuclearGeneratingUnit",
            _ => "ThermalGeneratingUnit",
        };
        let unit = b.add(unit_class, &format!("{key}:unit"), &name);
        b.link(
            &unit,
            "Equipment.EquipmentContainer",
            &substation[&group[at]],
        );
        b.flag(&unit, "Equipment.inService", g[GEN_STATUS] > 0.0);
        match unit_class {
            "HydroGeneratingUnit" => b.value(
                &unit,
                "HydroGeneratingUnit.energyConversionCapability",
                "HydroEnergyConversionKind.generator",
            ),
            "WindGeneratingUnit" => b.value(
                &unit,
                "WindGeneratingUnit.windGenUnitType",
                "WindGenUnitKind.onshore",
            ),
            _ => {}
        }
        let (pmax, pmin) = (col(g, PMAX, g[PG].max(0.0)), col(g, PMIN, 0.0));
        let pmin = pmin.min(pmax);
        // What the machine is, by the range of its output, as CGMES ties
        // the two together; what it does now, by its output.
        let (kind, mode) = match (pmin < 0.0, pmax > 0.0) {
            (false, true) => ("generator", "generator"),
            (true, true) if g[PG] < 0.0 => ("generatorOrMotor", "motor"),
            (true, true) => ("generatorOrMotor", "generator"),
            (true, false) => ("motor", "motor"),
            (false, false) => ("condenser", "condenser"),
        };
        b.num(&unit, "GeneratingUnit.maxOperatingP", pmax);
        b.num(&unit, "GeneratingUnit.minOperatingP", pmin);
        // MATPOWER's slack is one bus: its machines take the whole imbalance.
        b.num(
            &unit,
            "GeneratingUnit.normalPF",
            if gen_online[k] && slack[at] { 1.0 } else { 0.0 },
        );
        if let Some(fuel) = &fuel {
            b.text(&unit, "IdentifiedObject.description", fuel);
        }

        let sm = b.add("SynchronousMachine", &key, &name);
        b.link(&sm, "Equipment.EquipmentContainer", &vl(at));
        b.link(&sm, "RotatingMachine.GeneratingUnit", &unit);
        let mbase = col(g, MBASE, 0.0);
        // At least the unit's maximum output, which CGMES checks against it.
        let rated = if mbase > 0.0 { mbase } else { base_mva };
        if rated < pmax {
            raised += 1;
        }
        b.num(&sm, "RotatingMachine.ratedS", rated.max(pmax));
        b.num(&sm, "RotatingMachine.ratedU", kv[at]);
        b.num(&sm, "SynchronousMachine.maxQ", g[QMAX]);
        b.num(&sm, "SynchronousMachine.minQ", g[QMIN]);
        b.value(
            &sm,
            "SynchronousMachine.type",
            &format!("SynchronousMachineKind.{kind}"),
        );
        b.value(
            &sm,
            "SynchronousMachine.operatingMode",
            &format!("SynchronousMachineOperatingMode.{mode}"),
        );
        // Load sign: a machine producing has negative p and q.
        b.num(&sm, "RotatingMachine.p", -g[PG]);
        b.num(&sm, "RotatingMachine.q", -g[QG]);
        b.flag(&sm, "Equipment.inService", g[GEN_STATUS] > 0.0);
        let regulating = gen_online[k] && control.contains_key(&at);
        if let Some(rc) = control.get(&at) {
            b.link(&sm, "RegulatingCondEq.RegulatingControl", rc);
        }
        b.flag(&sm, "RegulatingCondEq.controlEnabled", regulating);
        b.int(
            &sm,
            "SynchronousMachine.referencePriority",
            i64::from(gen_online[k] && slack[at]),
        );
        let t = terminal(&mut b, &sm, &key, 1, &node[at], &tnode[at], gen_online[k]);
        sv.status.push((sm.clone(), gen_online[k]));
        let on = |v: f64| if gen_online[k] { v } else { 0.0 };
        sv.flow.push((t, on(-g[PG]), on(-g[QG])));
    }

    if raised > 0 {
        notes.push(format!(
            "{raised} generator(s) with an mBase below their Pmax rated at Pmax"
        ));
    }

    // --- Loads and shunts ---
    let mut injections = 0usize;
    for i in 0..n {
        let bus = &case.bus[i];
        let on = !isolated[i];
        let vm = bus[VM];
        if bus[PD] != 0.0 || bus[QD] != 0.0 {
            let key = format!("load:{}", numbers[i]);
            // CGMES has no negative loads: an EquivalentInjection stands in.
            let id = if bus[PD] < 0.0 || bus[QD] < 0.0 {
                injections += 1;
                let id = b.add("EquivalentInjection", &key, &format!("{} load", names[i]));
                b.num(&id, "EquivalentInjection.p", bus[PD]);
                b.num(&id, "EquivalentInjection.q", bus[QD]);
                b.flag(&id, "EquivalentInjection.regulationCapability", false);
                b.flag(&id, "EquivalentInjection.regulationStatus", false);
                id
            } else {
                let id = b.add("EnergyConsumer", &key, &format!("{} load", names[i]));
                b.num(&id, "EnergyConsumer.p", bus[PD]);
                b.num(&id, "EnergyConsumer.q", bus[QD]);
                id
            };
            b.link(&id, "Equipment.EquipmentContainer", &vl(i));
            b.flag(&id, "Equipment.inService", true);
            let t = terminal(&mut b, &id, &key, 1, &node[i], &tnode[i], on);
            sv.status.push((id, on));
            sv.flow.push((
                t,
                if on { bus[PD] } else { 0.0 },
                if on { bus[QD] } else { 0.0 },
            ));
        }
        if bus[GS] != 0.0 || bus[BS] != 0.0 {
            let key = format!("shunt:{}", numbers[i]);
            let id = b.add(
                "LinearShuntCompensator",
                &key,
                &format!("{} shunt", names[i]),
            );
            b.link(&id, "Equipment.EquipmentContainer", &vl(i));
            b.flag(&id, "Equipment.inService", true);
            let v2 = kv[i] * kv[i];
            b.num(&id, "LinearShuntCompensator.bPerSection", bus[BS] / v2);
            b.num(&id, "LinearShuntCompensator.gPerSection", bus[GS] / v2);
            b.int(&id, "ShuntCompensator.maximumSections", 1);
            b.int(&id, "ShuntCompensator.normalSections", 1);
            b.num(&id, "ShuntCompensator.nomU", kv[i]);
            b.num(&id, "ShuntCompensator.sections", 1.0);
            b.flag(&id, "RegulatingCondEq.controlEnabled", false);
            let t = terminal(&mut b, &id, &key, 1, &node[i], &tnode[i], on);
            sv.status.push((id.clone(), on));
            sv.sections.push(id);
            // Consumed at the case's voltage: Gs·Vm² and, a capacitor
            // producing, −Bs·Vm².
            let (p, q) = if on {
                (bus[GS] * vm * vm, -bus[BS] * vm * vm)
            } else {
                (0.0, 0.0)
            };
            sv.flow.push((t, p, q));
        }
    }

    if injections > 0 {
        notes.push(format!(
            "{injections} load(s) with a negative P or Q written as EquivalentInjections, \
             as CGMES has no negative loads"
        ));
    }

    // --- Islands and their angle references ---
    let mut islands = UnionFind::new(n);
    for br in branches.iter().filter(|b| !b.out) {
        islands.union(br.from, br.to);
    }
    let mut members: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for i in (0..n).filter(|&i| !isolated[i]) {
        members.entry(islands.find(i)).or_default().push(i);
    }
    let mut unreferenced = 0;
    let mut extra_slack = 0;
    for (root, buses) in &members {
        let id = b.add(
            "TopologicalIsland",
            &format!("island:{}", numbers[*root]),
            &format!("Island {}", numbers[*root]),
        );
        for &i in buses {
            b.link(&id, "TopologicalIsland.TopologicalNodes", &tnode[i]);
        }
        let mut refs = buses.iter().filter(|&&i| slack[i]);
        match refs.next() {
            Some(&r) => b.link(&id, "TopologicalIsland.AngleRefTopologicalNode", &tnode[r]),
            None => unreferenced += 1,
        }
        extra_slack += refs.count();
    }
    if extra_slack > 0 {
        notes.push(format!(
            "{extra_slack} reference bus(es) share an island with another; \
             the island's angle reference is its first"
        ));
    }
    if unreferenced > 0 {
        notes.push(format!(
            "{unreferenced} island(s) without a reference bus: no angle reference"
        ));
    }

    // --- SV ---
    let flat = case.bus.iter().all(|b| b[VM] == 1.0 && b[VA] == 0.0);
    if flat {
        notes.push("the SV is a flat start (every Vm 1, every Va 0), not a solved state".into());
    }
    sv.write(&mut b);

    for field in &case.unmapped {
        notes.push(format!("mpc.{field} not converted"));
    }

    let header = Header {
        time: options
            .time
            .clone()
            .unwrap_or_else(|| "1970-01-01T00:00:00Z".to_string()),
        description: format!("Converted from the MATPOWER case {seed} by cimoxide-import"),
        modeling_authority_set: "urn:cimoxide:import:matpower".to_string(),
    };
    let files = b
        .finish(stem, &header)
        .map_err(|e| MatpowerError::Unsupported(format!("writing the model: {e}")))?;
    Ok(Converted { files, notes })
}

/// A terminal of `equipment`, at node `cn` and topological node `tn`.
fn terminal(
    b: &mut Builder,
    equipment: &str,
    key: &str,
    seq: usize,
    cn: &str,
    tn: &str,
    connected: bool,
) -> String {
    let name = format!(
        "{} T{seq}",
        b.ds.entries[equipment]
            .get_str("IdentifiedObject.name")
            .unwrap_or_default()
    );
    let t = b.add("Terminal", &format!("{key}:t{seq}"), &name);
    b.link(&t, "Terminal.ConductingEquipment", equipment);
    b.link(&t, "Terminal.ConnectivityNode", cn);
    b.link(&t, "Terminal.TopologicalNode", tn);
    b.int(&t, "ACDCTerminal.sequenceNumber", seq as i64);
    b.value(&t, "Terminal.phases", "PhaseCode.ABC");
    b.flag(&t, "ACDCTerminal.connected", connected);
    t
}

/// The breaker at one end of a branch, between the bus's node and a node of
/// the branch's own, and that node with its topological node: the bus's while
/// the breaker is closed, one of its own while it is open.
#[allow(clippy::too_many_arguments)]
fn branch_end(
    b: &mut Builder,
    br: &Branch,
    bus: usize,
    side: usize,
    numbers: &[u64],
    node: &[String],
    tnode: &[String],
    vl: &str,
    bv: &str,
    sv: &mut SvState,
) -> (String, String) {
    let key = format!("{}:breaker{side}", br.key);
    let breaker = b.add("Breaker", &key, &format!("{} CB {}", br.name, numbers[bus]));
    b.link(&breaker, "Equipment.EquipmentContainer", vl);
    b.flag(&breaker, "Equipment.inService", true);
    sv.switches.push((breaker.clone(), br.out));
    b.flag(&breaker, "Switch.normalOpen", br.out);
    b.flag(&breaker, "Switch.open", br.out);
    b.flag(&breaker, "Switch.locked", false);
    b.flag(&breaker, "Switch.retained", false);
    let cn = b.add(
        "ConnectivityNode",
        &format!("{}:node{side}", br.key),
        &format!("{} end {}", br.name, numbers[bus]),
    );
    b.link(&cn, "ConnectivityNode.ConnectivityNodeContainer", vl);
    let tn = if br.out {
        let tn = b.add(
            "TopologicalNode",
            &format!("{}:tnode{side}", br.key),
            &format!("{} end {}", br.name, numbers[bus]),
        );
        b.link(&tn, "TopologicalNode.BaseVoltage", bv);
        b.link(&tn, "TopologicalNode.ConnectivityNodeContainer", vl);
        tn
    } else {
        tnode[bus].clone()
    };
    b.link(&cn, "ConnectivityNode.TopologicalNode", &tn);
    terminal(b, &breaker, &key, 1, &node[bus], &tnode[bus], true);
    terminal(b, &breaker, &key, 2, &cn, &tn, true);
    (cn, tn)
}

/// What the SV holds, gathered while the equipment is built.
#[derive(Default)]
struct SvState {
    /// Topological node, kV, degrees.
    voltage: Vec<(String, f64, f64)>,
    /// Terminal, MW, MVAr, load sign.
    flow: Vec<(String, f64, f64)>,
    /// Conducting equipment, in service.
    status: Vec<(String, bool)>,
    /// Phase tap changers, at their step.
    taps: Vec<(String, i64)>,
    /// Shunt compensators, each with its one section.
    sections: Vec<String>,
    /// Switches, open.
    switches: Vec<(String, bool)>,
}

impl SvState {
    fn write(self, b: &mut Builder) {
        for (tn, v, angle) in self.voltage {
            let id = b.add("SvVoltage", &format!("sv voltage:{tn}"), "");
            b.link(&id, "SvVoltage.TopologicalNode", &tn);
            b.num(&id, "SvVoltage.v", v);
            b.num(&id, "SvVoltage.angle", angle);
        }
        for (t, p, q) in self.flow {
            let id = b.add("SvPowerFlow", &format!("sv flow:{t}"), "");
            b.link(&id, "SvPowerFlow.Terminal", &t);
            b.num(&id, "SvPowerFlow.p", p);
            b.num(&id, "SvPowerFlow.q", q);
        }
        for (eq, on) in self.status {
            let id = b.add("SvStatus", &format!("sv status:{eq}"), "");
            b.link(&id, "SvStatus.ConductingEquipment", &eq);
            b.flag(&id, "SvStatus.inService", on);
        }
        for (tc, step) in self.taps {
            let id = b.add("SvTapStep", &format!("sv tap:{tc}"), "");
            b.link(&id, "SvTapStep.TapChanger", &tc);
            b.num(&id, "SvTapStep.position", step as f64);
        }
        for (sw, open) in self.switches {
            let id = b.add("SvSwitch", &format!("sv switch:{sw}"), "");
            b.link(&id, "SvSwitch.Switch", &sw);
            b.flag(&id, "SvSwitch.open", open);
        }
        for sc in self.sections {
            let id = b.add(
                "SvShuntCompensatorSections",
                &format!("sv sections:{sc}"),
                "",
            );
            b.link(&id, "SvShuntCompensatorSections.ShuntCompensator", &sc);
            b.num(&id, "SvShuntCompensatorSections.sections", 1.0);
        }
    }
}

/// An operational limit type: the permanent limit, or a temporary one of
/// `duration` seconds.
fn limit_type(b: &mut Builder, name: &str, duration: Option<f64>) -> String {
    let id = b.add("OperationalLimitType", &format!("limit type:{name}"), name);
    match duration {
        None => {
            b.value(&id, "OperationalLimitType.kind", "LimitKind.patl");
            b.flag(&id, "OperationalLimitType.isInfiniteDuration", true);
        }
        Some(seconds) => {
            b.value(&id, "OperationalLimitType.kind", "LimitKind.tatl");
            b.flag(&id, "OperationalLimitType.isInfiniteDuration", false);
            b.num(&id, "OperationalLimitType.acceptableDuration", seconds);
        }
    }
    b.value(
        &id,
        "OperationalLimitType.direction",
        "OperationalLimitDirectionKind.absoluteValue",
    );
    id
}
