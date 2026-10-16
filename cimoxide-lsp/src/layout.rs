//! The grid view's automatic layout (Cytoscape's `cose` is quadratic in the
//! nodes on every iteration: RealGrid's 4875 substations do not finish).
//!
//! Each connected component is projected by PivotMDS, which keeps graph
//! distances (a force layout alone, even multilevel, leaves a large grid
//! folded), then refined by local forces — Fruchterman–Reingold with
//! repulsion only between nodes in neighbouring cells, so a step is linear —
//! which pull apart what the projection put on top of each other. Linear in
//! nodes and edges times the pivots, and deterministic: the same grid, the
//! same layout.

use std::collections::VecDeque;

use cimmodel::base::FastMap;

/// The ideal distance between neighbours.
const K: f64 = 1.0;
/// Steps of local forces after the projection.
const REFINE_ITERATIONS: usize = 80;

/// A graph: its node count, and its edges without loops or repeats.
struct Graph {
    n: usize,
    edges: Vec<(usize, usize)>,
}

impl Graph {
    fn len(&self) -> usize {
        self.n
    }

    fn adjacency(&self) -> Vec<Vec<usize>> {
        let mut adjacent = vec![Vec::new(); self.len()];
        for &(a, b) in &self.edges {
            adjacent[a].push(b);
            adjacent[b].push(a);
        }
        adjacent
    }
}

/// Positions for `n` nodes joined by `edges`, in units of [`K`]: each
/// connected component by [`pivot_mds`], refined by local forces, the
/// components packed in rows, the largest first.
pub fn force(n: usize, edges: &[(usize, usize)]) -> Vec<(f64, f64)> {
    let g = Graph { n, edges: simple(edges.iter().copied()) };
    let adjacent = g.adjacency();
    // The components, largest first.
    let mut component = vec![usize::MAX; n];
    let mut members: Vec<Vec<usize>> = Vec::new();
    for s in 0..n {
        if component[s] != usize::MAX {
            continue;
        }
        let c = members.len();
        component[s] = c;
        let mut queue = VecDeque::from([s]);
        let mut held = Vec::new();
        while let Some(i) = queue.pop_front() {
            held.push(i);
            for &j in &adjacent[i] {
                if component[j] == usize::MAX {
                    component[j] = c;
                    queue.push_back(j);
                }
            }
        }
        members.push(held);
    }
    members.sort_by_key(|m| std::cmp::Reverse(m.len()));

    let mut pos = vec![(0.0, 0.0); n];
    let total_area: f64 = members.iter().map(|m| m.len() as f64).sum::<f64>() * 4.0 * K * K;
    let row_width = total_area.sqrt() * 1.3;
    let (mut x, mut y, mut row_height) = (0.0_f64, 0.0_f64, 0.0_f64);
    for held in &members {
        // The component on its own: local numbering.
        let local: FastMap<usize, usize> = held.iter().enumerate().map(|(i, &v)| (v, i)).collect();
        let sub = Graph {
            n: held.len(),
            edges: g.edges.iter().filter_map(|(a, b)| Some((*local.get(a)?, *local.get(b)?))).collect(),
        };
        let mut p = pivot_mds(&sub);
        refine(&mut p, &sub, REFINE_ITERATIONS, 1.0 * K);
        let (min_x, max_x) = p.iter().fold((f64::MAX, f64::MIN), |a, q| (a.0.min(q.0), a.1.max(q.0)));
        let (min_y, max_y) = p.iter().fold((f64::MAX, f64::MIN), |a, q| (a.0.min(q.1), a.1.max(q.1)));
        let (w, h) = (max_x - min_x, max_y - min_y);
        if x > 0.0 && x + w > row_width {
            (x, y, row_height) = (0.0, y + row_height + 2.0 * K, 0.0);
        }
        for (i, &v) in held.iter().enumerate() {
            pos[v] = (x + p[i].0 - min_x, y + p[i].1 - min_y);
        }
        x += w + 2.0 * K;
        row_height = row_height.max(h);
    }
    pos
}

