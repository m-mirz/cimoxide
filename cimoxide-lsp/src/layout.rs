//! A force-directed layout for the grid view's automatic layout (Cytoscape's
//! `cose` is quadratic in the nodes on every iteration: RealGrid's 4875
//! substations do not finish). Fruchterman–Reingold, with repulsion only
//! between nodes in neighbouring cells of a grid, so an iteration is linear;
//! started from a spiral in breadth-first order, so that what is connected
//! starts near. Deterministic: the same grid, the same layout.

use std::collections::VecDeque;

use cimmodel::base::FastMap;

/// The ideal distance between neighbours.
const K: f64 = 1.0;
const ITERATIONS: usize = 300;

/// Positions for `n` nodes joined by `edges`, in units of [`K`].
pub fn force(n: usize, edges: &[(usize, usize)]) -> Vec<(f64, f64)> {
    let mut adjacent = vec![Vec::new(); n];
    for &(a, b) in edges {
        if a != b {
            adjacent[a].push(b);
            adjacent[b].push(a);
        }
    }
    // Breadth-first, component by component, the best connected first.
    let mut order = Vec::with_capacity(n);
    let mut seen = vec![false; n];
    let mut starts: Vec<usize> = (0..n).collect();
    starts.sort_by_key(|&i| std::cmp::Reverse(adjacent[i].len()));
    for s in starts {
        if seen[s] {
            continue;
        }
        seen[s] = true;
        let mut queue = VecDeque::from([s]);
        while let Some(i) = queue.pop_front() {
            order.push(i);
            for &j in &adjacent[i] {
                if !seen[j] {
                    seen[j] = true;
                    queue.push_back(j);
                }
            }
        }
    }
    let mut pos = vec![(0.0, 0.0); n];
    for (rank, &i) in order.iter().enumerate() {
        let (r, a) = (0.8 * K * (rank as f64).sqrt(), rank as f64 * 2.399_963);
        pos[i] = (r * a.cos(), r * a.sin());
    }

    let cell = 2.0 * K;
    let side = (n as f64).sqrt() * K;
    let mut t = side / 8.0 + K;
    let mut disp = vec![(0.0, 0.0); n];
    let mut cells: FastMap<(i64, i64), Vec<usize>> = FastMap::default();
    for _ in 0..ITERATIONS {
        disp.iter_mut().for_each(|d| *d = (0.0, 0.0));
        cells.values_mut().for_each(Vec::clear);
        let key = |p: (f64, f64)| ((p.0 / cell).floor() as i64, (p.1 / cell).floor() as i64);
        for (i, &p) in pos.iter().enumerate() {
            cells.entry(key(p)).or_default().push(i);
        }
        // Repulsion, from the nodes near: k² / d.
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
        // Attraction along the edges: d² / k.
        for &(a, b) in edges {
            let (x, y) = (pos[a].0 - pos[b].0, pos[a].1 - pos[b].1);
            let f = (x * x + y * y).sqrt() / K;
            disp[a].0 -= x * f;
            disp[a].1 -= y * f;
            disp[b].0 += x * f;
            disp[b].1 += y * f;
        }
        // A little gravity, so that components stay together.
        let (sx, sy) = pos.iter().fold((0.0, 0.0), |a, p| (a.0 + p.0, a.1 + p.1));
        let c = (sx / n as f64, sy / n as f64);
        for (i, p) in pos.iter_mut().enumerate() {
            let (x, y) = (disp[i].0 - 0.02 * (p.0 - c.0), disp[i].1 - 0.02 * (p.1 - c.1));
            let len = (x * x + y * y).sqrt();
            if len > 0.0 {
                let step = len.min(t);
                p.0 += x / len * step;
                p.1 += y / len * step;
            }
        }
        t = (t * 0.985).max(0.02 * K);
    }
    pos
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A ring of 2000 nodes: no two on top of each other, neighbours near.
    #[test]
    fn spreads_and_keeps_neighbours_near() {
        let n = 2000;
        let edges: Vec<(usize, usize)> = (0..n).map(|i| (i, (i + 1) % n)).collect();
        let t = std::time::Instant::now();
        let pos = force(n, &edges);
        assert!(t.elapsed().as_secs_f64() < 2.0, "{:?}", t.elapsed());
        let d = |a: usize, b: usize| ((pos[a].0 - pos[b].0).powi(2) + (pos[a].1 - pos[b].1).powi(2)).sqrt();
        let mean_edge = edges.iter().map(|&(a, b)| d(a, b)).sum::<f64>() / n as f64;
        assert!(mean_edge > 0.3 * K && mean_edge < 3.0 * K, "{mean_edge}");
        let mut close = 0;
        for i in (0..n).step_by(7) {
            for j in 0..n {
                if i != j && d(i, j) < 0.1 * K {
                    close += 1;
                }
            }
        }
        assert!(close < 10, "{close} pairs nearly on top of each other");
    }
}
