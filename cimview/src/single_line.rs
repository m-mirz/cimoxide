//! A substation's automatic layout, drawn as a single-line diagram is: busbars
//! as horizontal bars, the bays hanging from them as vertical columns, and
//! every line horizontal or vertical.
//!
//! The substation's [`Inside`] is a graph of equipment and the connectivity
//! nodes (junctions) their terminals meet at. Some junctions are buses: those a
//! busbar section stands on; in a voltage level without busbar sections, its
//! junctions of four terminals or more, else of most; in one without switches
//! (bus-branch), every junction. Without the buses the graph falls apart into bays: a feeder's
//! switches and the line, load or machine they lead to, a coupler between two
//! buses, or a transformer's column between two voltage levels.
//!
//! Voltage levels are stacked by voltage, highest on top. In each, its buses
//! take a row each, or share one where no feeder joins them (bus sections, as
//! opposed to a double busbar). A bay hangs below its buses, a row for each
//! step away from them; a transformer's column hangs from its upper voltage
//! level and rises from its lower one, meeting at the transformer. Bays are
//! placed left to right: a voltage level's feeders, then the transformers down
//! from it, then the next level's feeders, so that a level's bays stay
//! together and its busbar spans only them.
//!
//! Each bay is a tree from its buses, drawn with its spine straight: a branch
//! takes a new column to the right. A link bends, if at all, at the junction's
//! row in the equipment's column, so every route is horizontal and vertical:
//! a junction is a stretch of wire across its row, equipment sits on a column.
//! Columns are as wide as their labels, which are drawn right of their items.
//!
//! Positions are in pixels (`unit` 1): the view draws them as they are,
//! without scaling them to fit.

use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};

use crate::grid::{Inside, Layout};

/// Between rows.
const ROW: f64 = 28.0;
/// A symbol's width, and its column's least.
const SYMBOL: f64 = 20.0;
/// Between columns, after a label.
const GAP: f64 = 12.0;
/// A label's width per character: equipment's, then a junction's (smaller).
const CHAR: (f64, f64) = (4.2, 3.6);
/// Labels longer than this many characters may overlap the next column.
const MAX_CHARS: usize = 18;
/// A busbar's thickness.
const BAR: f64 = 4.0;

/// Whether equipment of `class` is a switch: it joins a bay's junctions.
fn is_switch(class: &str) -> bool {
    class.ends_with("Switch")
        || class.contains("Breaker")
        || class.contains("Disconnector")
        || matches!(class, "Fuse" | "Jumper" | "Recloser" | "Sectionaliser")
}

