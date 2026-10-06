//! Splitting a walk over a dataset across threads.
//!
//! Validation is memory-bound — each element costs a cache miss into its own
//! field map — and threads overlap those misses well. Work is cut into
//! contiguous runs and the results concatenated in run order, so the output is
//! exactly what one thread produces, order included.

/// Below this many elements a walk stays on the calling thread.
const PARALLEL_MIN: usize = 20_000;
/// Past a handful of threads a walk is limited by memory, not by cores.
const MAX_THREADS: usize = 8;

/// How many threads a walk over `total` elements gets.
pub(crate) fn threads_for(total: usize) -> usize {
    if total < PARALLEL_MIN {
        1
    } else {
        std::thread::available_parallelism().map_or(1, |n| n.get()).min(MAX_THREADS)
    }
}

/// Cut `items` into at most `threads` contiguous runs of about equal `weight`.
pub(crate) fn runs<T>(items: &[T], threads: usize, weight: impl Fn(&T) -> usize) -> Vec<&[T]> {
    let total: usize = items.iter().map(&weight).sum();
    let per_thread = total.div_ceil(threads.max(1)).max(1);
    let mut out = Vec::with_capacity(threads);
    let (mut start, mut size) = (0, 0);
    for (i, item) in items.iter().enumerate() {
        size += weight(item);
        if size >= per_thread {
            out.push(&items[start..=i]);
            (start, size) = (i + 1, 0);
        }
    }
    if start < items.len() {
        out.push(&items[start..]);
    }
    out
}

/// Run `f` over each run on its own thread and return the results in run order.
pub(crate) fn par_map<T: Sync, R: Send>(runs: &[&[T]], f: impl Fn(&[T]) -> R + Sync) -> Vec<R> {
    match runs {
        [one] => vec![f(one)],
        _ => std::thread::scope(|s| {
            let handles: Vec<_> = runs.iter().map(|run| s.spawn(|| f(run))).collect();
            handles.into_iter().map(|h| h.join().expect("validation thread panicked")).collect()
        }),
    }
}

/// [`par_map`], concatenating the results.
pub(crate) fn par_concat<T: Sync, R: Send>(runs: &[&[T]], f: impl Fn(&[T]) -> Vec<R> + Sync) -> Vec<R> {
    par_map(runs, f).into_iter().flatten().collect()
}

/// Run independent rule groups over one dataset on their own threads, and
/// concatenate their results in the order given.
pub(crate) fn par_groups(
    dataset: &cimdecoder::CimDataset,
    groups: &[fn(&cimdecoder::CimDataset) -> Vec<crate::Violation>],
) -> Vec<crate::Violation> {
    std::thread::scope(|s| {
        let handles: Vec<_> = groups.iter().map(|g| s.spawn(move || g(dataset))).collect();
        handles
            .into_iter()
            .flat_map(|h| h.join().expect("validation thread panicked"))
            .collect()
    })
}

#[cfg(test)]
mod tests {
    use super::runs;

    /// The runs, concatenated, are the input — nothing dropped, repeated or
    /// reordered. That is what makes a parallel walk's output the sequential one.
    #[test]
    fn runs_cover_the_input_in_order() {
        let items: Vec<usize> = (0..1000).collect();
        for threads in [1, 2, 3, 7, 8, 16, 2000] {
            let rs = runs(&items, threads, |_| 1);
            assert!(rs.len() <= threads.max(1), "{threads} threads gave {} runs", rs.len());
            assert!(rs.iter().all(|r| !r.is_empty()));
            let joined: Vec<usize> = rs.concat();
            assert_eq!(joined, items, "{threads} threads");
        }
    }

    #[test]
    fn runs_balance_by_weight() {
        // One heavy item and many light ones: the heavy one gets a run of its own.
        let items = [100, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
        let rs = runs(&items, 2, |w| *w);
        assert_eq!(rs.concat(), items);
        assert_eq!(rs[0], &[100]);
    }

    #[test]
    fn empty_input_has_no_runs() {
        assert!(runs::<u8>(&[], 4, |_| 1).is_empty());
    }
}