/// How many pivots [`pivot_mds`] measures from.
const PIVOTS: usize = 50;

/// PivotMDS (Brandes & Pich 2006) of a connected graph: breadth-first
/// distances from a few pivots, each the farthest from those before; the
/// double-centred squared distances' two main directions, by power
/// iteration on their k × k Gram matrix; scaled to edges of length [`K`].
fn pivot_mds(g: &Graph) -> Vec<(f64, f64)> {
    let n = g.len();
    if n <= 2 {
        return (0..n).map(|i| (i as f64 * K, 0.0)).collect();
    }
    let adjacent = g.adjacency();
    let bfs = |s: usize| {
        let mut d = vec![f64::MAX; n];
        d[s] = 0.0;
        let mut queue = VecDeque::from([s]);
        while let Some(i) = queue.pop_front() {
            for &j in &adjacent[i] {
                if d[j] == f64::MAX {
                    d[j] = d[i] + 1.0;
                    queue.push_back(j);
                }
            }
        }
        d
    };
    let k = PIVOTS.min(n);
    // Pivots: max-min, from the best connected node.
    let first = (0..n).max_by_key(|&i| (adjacent[i].len(), std::cmp::Reverse(i))).unwrap_or(0);
    let mut columns: Vec<Vec<f64>> = vec![bfs(first)];
    let mut nearest = columns[0].clone();
    while columns.len() < k {
        let next = (0..n).max_by(|&a, &b| nearest[a].total_cmp(&nearest[b]).then(b.cmp(&a))).unwrap();
        if nearest[next] == 0.0 {
            break;
        }
        let d = bfs(next);
        for (m, v) in nearest.iter_mut().zip(&d) {
            *m = m.min(*v);
        }
        columns.push(d);
    }
    let k = columns.len();
    // C: −½ of the squared distances, double-centred (n × k).
    let mut c: Vec<Vec<f64>> = columns.iter().map(|col| col.iter().map(|d| d * d).collect()).collect();
    let col_mean: Vec<f64> = c.iter().map(|col| col.iter().sum::<f64>() / n as f64).collect();
    let row_mean: Vec<f64> = (0..n).map(|i| c.iter().map(|col| col[i]).sum::<f64>() / k as f64).collect();
    let all = col_mean.iter().sum::<f64>() / k as f64;
    for (j, col) in c.iter_mut().enumerate() {
        for (i, v) in col.iter_mut().enumerate() {
            *v = -0.5 * (*v - row_mean[i] - col_mean[j] + all);
        }
    }
    // B = CᵀC (k × k), and its two leading eigenvectors.
    let b: Vec<Vec<f64>> = (0..k).map(|p| (0..k).map(|q| c[p].iter().zip(&c[q]).map(|(x, y)| x * y).sum()).collect()).collect();
    let mut vectors: Vec<Vec<f64>> = Vec::new();
    for axis in 0..2 {
        // A deterministic start, apart from the first axis.
        let mut v: Vec<f64> = (0..k).map(|i| if (i + axis) % 2 == 0 { 1.0 } else { 0.5 }).collect();
        for _ in 0..200 {
            let mut w: Vec<f64> = (0..k).map(|p| b[p].iter().zip(&v).map(|(x, y)| x * y).sum()).collect();
            for u in &vectors {
                let dot: f64 = w.iter().zip(u).map(|(x, y)| x * y).sum();
                w.iter_mut().zip(u).for_each(|(x, y)| *x -= dot * y);
            }
            let norm = w.iter().map(|x| x * x).sum::<f64>().sqrt();
            if norm < 1e-12 {
                break;
            }
            v = w.into_iter().map(|x| x / norm).collect();
        }
        vectors.push(v);
    }
    let mut pos: Vec<(f64, f64)> = (0..n)
        .map(|i| {
            let along = |v: &Vec<f64>| (0..k).map(|p| c[p][i] * v[p]).sum::<f64>();
            (along(&vectors[0]), along(&vectors[1]))
        })
        .collect();
    // Edges of length K, on average.
    let mean = g.edges.iter().map(|&(a, b)| ((pos[a].0 - pos[b].0).powi(2) + (pos[a].1 - pos[b].1).powi(2)).sqrt()).sum::<f64>()
        / g.edges.len().max(1) as f64;
    let scale = if mean > 1e-12 { K / mean } else { 1.0 };
    // Nodes alike in their distances (the boundary points between the same
    // two substations) project onto one point, where no force parts them:
    // apart a little, each its own way.
    pos.iter_mut().enumerate().for_each(|(i, p)| {
        let a = i as f64 * 2.399_963;
        *p = (p.0 * scale + 0.05 * K * a.cos(), p.1 * scale + 0.05 * K * a.sin());
    });
    pos
}

