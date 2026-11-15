//! The grid's automatic layout as a schematic: lines only horizontal, vertical or at 45°, as
//! transmission overview diagrams draw them.
//!
//! The positions [`crate::layout::force`] finds are snapped to a lattice, the busiest node first,
//! one that would land on another taking the nearest free point. Each line then runs from lattice
//! point to lattice point in at most three straight pieces, each along an axis or a diagonal: of
//! the shapes it can take (straight; a diagonal and a straight piece, either way round; a
//! straight piece, a diagonal and another), the one that passes through no other node, shares no
//! piece with a line already drawn, crosses the fewest, and bends least. The shorter lines go
//! first, so that local connections keep the plainest shapes. Lines joining the same two nodes
//! (double circuits) run side by side, stepped aside where they leave a node.
//!
//! Linear in the lines times their lengths on the lattice, and deterministic.

use std::collections::{BTreeMap, HashSet};

/// The lattice spacing, in the units of the positions it snaps ([`crate::layout::force`]'s, a
/// line's ideal length being one): a line spans about two points.
const STEP: f64 = 0.5;
/// How far apart lines joining the same two nodes run, in those units.
const APART: f64 = 0.08;

/// What a shape costs: passing through another node, sharing a piece with a line drawn, crossing
/// one, and each bend.
const THROUGH_NODE: u32 = 40;
const SHARED: u32 = 12;
const CROSSING: u32 = 3;
const BEND: u32 = 1;

type Point = (i64, i64);

/// Nodes at `pos` (in [`crate::layout::force`]'s units) and their `edges`, as a schematic: each
/// node's position, and each edge's route from its source to its target, both in those units.
/// A loop (an edge from a node to itself) has no route.
pub fn octilinear(pos: &[(f64, f64)], edges: &[(usize, usize)]) -> (Vec<(f64, f64)>, Vec<Vec<(f64, f64)>>) {
    let at = snap(pos, edges);
    let unit = |p: Point| (p.0 as f64 * STEP, p.1 as f64 * STEP);
    let taken: HashSet<Point> = at.iter().copied().collect();

    // The edges by the two nodes they join; the pairs, shortest first.
    let mut pairs: BTreeMap<(usize, usize), Vec<usize>> = BTreeMap::new();
    for (k, &(a, b)) in edges.iter().enumerate() {
        if a != b {
            pairs.entry((a.min(b), a.max(b))).or_default().push(k);
        }
    }
    let mut order: Vec<(usize, usize)> = pairs.keys().copied().collect();
    let span = |&(a, b): &(usize, usize)| {
        let (d0, d1) = ((at[b].0 - at[a].0).abs(), (at[b].1 - at[a].1).abs());
        (d0.max(d1), d0.min(d1))
    };
    order.sort_by_key(|p| (span(p), *p));

    let mut drawn = Drawn::default();
    let mut routes = vec![Vec::new(); edges.len()];
    for pair in order {
        let (a, b) = pair;
        let path = shapes(at[a], at[b])
            .into_iter()
            .map(|path| (drawn.cost(&path, &taken), path))
            .min_by_key(|(cost, _)| *cost)
            .map(|(_, path)| path)
            .expect("a shape at least");
        drawn.add(&path);
        let ks = &pairs[&pair];
        let points: Vec<(f64, f64)> = path.iter().map(|&p| unit(p)).collect();
        // Side by side: each a step aside from the first piece's direction, both ends stepping
        // out of the node and back.
        let (dx, dy) = ((path[1].0 - path[0].0) as f64, (path[1].1 - path[0].1) as f64);
        let length = dx.hypot(dy);
        let across = (-dy / length, dx / length);
        for (i, &k) in ks.iter().enumerate() {
            let route: Vec<(f64, f64)> = if ks.len() == 1 {
                points.clone()
            } else {
                let o = (i as f64 - (ks.len() - 1) as f64 / 2.0) * APART;
                let shifted = points.iter().map(|p| (p.0 + o * across.0, p.1 + o * across.1));
                let (first, last) = (points[0], points[points.len() - 1]);
                std::iter::once(first).chain(shifted).chain(std::iter::once(last)).collect()
            };
            // From the edge's own source to its target.
            routes[k] = if edges[k].0 == a { route } else { route.into_iter().rev().collect() };
        }
    }
    (at.into_iter().map(unit).collect(), routes)
}

