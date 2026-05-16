mod builder;
mod set;

mod test_support {
    use int_interval::I8CO;

    use proptest::prelude::*;

    #[inline]
    pub(super) fn iv(start: i8, end_excl: i8) -> I8CO {
        I8CO::try_new(start, end_excl).unwrap()
    }

    pub(super) const MID_VALUE: i8 = 0;

    pub(super) fn interval_pair() -> impl Strategy<Value = (i8, i8)> {
        (any::<i8>(), any::<i8>()).prop_filter_map("non-empty half-open interval", |(a, b)| {
            let start = a.min(b);
            let end_excl = a.max(b);

            (start < end_excl).then_some((start, end_excl))
        })
    }

    pub(super) fn build_from_vec(xs: Vec<(i8, i8)>) -> crate::I8COSet {
        let b = crate::I8COSetBuilder::new();

        for (start, end_excl) in xs {
            b.insert(iv(start, end_excl));
        }

        b.seal()
    }
}
