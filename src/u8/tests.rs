mod builder;
mod set;

mod test_support {
    use int_interval::U8CO;

    use proptest::prelude::*;

    #[inline]
    pub(super) fn iv(start: u8, end_excl: u8) -> U8CO {
        U8CO::try_new(start, end_excl).unwrap()
    }

    pub(super) const MID_VALUE: u8 = u8::MAX / 2 + 1;

    pub(super) fn interval_pair() -> impl Strategy<Value = (u8, u8)> {
        (any::<u8>(), any::<u8>()).prop_filter_map("non-empty half-open interval", |(a, b)| {
            let start = a.min(b);
            let end_excl = a.max(b);

            (start < end_excl).then_some((start, end_excl))
        })
    }

    pub(super) fn build_from_vec(xs: Vec<(u8, u8)>) -> crate::U8COSet {
        let b = crate::U8COSetBuilder::new();

        for (start, end_excl) in xs {
            b.insert(iv(start, end_excl));
        }

        b.seal()
    }
}
