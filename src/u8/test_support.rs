use int_interval::U8CO;
use proptest::prelude::*;

use crate::U8COSet;

#[inline]
pub(super) fn iv(start: u8, end_excl: u8) -> U8CO {
    U8CO::try_new(start, end_excl).unwrap()
}

#[inline]
pub(super) fn intervals(set: &U8COSet) -> &[U8CO] {
    set.intervals.as_ref()
}

#[inline]
pub(super) fn build<const N: usize>(pairs: [(u8, u8); N]) -> U8COSet {
    pairs
        .into_iter()
        .map(|(start, end)| iv(start, end))
        .collect()
}

/// Generates valid, non-empty intervals without enumerating the domain.
///
/// For code generation, replace `u8` and `U8CO` with the target primitive
/// and interval type.
pub(super) fn arb_iv() -> impl Strategy<Value = U8CO> {
    (any::<u8>(), any::<u8>()).prop_filter_map("interval endpoints must differ", |(a, b)| {
        let (start, end) = if a < b { (a, b) } else { (b, a) };
        U8CO::try_new(start, end)
    })
}
