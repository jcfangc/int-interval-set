use super::*;

/// Checks the canonical invariant used by binary-search queries.
///
/// `I` itself already guarantees single-interval validity. This function only
/// checks the relationship between adjacent intervals.
#[inline]
pub(super) fn is_canonical<I: IntCO>(intervals: &[I]) -> bool {
    intervals.windows(2).all(|w| w[0].end_excl() < w[1].start())
}

#[inline]
pub(super) fn normalize<I: IntCO>(mut intervals: Vec<I>) -> Vec<I> {
    intervals.sort_unstable();
    canonicalize_sorted(intervals)
}

#[inline]
pub(super) fn merge<I: IntCO>(left: Vec<I>, right: Vec<I>) -> Vec<I> {
    if left.is_empty() {
        return right;
    }

    if right.is_empty() {
        return left;
    }

    canonicalize_sorted(merge_sorted(left, right))
}

pub(super) fn canonicalize_sorted<I, T>(intervals: T) -> Vec<I>
where
    I: IntCO,
    T: IntoIterator<Item = I>,
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

pub(super) fn merge_sorted<I, L, R>(left: L, right: R) -> impl Iterator<Item = I>
where
    I: IntCO,
    L: IntoIterator<Item = I>,
    R: IntoIterator<Item = I>,
{
    struct Merge<L, R>
    where
        L: Iterator,
        R: Iterator<Item = L::Item>,
    {
        left: std::iter::Peekable<L>,
        right: std::iter::Peekable<R>,
    }

    impl<I, L, R> Iterator for Merge<L, R>
    where
        I: IntCO,
        L: Iterator<Item = I>,
        R: Iterator<Item = I>,
    {
        type Item = I;

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