/// Each node's lattice point: the nearest free one to its position, the nodes with most edges
/// placed first.
fn snap(pos: &[(f64, f64)], edges: &[(usize, usize)]) -> Vec<Point> {
    let mut degree = vec![0usize; pos.len()];
    for &(a, b) in edges {
        degree[a] += 1;
        degree[b] += 1;
    }
    let mut order: Vec<usize> = (0..pos.len()).collect();
    order.sort_by_key(|&i| (std::cmp::Reverse(degree[i]), i));
    let mut taken: HashSet<Point> = HashSet::new();
    let mut at = vec![(0, 0); pos.len()];
    for i in order {
        let (x, y) = (pos[i].0 / STEP, pos[i].1 / STEP);
        let home = (x.round() as i64, y.round() as i64);
        let mut best = home;
        if taken.contains(&home) {
            // The ring around it, then the next, until one has a free point: its nearest.
            'rings: for r in 1i64.. {
                let mut ring: Vec<Point> = (-r..=r)
                    .flat_map(|i| [(home.0 + i, home.1 - r), (home.0 + i, home.1 + r), (home.0 - r, home.1 + i), (home.0 + r, home.1 + i)])
                    .filter(|p| !taken.contains(p))
                    .collect();
                if ring.is_empty() {
                    continue;
                }
                ring.sort_by(|p, q| {
                    let d = |p: &Point| (p.0 as f64 - x).powi(2) + (p.1 as f64 - y).powi(2);
                    d(p).total_cmp(&d(q)).then(p.cmp(q))
                });
                best = ring[0];
                break 'rings;
            }
        }
        taken.insert(best);
        at[i] = best;
    }
    at
}

/// The shapes a line from `a` to `b` can take, as its lattice points, ends included: straight
/// where it can be; else a diagonal and a straight piece, either way round, and, where the
/// straight part is long enough to halve, a diagonal between its halves.
fn shapes(a: Point, b: Point) -> Vec<Vec<Point>> {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let (ax, ay) = (dx.abs(), dy.abs());
    if ax == 0 || ay == 0 || ax == ay {
        return vec![vec![a, b]];
    }
    let m = ax.min(ay);
    let r = ax.max(ay) - m;
    let diagonal = (dx.signum(), dy.signum());
    let straight = if ax > ay { (dx.signum(), 0) } else { (0, dy.signum()) };
    let go = |p: Point, d: Point, n: i64| (p.0 + d.0 * n, p.1 + d.1 * n);
    let mut out = vec![vec![a, go(a, diagonal, m), b], vec![a, go(a, straight, r), b]];
    if r >= 2 {
        let h = go(a, straight, r / 2);
        out.push(vec![a, h, go(h, diagonal, m), b]);
    }
    out
}

/// The lattice's unit pieces lines are drawn along, and the nodes' points.
#[derive(Default)]
struct Drawn {
    pieces: HashSet<(Point, Point)>,
}

/// A unit piece, the same whichever way it is walked.
fn piece(p: Point, q: Point) -> (Point, Point) {
    if p <= q { (p, q) } else { (q, p) }
}

/// Each unit step along `path`: from where, and which way.
fn steps(path: &[Point]) -> impl Iterator<Item = (Point, Point)> + '_ {
    path.windows(2).flat_map(|w| {
        let (p, q) = (w[0], w[1]);
        let d = ((q.0 - p.0).signum(), (q.1 - p.1).signum());
        let n = (q.0 - p.0).abs().max((q.1 - p.1).abs());
        (0..n).map(move |i| ((p.0 + d.0 * i, p.1 + d.1 * i), d))
    })
}

impl Drawn {
    fn cost(&self, path: &[Point], nodes: &HashSet<Point>) -> u32 {
        let (a, b) = (path[0], path[path.len() - 1]);
        let mut cost = BEND * (path.len() as u32 - 2);
        for (p, d) in steps(path) {
            let q = (p.0 + d.0, p.1 + d.1);
            if q != b && q != a && nodes.contains(&q) {
                cost += THROUGH_NODE;
            }
            if self.pieces.contains(&piece(p, q)) {
                cost += SHARED;
            }
            // A diagonal crosses the other diagonal of its square.
            if d.0 != 0 && d.1 != 0 && self.pieces.contains(&piece((p.0 + d.0, p.1), (p.0, p.1 + d.1))) {
                cost += CROSSING;
            }
        }
        cost
    }

    fn add(&mut self, path: &[Point]) {
        for (p, d) in steps(path) {
            self.pieces.insert(piece(p, (p.0 + d.0, p.1 + d.1)));
        }
    }
}