/// The substation's single-line diagram: positions by item, routes by link
/// (from the item's end to the junction's), and the size of each busbar.
pub fn single_line(inside: &Inside) -> Layout {
    let items = &inside.items;
    let n = items.len();
    let mut layout = Layout { id: "auto".into(), name: "Single-line".into(), kind: "diagram", unit: Some(1.0), ..Default::default() };
    if n == 0 {
        return layout;
    }
    let mut adj: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n];
    for (k, l) in inside.links.iter().enumerate() {
        adj[l.item].push((l.junction, k));
        adj[l.junction].push((l.item, k));
    }

    // Voltage levels by voltage, highest first; what is in none, last.
    let none = inside.groups.len();
    let kv = |g: usize| inside.groups.get(g).and_then(|x| x.kv);
    let mut order: Vec<usize> = (0..=none).collect();
    order.sort_by(|&a, &b| match (kv(a), kv(b)) {
        (Some(x), Some(y)) => y.total_cmp(&x),
        (x, y) => y.is_some().cmp(&x.is_some()),
    }.then(a.cmp(&b)));
    let mut rank = vec![0; none + 1];
    for (r, &g) in order.iter().enumerate() {
        rank[g] = r;
    }
    let rank_of = |i: usize| rank[items[i].group.unwrap_or(none)];

    let busbar = |i: usize| items[i].class == "BusbarSection";
    let mut bus = vec![false; n];
    for g in 0..=none {
        let js: Vec<usize> = (0..n).filter(|&i| items[i].junction && items[i].group.unwrap_or(none) == g).collect();
        if !js.iter().any(|&j| adj[j].iter().any(|&(e, _)| is_switch(items[e].class))) {
            js.iter().for_each(|&j| bus[j] = true);
            continue;
        }
        let bars: Vec<usize> = js.iter().copied().filter(|&j| adj[j].iter().any(|&(e, _)| busbar(e))).collect();
        if !bars.is_empty() {
            bars.iter().for_each(|&j| bus[j] = true);
        } else if js.iter().any(|&j| adj[j].len() >= 4) {
            // Where four or more meet: a bay's own junctions join three at most
            // (a breaker, a disconnector, an earthing switch).
            js.iter().filter(|&&j| adj[j].len() >= 4).for_each(|&j| bus[j] = true);
        } else if let Some(most) = js.iter().map(|&j| adj[j].len()).max().filter(|&d| d >= 3) {
            js.iter().filter(|&&j| adj[j].len() == most).for_each(|&j| bus[j] = true);
        } else {
            // A level of single bays (a generator's): its bus where it is fed, from a
            // transformer or another level, else where most meet.
            let fed = |j: usize| adj[j].iter().any(|&(e, _)| items[e].group != Some(g));
            let j = js.iter().copied().max_by_key(|&j| (fed(j), adj[j].len(), Reverse(j))).unwrap();
            bus[j] = true;
        }
    }

    // Bays, until each has a bus: one without takes its likeliest junction.
    let (bar_of, bays) = loop {
        let bar_of: Vec<Option<usize>> = (0..n)
            .map(|i| {
                let on: Vec<usize> = adj[i].iter().map(|&(j, _)| j).collect();
                (busbar(i) && !on.is_empty() && on.iter().all(|&j| bus[j])).then(|| on[0])
            })
            .collect();
        let bays = components(&adj, |i| bus[i] || bar_of[i].is_some());
        let mut added = false;
        for members in &bays {
            if members.iter().any(|&i| adj[i].iter().any(|&(o, _)| bus[o])) {
                continue;
            }
            let likeliest = members
                .iter()
                .copied()
                .filter(|&i| items[i].junction)
                .max_by_key(|&j| (adj[j].iter().any(|&(e, _)| busbar(e)), adj[j].len(), Reverse(j)));
            if let Some(j) = likeliest {
                bus[j] = true;
                added = true;
            }
        }
        if !added {
            break (bar_of, bays);
        }
    };

    let touched: Vec<BTreeSet<usize>> =
        bays.iter().map(|m| m.iter().flat_map(|&i| adj[i].iter().map(|&(o, _)| o)).filter(|&o| bus[o]).collect()).collect();

    // A voltage level's buses take a row each where a feeder joins them.
    let buses: Vec<usize> = (0..n).filter(|&i| bus[i]).collect();
    let mut apart: BTreeSet<(usize, usize)> = BTreeSet::new();
    for (members, on) in bays.iter().zip(&touched) {
        if members.iter().all(|&i| items[i].junction || is_switch(items[i].class)) {
            continue;
        }
        for &a in on {
            for &b in on {
                if a < b && rank_of(a) == rank_of(b) {
                    apart.insert((a, b));
                }
            }
        }
    }
    let mut bus_row: HashMap<usize, usize> = HashMap::new();
    for &b in &buses {
        let taken: BTreeSet<usize> = buses
            .iter()
            .filter(|&&o| apart.contains(&(o.min(b), o.max(b))))
            .filter_map(|o| bus_row.get(o).copied())
            .collect();
        bus_row.insert(b, (0..).find(|r| !taken.contains(r)).unwrap());
    }
    let mut by_place = buses.clone();
    by_place.sort_by_key(|&b| (rank_of(b), bus_row[&b], b));
    let place: HashMap<usize, usize> = by_place.iter().enumerate().map(|(p, &b)| (b, p)).collect();

    // Bays left to right: a level's feeders, then its transformers down to the
    // lowest level first; bays on no bus last.
    let mut placed: Vec<usize> = (0..bays.len()).filter(|&c| !touched[c].is_empty()).collect();
    // The levels a bay spans: its buses', and its items' own (a transformer's
    // side may lead out of the substation, not to a bus of its level).
    let span = |c: usize| {
        let ranks: Vec<usize> = touched[c]
            .iter()
            .map(|&b| rank_of(b))
            .chain(bays[c].iter().filter_map(|&i| items[i].group.map(|g| rank[g])))
            .collect();
        (*ranks.iter().min().unwrap(), *ranks.iter().max().unwrap())
    };
    placed.sort_by_key(|&c| {
        let (top, bottom) = span(c);
        let places: Vec<usize> = touched[c].iter().map(|b| place[b]).collect();
        let first = *places.iter().min().unwrap();
        let last = *places.iter().max().unwrap();
        (top, top != bottom, if top == bottom { first } else { usize::MAX - bottom }, last, bays[c][0])
    });

    // Each bay's columns, and each item's row from its buses.
    let mut column: Vec<usize> = vec![0; n];
    let mut widths: Vec<f64> = Vec::new();
    let mut side: HashMap<usize, (usize, usize, bool)> = HashMap::new();
    let mut above: BTreeMap<usize, usize> = BTreeMap::new();
    let mut below: BTreeMap<usize, usize> = BTreeMap::new();
    let width = |i: usize| {
        let chars = items[i].label.chars().count().min(MAX_CHARS) as f64;
        SYMBOL + GAP + chars * if items[i].junction { CHAR.1 } else { CHAR.0 }
    };
    for &c in &placed {
        let members: BTreeSet<usize> = bays[c].iter().copied().collect();
        let (top, _) = span(c);
        let mut ranks: Vec<usize> = touched[c].iter().map(|&b| rank_of(b)).collect();
        ranks.sort();
        ranks.dedup();
        let first = ranks[0];
        let from = |r: usize| {
            let mut seeds: Vec<usize> = touched[c].iter().copied().filter(|&b| rank_of(b) == r).collect();
            seeds.sort_by_key(|b| place[b]);
            seeds
        };
        // The tree from the top level's buses gives the columns.
        let (dist_top, parent) = reach(&adj, &members, &from(first));
        let mut children: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut roots = Vec::new();
        for &i in &bays[c] {
            match parent.get(&i) {
                Some(&p) if members.contains(&p) => children.entry(p).or_default().push(i),
                _ => roots.push(i),
            }
        }
        roots.sort_by_key(|i| dist_top.get(i).map(|&(_, at)| at).unwrap_or(usize::MAX));
        for kids in children.values_mut() {
            kids.sort_by_key(|i| dist_top[i].1);
        }
        for &r in &roots {
            columns(r, &children, &mut column, &mut widths);
        }
        for &i in &bays[c] {
            let w = &mut widths[column[i]];
            *w = w.max(width(i));
        }
        // Rows: from its own level's buses, else the nearest (a transformer's,
        // a line's), below them on the top level, above them on a lower one.
        // An item of a level whose buses the bay does not reach (equipment
        // with no level takes its neighbours' then) is in that level's rows,
        // nearer them the farther it is from the buses the bay does reach.
        let dists: Vec<(usize, Steps)> =
            ranks.iter().map(|&r| (r, if r == first { dist_top.clone() } else { reach(&adj, &members, &from(r)).0 })).collect();
        let nearest = |i: usize| dists.iter().filter_map(|(r, d)| d.get(&i).map(|&(d, _)| (*r, d))).min_by_key(|&(r, d)| (d, r));
        let mut astray: Vec<(usize, usize, usize)> = Vec::new();
        for &i in &bays[c] {
            let own = items[i].group.map(|g| rank[g]).or_else(|| {
                let theirs: BTreeSet<usize> = adj[i].iter().filter_map(|&(o, _)| items[o].group.map(|g| rank[g])).collect();
                (theirs.len() == 1).then(|| *theirs.first().unwrap()).filter(|r| !ranks.contains(r))
            });
            let (near, d) = nearest(i).unwrap_or((first, 1));
            match own {
                Some(r) if !ranks.contains(&r) => astray.push((i, r, d)),
                _ => {
                    let r = own.unwrap_or(near);
                    let d = dists.iter().find(|(x, _)| *x == r).and_then(|(_, m)| m.get(&i)).map_or(d, |&(d, _)| d);
                    side.insert(i, (r, d, r == top));
                }
            }
        }
        for &(i, r, d) in &astray {
            let most = astray.iter().filter(|a| a.1 == r).map(|a| a.2).max().unwrap();
            side.insert(i, (r, most - d + 1, r < first));
        }
        for &i in &bays[c] {
            let (r, d, down) = side[&i];
            let most = if down { below.entry(r).or_default() } else { above.entry(r).or_default() };
            *most = (*most).max(d);
        }
    }

    // Columns' x, left to right.
    let mut lefts = Vec::with_capacity(widths.len());
    let mut x = 0.0;
    for w in &widths {
        lefts.push(x);
        x += w;
    }
    let right = x;

    // Rows' y, level by level.
    let mut levels: BTreeSet<usize> = buses.iter().map(|&b| rank_of(b)).collect();
    levels.extend(above.keys());
    levels.extend(below.keys());
    let mut top_row: HashMap<usize, usize> = HashMap::new();
    let mut bottom_row: HashMap<usize, usize> = HashMap::new();
    let mut cursor = 0;
    for &r in &levels {
        let rows = buses.iter().filter(|&&b| rank_of(b) == r).map(|b| bus_row[b] + 1).max().unwrap_or(0);
        let t = cursor + above.get(&r).copied().unwrap_or(0);
        top_row.insert(r, t);
        bottom_row.insert(r, t + rows.max(1) - 1);
        cursor = t + rows.max(1) + below.get(&r).copied().unwrap_or(0) + 2;
    }
    let y = |row: usize| row as f64 * ROW;

    let mut at: BTreeMap<usize, (f64, f64)> = BTreeMap::new();
    for (&i, &(r, d, down)) in &side {
        let row = if down { bottom_row[&r] + d } else { top_row[&r] - d };
        at.insert(i, (lefts[column[i]] + SYMBOL / 2.0, y(row)));
    }

    // Bays on no bus, in a row below the rest.
    let mut x = 0.0;
    for (c, members) in bays.iter().enumerate() {
        if !touched[c].is_empty() {
            continue;
        }
        for &i in members {
            at.insert(i, (x + SYMBOL / 2.0, y(cursor)));
            x += width(i);
        }
    }

    // Busbars across their bays, the bus's junction at the right end.
    for &b in &buses {
        let row = top_row[&rank_of(b)] + bus_row[&b];
        let xs: Vec<f64> = adj[b].iter().filter(|&&(e, _)| bar_of[e].is_none()).filter_map(|&(e, _)| at.get(&e).map(|p| p.0)).collect();
        let (lo, hi) = if xs.is_empty() {
            (right, right + 2.0 * SYMBOL)
        } else {
            let lo = xs.iter().copied().fold(f64::INFINITY, f64::min) - SYMBOL / 2.0;
            (lo, xs.iter().copied().fold(f64::NEG_INFINITY, f64::max) + SYMBOL / 2.0)
        };
        let bars: Vec<usize> = adj[b].iter().map(|&(e, _)| e).filter(|&e| bar_of[e] == Some(b)).collect();
        if bars.is_empty() {
            at.insert(b, ((lo + hi) / 2.0, y(row)));
            layout.sizes.insert(b, (hi - lo, BAR));
        } else {
            at.insert(b, (hi, y(row)));
            let piece = (hi - lo) / bars.len() as f64;
            for (k, &e) in bars.iter().enumerate() {
                at.insert(e, (lo + piece * (k as f64 + 0.5), y(row)));
                layout.sizes.insert(e, (piece, BAR));
            }
        }
    }

    // Routes: a bend at the junction's row in the equipment's column; on a
    // bar, the route ends where it meets it.
    for (k, l) in inside.links.iter().enumerate() {
        let (Some(&e), Some(&j)) = (at.get(&l.item), at.get(&l.junction)) else { continue };
        let corner = (e.0, j.1);
        let mut route = vec![e, corner];
        if !(layout.sizes.contains_key(&l.junction) || bar_of[l.item].is_some() && bus[l.junction]) {
            route.push(j);
        }
        route.dedup();
        layout.edges.insert(k, route);
    }
    layout.nodes = at;
    layout
}