/// `edges` without loops or repeats, each as (smaller, larger).
fn simple(edges: impl Iterator<Item = (usize, usize)>) -> Vec<(usize, usize)> {
    let mut out: Vec<(usize, usize)> = edges.filter(|(a, b)| a != b).map(|(a, b)| (a.min(b), a.max(b))).collect();
    out.sort_unstable();
    out.dedup();
    out
}

/// `iterations` steps of Fruchterman–Reingold from `pos`, the step at most
/// `t`, cooling: repulsion k² / d from the nodes in neighbouring cells, and
/// attraction d² / k along the edges.
fn refine(pos: &mut [(f64, f64)], g: &Graph, iterations: usize, mut t: f64) {
    let n = pos.len();
    let cell = 2.0 * K;
    let mut disp = vec![(0.0, 0.0); n];
    let mut cells: FastMap<(i64, i64), Vec<usize>> = FastMap::default();
    let cool = (0.02 * K / t).powf(1.0 / iterations.max(1) as f64).min(0.99);
    for _ in 0..iterations {
        disp.iter_mut().for_each(|d| *d = (0.0, 0.0));
        cells.values_mut().for_each(Vec::clear);
        let key = |p: (f64, f64)| ((p.0 / cell).floor() as i64, (p.1 / cell).floor() as i64);
        for (i, &p) in pos.iter().enumerate() {
            cells.entry(key(p)).or_default().push(i);
        }
        for (i, &p) in pos.iter().enumerate() {
            let (cx, cy) = key(p);
            for dx in -1..=1 {
                for dy in -1..=1 {
                    let Some(near) = cells.get(&(cx + dx, cy + dy)) else { continue };
                    for &j in near {
                        if j <= i {
                            continue;
                        }
                        let (x, y) = (p.0 - pos[j].0, p.1 - pos[j].1);
                        let d2 = (x * x + y * y).max(1e-4);
                        if d2 > cell * cell {
                            continue;
                        }
                        let f = K * K / d2;
                        disp[i].0 += x * f;
                        disp[i].1 += y * f;
                        disp[j].0 -= x * f;
                        disp[j].1 -= y * f;
                    }
                }
            }
        }
        for &(a, b) in &g.edges {
            let (x, y) = (pos[a].0 - pos[b].0, pos[a].1 - pos[b].1);
            let f = (x * x + y * y).sqrt() / K;
            disp[a].0 -= x * f;
            disp[a].1 -= y * f;
            disp[b].0 += x * f;
            disp[b].1 += y * f;
        }
        for (i, p) in pos.iter_mut().enumerate() {
            let (x, y) = disp[i];
            let len = (x * x + y * y).sqrt();
            if len > 0.0 {
                let step = len.min(t);
                p.0 += x / len * step;
                p.1 += y / len * step;
            }
        }
        t *= cool;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// How well the layout keeps graph distances: the correlation of hops and
    /// distance from a few nodes to all the others they reach.
    fn faithfulness(n: usize, edges: &[(usize, usize)], pos: &[(f64, f64)]) -> f64 {
        let g = Graph { n, edges: simple(edges.iter().copied()) };
        let adjacent = g.adjacency();
        let (mut xs, mut ys) = (Vec::new(), Vec::new());
        for s in (0..n).step_by(n / 7 + 1) {
            let mut hops = vec![usize::MAX; n];
            hops[s] = 0;
            let mut queue = VecDeque::from([s]);
            while let Some(i) = queue.pop_front() {
                for &j in &adjacent[i] {
                    if hops[j] == usize::MAX {
                        hops[j] = hops[i] + 1;
                        queue.push_back(j);
                    }
                }
            }
            for j in (0..n).filter(|&j| j != s && hops[j] != usize::MAX) {
                xs.push(hops[j] as f64);
                ys.push(((pos[s].0 - pos[j].0).powi(2) + (pos[s].1 - pos[j].1).powi(2)).sqrt());
            }
        }
        let m = xs.len() as f64;
        let (mx, my) = (xs.iter().sum::<f64>() / m, ys.iter().sum::<f64>() / m);
        let cov: f64 = xs.iter().zip(&ys).map(|(x, y)| (x - mx) * (y - my)).sum();
        let (vx, vy): (f64, f64) = (xs.iter().map(|x| (x - mx).powi(2)).sum(), ys.iter().map(|y| (y - my).powi(2)).sum());
        cov / (vx * vy).sqrt()
    }

    fn overlapping(pos: &[(f64, f64)]) -> usize {
        let n = pos.len();
        let d = |a: usize, b: usize| ((pos[a].0 - pos[b].0).powi(2) + (pos[a].1 - pos[b].1).powi(2)).sqrt();
        (0..n).step_by(7).map(|i| (0..n).filter(|&j| i != j && d(i, j) < 0.1 * K).count()).sum()
    }

    /// A ring of 2000 nodes: spread, neighbours near, distances kept.
    #[test]
    fn lays_out_a_ring() {
        let n = 2000;
        let edges: Vec<(usize, usize)> = (0..n).map(|i| (i, (i + 1) % n)).collect();
        let t = std::time::Instant::now();
        let pos = force(n, &edges);
        assert!(t.elapsed().as_secs_f64() < 2.0, "{:?}", t.elapsed());
        assert!(overlapping(&pos) < 10);
        let f = faithfulness(n, &edges, &pos);
        assert!(f > 0.95, "{f}");
    }

    /// A 60 × 60 lattice, which a force layout leaves folded: unfolded.
    #[test]
    fn unfolds_a_lattice() {
        let side = 60;
        let n = side * side;
        let mut edges = Vec::new();
        for r in 0..side {
            for c in 0..side {
                let i = r * side + c;
                if c + 1 < side {
                    edges.push((i, i + 1));
                }
                if r + 1 < side {
                    edges.push((i, i + side));
                }
            }
        }
        let t = std::time::Instant::now();
        let pos = force(n, &edges);
        assert!(t.elapsed().as_secs_f64() < 3.0, "{:?}", t.elapsed());
        assert!(overlapping(&pos) < 20);
        let f = faithfulness(n, &edges, &pos);
        assert!(f > 0.95, "{f}");
    }

    /// Three nodes between the same two (MicroGrid's border points): parted.
    #[test]
    fn parts_nodes_alike() {
        let edges = [(0, 2), (1, 2), (0, 3), (1, 3), (0, 4), (1, 4)];
        let pos = force(5, &edges);
        for (a, b) in [(2, 3), (2, 4), (3, 4)] {
            let d = ((pos[a].0 - pos[b].0).powi(2) + (pos[a].1 - pos[b].1).powi(2)).sqrt();
            assert!(d > 0.3 * K, "{a} and {b} at {d}");
        }
    }
}