/// Lines crossing others where the shapes are drawn, counted for the tests: each pair of unit
/// pieces crossing in a square.
#[cfg(test)]
fn crossings(routes: &[Vec<(f64, f64)>]) -> usize {
    let mut drawn: std::collections::HashMap<(Point, Point), usize> = std::collections::HashMap::new();
    let lattice = |p: (f64, f64)| ((p.0 / STEP).round() as i64, (p.1 / STEP).round() as i64);
    let mut count = 0;
    for (k, r) in routes.iter().enumerate() {
        let path: Vec<Point> = r.iter().map(|&p| lattice(p)).collect();
        let mut path = path;
        path.dedup();
        for (p, d) in steps(&path) {
            if d.0 != 0 && d.1 != 0 && drawn.get(&piece((p.0 + d.0, p.1), (p.0, p.1 + d.1))).is_some_and(|&o| o != k) {
                count += 1;
            }
        }
        for (p, d) in steps(&path) {
            drawn.insert(piece(p, (p.0 + d.0, p.1 + d.1)), k);
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Each piece of each route along an axis or a diagonal.
    fn octilinear_pieces(routes: &[Vec<(f64, f64)>]) -> bool {
        routes.iter().all(|r| {
            r.windows(2).all(|w| {
                let (dx, dy) = ((w[1].0 - w[0].0).abs(), (w[1].1 - w[0].1).abs());
                dx < 1e-9 || dy < 1e-9 || (dx - dy).abs() < 1e-9
            })
        })
    }

    #[test]
    fn lines_run_along_axes_and_diagonals() {
        let pos = [(0.0, 0.0), (1.7, 0.6), (0.4, 1.9), (2.2, 2.4), (-1.3, 0.8)];
        let edges = [(0, 1), (0, 2), (1, 3), (2, 3), (0, 4), (4, 2)];
        let (nodes, routes) = octilinear(&pos, &edges);
        assert!(octilinear_pieces(&routes), "{routes:?}");
        for (k, &(a, b)) in edges.iter().enumerate() {
            assert_eq!(routes[k].first(), Some(&nodes[a]), "edge {k} from its source");
            assert_eq!(routes[k].last(), Some(&nodes[b]), "edge {k} to its target");
            assert!(routes[k].len() <= 4, "at most two bends: {:?}", routes[k]);
        }
    }

    #[test]
    fn nodes_keep_apart_and_near_where_they_were() {
        // Three on top of each other: each its own point, the nearest free ones.
        let pos = [(0.0, 0.0), (0.01, 0.0), (0.0, 0.02), (3.0, 3.0)];
        let (nodes, _) = octilinear(&pos, &[(0, 3), (1, 3), (2, 3)]);
        let distinct: HashSet<(i64, i64)> = nodes.iter().map(|p| ((p.0 * 100.0).round() as i64, (p.1 * 100.0).round() as i64)).collect();
        assert_eq!(distinct.len(), 4);
        assert!(nodes.iter().zip(&pos).all(|(p, q)| (p.0 - q.0).abs() <= 2.0 * STEP && (p.1 - q.1).abs() <= 2.0 * STEP));
    }

    #[test]
    fn a_line_goes_round_a_node_in_its_way() {
        // A straight line from 0 to 2 would pass through 1.
        let pos = [(0.0, 0.0), (1.0, 0.0), (2.0, 0.5)];
        let (nodes, routes) = octilinear(&pos, &[(0, 2), (0, 1)]);
        let through = |r: &Vec<(f64, f64)>, p: (f64, f64)| {
            r.windows(2).any(|w| {
                let (a, b) = (w[0], w[1]);
                let cross = (b.0 - a.0) * (p.1 - a.1) - (b.1 - a.1) * (p.0 - a.0);
                let within = (p.0 - a.0) * (p.0 - b.0) <= 1e-9 && (p.1 - a.1) * (p.1 - b.1) <= 1e-9;
                cross.abs() < 1e-9 && within && p != a && p != b
            })
        };
        assert!(!through(&routes[0], nodes[1]), "{:?} through {:?}", routes[0], nodes[1]);
    }

    #[test]
    fn double_circuits_run_side_by_side() {
        let pos = [(0.0, 0.0), (2.0, 1.0)];
        let (nodes, routes) = octilinear(&pos, &[(0, 1), (1, 0), (0, 1)]);
        assert!(octilinear_pieces(&routes), "{routes:?}");
        for r in &routes {
            let ends = [r[0], r[r.len() - 1]];
            assert!(ends.contains(&nodes[0]) && ends.contains(&nodes[1]));
        }
        // Their middles apart.
        let mid = |r: &Vec<(f64, f64)>| r[r.len() / 2];
        let (m0, m2) = (mid(&routes[0]), mid(&routes[2]));
        assert!((m0.0 - m2.0).hypot(m0.1 - m2.1) > APART);
        // The one written the other way round, from its own source.
        assert_eq!(routes[1][0], nodes[1]);
    }

    #[test]
    fn the_same_grid_the_same_schematic() {
        let pos: Vec<(f64, f64)> = (0..60).map(|i| (((i * 37) % 23) as f64 * 0.7, ((i * 53) % 19) as f64 * 0.6)).collect();
        let edges: Vec<(usize, usize)> = (0..59).map(|i| (i, (i * 7 + 3) % 60)).collect();
        assert_eq!(octilinear(&pos, &edges), octilinear(&pos, &edges));
        let (_, routes) = octilinear(&pos, &edges);
        assert!(octilinear_pieces(&routes));
        assert!(crossings(&routes) < edges.len());
    }
}

#[cfg(test)]
mod timing {
    /// RealGrid's size: 4875 nodes, 7752 edges, laid out by force first. `cargo test --release -- --ignored`.
    #[test]
    #[ignore]
    fn realgrid_sized() {
        let n = 4875;
        let mut edges: Vec<(usize, usize)> = (1..n).map(|i| (i, (i * 7919) % i)).collect();
        edges.extend((0..2878).map(|i| ((i * 31) % n, (i * 31 + 17) % n)));
        let pos = crate::layout::force(n, &edges);
        let t = std::time::Instant::now();
        let (_, routes) = super::octilinear(&pos, &edges);
        eprintln!("octilinear: {:?} for {} routes", t.elapsed(), routes.len());
    }
}