/// Connected components of the items not `cut`, through their links; each
/// sorted, in the order of their first item.
fn components(adj: &[Vec<(usize, usize)>], cut: impl Fn(usize) -> bool) -> Vec<Vec<usize>> {
    let mut seen = vec![false; adj.len()];
    let mut out = Vec::new();
    for start in 0..adj.len() {
        if seen[start] || cut(start) {
            continue;
        }
        seen[start] = true;
        let mut members = vec![start];
        let mut queue = VecDeque::from([start]);
        while let Some(i) = queue.pop_front() {
            for &(o, _) in &adj[i] {
                if !seen[o] && !cut(o) {
                    seen[o] = true;
                    members.push(o);
                    queue.push_back(o);
                }
            }
        }
        members.sort();
        out.push(members);
    }
    out
}

/// Item → steps to it, and the order it was reached in.
type Steps = HashMap<usize, (usize, usize)>;

/// Steps from `seeds` (buses, outside `members`) to each of `members`, with
/// the order each was reached in; and the item each was reached from.
fn reach(adj: &[Vec<(usize, usize)>], members: &BTreeSet<usize>, seeds: &[usize]) -> (Steps, HashMap<usize, usize>) {
    let mut dist = HashMap::new();
    let mut parent = HashMap::new();
    let mut queue = VecDeque::new();
    for &s in seeds {
        for &(o, _) in &adj[s] {
            if members.contains(&o) && !dist.contains_key(&o) {
                dist.insert(o, (1, dist.len()));
                parent.insert(o, s);
                queue.push_back(o);
            }
        }
    }
    while let Some(i) = queue.pop_front() {
        let d = dist[&i].0;
        for &(o, _) in &adj[i] {
            if members.contains(&o) && !dist.contains_key(&o) {
                dist.insert(o, (d + 1, dist.len()));
                parent.insert(o, i);
                queue.push_back(o);
            }
        }
    }
    (dist, parent)
}

