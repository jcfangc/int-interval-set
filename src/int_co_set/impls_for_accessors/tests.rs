use proptest::prelude::*;

use crate::{
    I8COSet,
    int_co_set::test_support::{arb_iv, build, iv},
};

#[test]
fn empty_set_has_empty_shape() {
    let set = build([]);

    assert!(set.is_empty());
    assert_eq!(set.interval_count(), 0);
    assert_eq!(set.as_slice(), &[]);
    assert_eq!(set.iter_intervals().collect::<Vec<_>>(), Vec::new());
}

#[test]
fn interval_count_counts_canonical_intervals() {
    let set = build([(-10, -5), (-5, 0), (10, 20), (30, 40)]);

    assert_eq!(set.interval_count(), 3);
    assert_eq!(set.as_slice(), &[iv(-10, 0), iv(10, 20), iv(30, 40)]);
}

#[test]
fn as_slice_returns_sorted_canonical_slice() {
    let set = build([(30, 40), (-20, -10), (-12, 0), (50, 60)]);

    assert_eq!(set.as_slice(), &[iv(-20, 0), iv(30, 40), iv(50, 60)]);
}

#[test]
fn iter_intervals_yields_canonical_intervals_in_order() {
    let set = build([(10, 20), (-10, -5), (-5, -2), (30, 40)]);

    assert_eq!(
        set.iter_intervals().collect::<Vec<_>>(),
        vec![iv(-10, -2), iv(10, 20), iv(30, 40)]
    );
}

#[test]
fn clone_preserves_logical_content() {
    let set = build([(-10, 0), (20, 30)]);
    let cloned = set.clone();

    assert_eq!(cloned.as_slice(), set.as_slice());
    assert_eq!(cloned.interval_count(), set.interval_count());
    assert_eq!(cloned.is_empty(), set.is_empty());
}

#[test]
fn sparse_intervals_remain_sorted_and_canonical_at_domain_edges() {
    let set = build([
        (i8::MIN, i8::MIN + 1),
        (i8::MIN + 2, i8::MIN + 3),
        (-2, -1),
        (-1, 1),
        (10, 20),
        (i8::MAX - 1, i8::MAX),
    ]);

    assert_eq!(set.interval_count(), 5);
    assert_eq!(
        set.as_slice(),
        &[
            iv(i8::MIN, i8::MIN + 1),
            iv(i8::MIN + 2, i8::MIN + 3),
            iv(-2, 1),
            iv(10, 20),
            iv(i8::MAX - 1, i8::MAX),
        ]
    );
}

#[test]
fn is_empty_is_false_after_any_interval() {
    let set = build([(-1, 1)]);

    assert!(!set.is_empty());
    assert_eq!(set.interval_count(), 1);
    assert_eq!(set.as_slice(), &[iv(-1, 1)]);
}

proptest! {
    #[test]
    fn prop_accessors_are_consistent(xs in prop::collection::vec(arb_iv(), 0..64)) {
        let set: I8COSet = xs.into_iter().collect();
        let iterated = set.iter_intervals().collect::<Vec<_>>();

        prop_assert_eq!(iterated.as_slice(), set.as_slice());
        prop_assert_eq!(set.interval_count(), set.as_slice().len());
        prop_assert_eq!(set.is_empty(), set.as_slice().is_empty());
    }

    #[test]
    fn prop_clone_preserves_accessors(xs in prop::collection::vec(arb_iv(), 0..64)) {
        let set: I8COSet = xs.into_iter().collect();
        let cloned = set.clone();

        prop_assert_eq!(cloned.as_slice(), set.as_slice());
        prop_assert_eq!(
            cloned.iter_intervals().collect::<Vec<_>>(),
            set.iter_intervals().collect::<Vec<_>>()
        );
        prop_assert_eq!(cloned.interval_count(), set.interval_count());
        prop_assert_eq!(cloned.is_empty(), set.is_empty());
    }
}
