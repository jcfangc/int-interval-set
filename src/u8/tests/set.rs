mod basic;
mod coverage;
mod predicate;
mod search;

mod test_support {
    use crate::{U8COSetBuilder, u8::tests::test_support::iv};

    pub(super) fn build<const N: usize>(intervals: [(u8, u8); N]) -> crate::U8COSet {
        let b = U8COSetBuilder::new();

        for (start, end_excl) in intervals {
            b.insert(iv(start, end_excl));
        }

        b.seal()
    }
}