/// Columns for the tree under `i`: a leaf takes a new one, an item its first
/// child's, so a chain runs straight down.
fn columns(i: usize, children: &HashMap<usize, Vec<usize>>, column: &mut [usize], widths: &mut Vec<f64>) {
    match children.get(&i) {
        Some(kids) if !kids.is_empty() => {
            for &k in kids {
                columns(k, children, column, widths);
            }
            column[i] = column[kids[0]];
        }
        _ => {
            column[i] = widths.len();
            widths.push(SYMBOL);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::single_line;
    use crate::grid::{Grid, Inside};
    use crate::testing::{collect, validate};

    fn grid(case: &str) -> Grid {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../CGMES-Test-Configurations/v3.0").join(case);
        let files = collect(&dir);
        assert!(!files.is_empty(), "missing test data — run `git submodule update --init`");
        validate(&files).grid
    }

    /// Every item placed, every route horizontal and vertical, no two symbols
    /// on one spot, and the same each time.
    fn check(name: &str, inside: &Inside) {
        let l = single_line(inside);
        assert_eq!(l.nodes.len(), inside.items.len(), "{name}: every item placed");
        assert_eq!(l.edges.len(), inside.links.len(), "{name}: every link routed");
        for (k, route) in &l.edges {
            for w in route.windows(2) {
                let (a, b) = (w[0], w[1]);
                assert!(a.0 == b.0 || a.1 == b.1, "{name}: link {} bends off the axes: {route:?}", inside.links[*k].id);
            }
        }
        let mut spots = std::collections::BTreeSet::new();
        for (&i, &(x, y)) in &l.nodes {
            if inside.items[i].junction || l.sizes.contains_key(&i) {
                continue;
            }
            assert!(spots.insert(((x * 10.0) as i64, (y * 10.0) as i64)), "{name}: {} on another's spot", inside.items[i].label);
        }
        assert_eq!(single_line(inside).nodes, l.nodes, "{name}: the same each time");
        // Voltage levels apart: each level's rows below the one above it.
        let mut bands: Vec<(f64, f64, f64)> = (0..inside.groups.len())
            .filter_map(|g| {
                let ys: Vec<f64> = l.nodes.iter().filter(|(i, _)| inside.items[**i].group == Some(g)).map(|(_, p)| p.1).collect();
                (!ys.is_empty()).then(|| {
                    (inside.groups[g].kv.unwrap_or(0.0), ys.iter().copied().fold(f64::MAX, f64::min), ys.iter().copied().fold(f64::MIN, f64::max))
                })
            })
            .collect();
        bands.sort_by(|a, b| b.0.total_cmp(&a.0));
        for w in bands.windows(2) {
            assert!(w[0].2 < w[1].1 || w[0].0 == w[1].0, "{name}: levels overlap: {bands:?}");
        }
    }

    /// MicroGrid Type 1, node-breaker: busbars as bars across their bays,
    /// voltage levels top to bottom by voltage.
    #[test]
    fn node_breaker() {
        let g = grid("MicroGrid/MicroGrid-Type1/MicroGrid-Type1-Merged");
        for (n, inside) in &g.inside {
            check(&g.nodes[*n].label, inside);
        }
        let index = |label: &str| g.nodes.iter().position(|n| n.label == label).unwrap();
        let anvers = &g.inside[&index("Anvers")];
        let l = single_line(anvers);
        // Its three busbar sections are bars; each line hangs below one.
        let bars: Vec<usize> = (0..anvers.items.len()).filter(|&i| anvers.items[i].class == "BusbarSection").collect();
        assert_eq!(bars.len(), 3);
        assert!(bars.iter().all(|b| l.sizes[b].0 > l.sizes[b].1));
        let lowest_bar = bars.iter().map(|b| l.nodes[b].1).fold(f64::MIN, f64::max);
        for (i, it) in anvers.items.iter().enumerate() {
            if it.class == "ACLineSegment" {
                assert!(l.nodes[&i].1 > lowest_bar, "{} below the busbars", it.label);
            }
        }

        // Amsterdam: 400 kV above 220 kV above the lowest.
        let amsterdam = &g.inside[&index("PP_Amsterdam")];
        let l = single_line(amsterdam);
        let level_y = |g: usize| {
            amsterdam.items.iter().enumerate().filter(|(_, it)| it.junction && it.group == Some(g)).map(|(i, _)| l.nodes[&i].1).fold(f64::MAX, f64::min)
        };
        let mut levels: Vec<(f64, f64)> =
            amsterdam.groups.iter().enumerate().map(|(g, gr)| (gr.kv.unwrap_or(0.0), level_y(g))).collect();
        levels.sort_by(|a, b| b.0.total_cmp(&a.0));
        assert!(levels.windows(2).all(|w| w[0].1 < w[1].1), "{levels:?}");
    }

    /// MicroGrid's base case, bus-branch: each topological node a bus.
    #[test]
    fn bus_branch() {
        let g = grid("MicroGrid/MicroGid-BaseCase/MicroGrid-BaseCase-Merged");
        for (n, inside) in &g.inside {
            check(&g.nodes[*n].label, inside);
            let l = single_line(inside);
            // A bar itself, or where a busbar section is drawn as one.
            for (i, it) in inside.items.iter().enumerate() {
                if it.junction {
                    let on_bar = inside.links.iter().any(|k| k.junction == i && l.sizes.contains_key(&k.item));
                    assert!(l.sizes.contains_key(&i) || on_bar, "{}: {} a bar", g.nodes[*n].label, it.label);
                }
            }
        }
    }

    /// Every substation of the larger test grids.
    #[test]
    fn test_grids() {
        for case in ["MiniGrid/MiniGrid-Merged", "SmallGrid/SmallGrid-Merged", "RealGrid/RealGrid-Merged"] {
            let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../CGMES-Test-Configurations/v3.0").join(case);
            if !dir.exists() {
                continue;
            }
            let g = grid(case);
            for (n, inside) in &g.inside {
                check(&format!("{case} {}", g.nodes[*n].label), inside);
            }
        }
    }
}
