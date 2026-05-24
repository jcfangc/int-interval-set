use proptest::prelude::*;

use crate::{
    I8COSet,
    int_co_set::test_support::{arb_iv, build, iv},
};

#[test]
fn symmetric_difference_with_empty_set_returns_other_set() {
    let empty = build([]);
    let set = build([(-20, -10), (10, 20)]);

    assert_eq!(empty.symmetric_difference_with_set(&set), set);
    assert_eq!(set.symmetric_difference_with_set(&empty), set);
}

#[test]
fn symmetric_difference_of_identical_sets_is_empty() {
    let set = build([(-20, -10), (10, 20)]);

    assert!(set.symmetric_difference_with_set(&set).is_empty());
}

#[test]
fn symmetric_difference_removes_overlapping_segments() {
    let left = build([(-20, -10), (10, 20)]);
    let right = build([(-15, -5), (15, 25)]);

    assert_eq!(
        left.symmetric_difference_with_set(&right).as_slice(),
        &[iv(-20, -15), iv(-10, -5), iv(10, 15), iv(20, 25)]
    );
}

#[test]
fn symmetric_difference_merges_adjacent_disjoint_coverage() {
    let left = build([(-40, -30), (10, 20)]);
    let right = build([(-30, 10)]);

    assert_eq!(
        left.symmetric_difference_with_set(&right).as_slice(),
        &[iv(-40, 20)]
    );
}

#[test]
fn symmetric_difference_retains_gaps_between_removed_segments() {
    let left = build([(-50, 50)]);
    let right = build([(-30, -20), (20, 30)]);

    assert_eq!(
        left.symmetric_difference_with_set(&right).as_slice(),
        &[iv(-50, -30), iv(-20, 20), iv(30, 50)]
    );
}

#[test]
fn symmetric_difference_handles_crossing_long_intervals() {
    let left = build([(-50, -40), (-20, 0), (20, 30)]);
    let right = build([(-45, 25)]);

    assert_eq!(
        left.symmetric_difference_with_set(&right).as_slice(),
        &[iv(-50, -45), iv(-40, -20), iv(0, 20), iv(25, 30)]
    );
}

#[test]
fn symmetric_difference_handles_domain_edges() {
    let left = build([(i8::MIN, i8::MIN + 10), (i8::MAX - 10, i8::MAX)]);
    let right = build([(i8::MIN + 5, i8::MAX - 5)]);

    assert_eq!(
        left.symmetric_difference_with_set(&right).as_slice(),
        &[
            iv(i8::MIN, i8::MIN + 5),
            iv(i8::MIN + 10, i8::MAX - 10),
            iv(i8::MAX - 5, i8::MAX),
        ]
    );
}

proptest! {
    #[test]
    fn prop_symmetric_difference_matches_pointwise_membership(
        xs in prop::collection::vec(arb_iv(), 0..64),
        ys in prop::collection::vec(arb_iv(), 0..64),
        x in any::<i8>(),
    ) {
        let left: I8COSet = xs.into_iter().collect();
        let right: I8COSet = ys.into_iter().collect();
        let result = left.symmetric_difference_with_set(&right);

        prop_assert_eq!(
            result.contains_point(x),
            left.contains_point(x) ^ right.contains_point(x)
        );
    }

    #[test]
    fn prop_symmetric_difference_is_commutative(
        xs in prop::collection::vec(arb_iv(), 0..64),
        ys in prop::collection::vec(arb_iv(), 0..64),
    ) {
        let left: I8COSet = xs.into_iter().collect();
        let right: I8COSet = ys.into_iter().collect();

        prop_assert_eq!(
            left.symmetric_difference_with_set(&right),
            right.symmetric_difference_with_set(&left)
        );
    }

    #[test]
    fn prop_symmetric_difference_matches_difference_union_reference_model(
        xs in prop::collection::vec(arb_iv(), 0..64),
        ys in prop::collection::vec(arb_iv(), 0..64),
    ) {
        let left: I8COSet = xs.into_iter().collect();
        let right: I8COSet = ys.into_iter().collect();

        let expected = left
            .difference_with_set(&right)
            .union_with_set(&right.difference_with_set(&left));

        prop_assert_eq!(left.symmetric_difference_with_set(&right), expected);
    }

    #[test]
    fn prop_symmetric_difference_is_disjoint_from_intersection(
        xs in prop::collection::vec(arb_iv(), 0..64),
        ys in prop::collection::vec(arb_iv(), 0..64),
    ) {
        let left: I8COSet = xs.into_iter().collect();
        let right: I8COSet = ys.into_iter().collect();

        let symmetric = left.symmetric_difference_with_set(&right);
        let intersection = left.intersection_with_set(&right);

        prop_assert!(
            symmetric.intersection_with_set(&intersection).is_empty()
        );
    }
}
