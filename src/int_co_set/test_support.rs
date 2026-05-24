use int_interval::I8CO;
use proptest::prelude::*;

use crate::I8COSet;

#[inline]
pub(super) fn iv(start: i8, end_excl: i8) -> I8CO {
    I8CO::try_new(start, end_excl).unwrap()
}

#[inline]
pub(super) fn intervals(set: &I8COSet) -> &[I8CO] {
    set.intervals.as_ref()
}

#[inline]
pub(super) fn build<const N: usize>(pairs: [(i8, i8); N]) -> I8COSet {
    pairs
        .into_iter()
        .map(|(start, end)| iv(start, end))
        .collect()
}

/// Generates valid, non-empty intervals without enumerating the domain.
///
/// For code generation, replace `i8` and `I8CO` with the target primitive
/// and interval type.
pub(super) fn arb_iv() -> impl Strategy<Value = I8CO> {
    (any::<i8>(), any::<i8>()).prop_filter_map("interval endpoints must differ", |(a, b)| {
        let (start, end) = if a < b { (a, b) } else { (b, a) };
        I8CO::try_new(start, end)
    })
}
