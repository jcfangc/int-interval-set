use rayon::iter::{FromParallelIterator, IntoParallelIterator, ParallelIterator};

use super::*;

const BATCH_SIZE: usize = 128;

impl U8COSet {
    /// Builds a set from an already canonical interval vector.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that `intervals` is canonical:
    ///
    /// - intervals are sorted by ascending `start`;
    /// - intervals are non-overlapping;
    /// - contiguous intervals have already been merged;
    /// - therefore, for every adjacent pair `a, b`,
    ///   `a.end_excl() < b.start()` holds.
    ///
    /// Violating this invariant can make binary-search based queries
    /// return incorrect results.
    #[inline]
    unsafe fn new_unchecked(intervals: Vec<U8CO>) -> Self {
        debug_assert!(is_canonical(&intervals));

        Self {
            intervals: Arc::from(intervals.into_boxed_slice()),
        }
    }
}

/// Checks the canonical invariant used by binary-search queries.
///
/// `U8CO` itself already guarantees single-interval validity. This
/// function only checks the relationship between adjacent intervals.
#[inline]
fn is_canonical(intervals: &[U8CO]) -> bool {
    intervals.windows(2).all(|w| w[0].end_excl() < w[1].start())
}

impl FromIterator<U8CO> for U8COSet {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = U8CO>,
    {
        let mut batch = Vec::with_capacity(BATCH_SIZE);
        let mut reduced = Vec::new();

        for iv in iter {
            batch.push(iv);

            if batch.len() == BATCH_SIZE {
                let run = normalize(std::mem::replace(
                    &mut batch,
                    Vec::with_capacity(BATCH_SIZE),
                ));

                reduced = merge(reduced, run);
            }
        }

        if !batch.is_empty() {
            reduced = merge(reduced, normalize(batch));
        }

        // SAFETY:
        // Each normalized batch is canonical. `merge` preserves sorted,
        // non-overlapping, and non-adjacent interval invariants.
        unsafe { U8COSet::new_unchecked(reduced) }
    }
}

impl FromParallelIterator<U8CO> for U8COSet {
    fn from_par_iter<I>(iter: I) -> Self
    where
        I: IntoParallelIterator<Item = U8CO>,
    {
        let reduced = iter
            .into_par_iter()
            .fold(
                || (Vec::with_capacity(BATCH_SIZE), Vec::<U8CO>::new()),
                |(mut batch, mut reduced), iv| {
                    batch.push(iv);

                    if batch.len() == BATCH_SIZE {
                        let run = normalize(std::mem::replace(
                            &mut batch,
                            Vec::with_capacity(BATCH_SIZE),
                        ));

                        reduced = merge(reduced, run);
                    }

                    (batch, reduced)
                },
            )
            .map(|(batch, reduced)| {
                if batch.is_empty() {
                    reduced
                } else {
                    merge(reduced, normalize(batch))
                }
            })
            .reduce(Vec::new, merge);

        // SAFETY:
        // Every parallel fold result is canonical, and `merge` preserves the
        // canonical interval invariant during reduction.
        unsafe { U8COSet::new_unchecked(reduced) }
    }
}

#[inline]
fn normalize(mut intervals: Vec<U8CO>) -> Vec<U8CO> {
    intervals.sort_unstable();
    canonicalize_sorted(intervals)
}

#[inline]
fn merge(left: Vec<U8CO>, right: Vec<U8CO>) -> Vec<U8CO> {
    if left.is_empty() {
        return right;
    }

    if right.is_empty() {
        return left;
    }

    canonicalize_sorted(merge_sorted(left, right))
}

fn canonicalize_sorted<I>(intervals: I) -> Vec<U8CO>
where
    I: IntoIterator<Item = U8CO>,
{
    let mut iter = intervals.into_iter();

    let Some(mut cur) = iter.next() else {
        return Vec::new();
    };

    let mut out = Vec::new();

    for iv in iter {
        if cur.is_contiguous_with(iv) {
            cur = cur.convex_hull(iv);
        } else {
            out.push(cur);
            cur = iv;
        }
    }

    out.push(cur);
    out
}

fn merge_sorted<I, J>(left: I, right: J) -> impl Iterator<Item = U8CO>
where
    I: IntoIterator<Item = U8CO>,
    J: IntoIterator<Item = U8CO>,
{
    struct Merge<I, J>
    where
        I: Iterator<Item = U8CO>,
        J: Iterator<Item = U8CO>,
    {
        left: std::iter::Peekable<I>,
        right: std::iter::Peekable<J>,
    }

    impl<I, J> Iterator for Merge<I, J>
    where
        I: Iterator<Item = U8CO>,
        J: Iterator<Item = U8CO>,
    {
        type Item = U8CO;

        #[inline]
        fn next(&mut self) -> Option<Self::Item> {
            match (self.left.peek(), self.right.peek()) {
                (Some(left), Some(right)) if left <= right => self.left.next(),
                (Some(_), Some(_)) => self.right.next(),
                (Some(_), None) => self.left.next(),
                (None, Some(_)) => self.right.next(),
                (None, None) => None,
            }
        }
    }

    Merge {
        left: left.into_iter().peekable(),
        right: right.into_iter().peekable(),
    }
}

#[cfg(test)]
mod tests_for_from_iter;
#[cfg(test)]
mod tests_for_from_par_iter;
