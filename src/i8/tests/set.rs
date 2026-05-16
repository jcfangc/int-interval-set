mod basic;
mod coverage;
mod predicate;
mod search;

mod test_support {
    use crate::{I8COSetBuilder, i8::tests::test_support::iv};

    pub(super) fn build<const N: usize>(intervals: [(i8, i8); N]) -> crate::I8COSet {
        let b = I8COSetBuilder::new();

        for (start, end_excl) in intervals {
            b.insert(iv(start, end_excl));
        }

        b.seal()
    }
}
