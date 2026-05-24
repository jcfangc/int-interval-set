use proptest::prelude::*;

use crate::{
    U8COSet,
    u8::test_support::{arb_iv, build, iv},
};

#[test]
fn symmetric_difference_with_empty_set_returns_other_set() {
    let empty = build([]);
    let set = build([(10, 20), (30, 40)]);

    assert_eq!(empty.symmetric_difference_with_set(&set), set);
    assert_eq!(set.symmetric_difference_with_set(&empty), set);
}

#[test]
fn symmetric_difference_of_identical_sets_is_empty() {
    let set = build([(10, 20), (30, 40)]);

    assert!(set.symmetric_difference_with_set(&set).is_empty());
}

#[test]
fn symmetric_difference_removes_overlapping_segments() {
    let left = build([(0, 10), (20, 30)]);
    let right = build([(5, 15), (25, 35)]);

    assert_eq!(
        left.symmetric_difference_with_set(&right).as_slice(),
        &[iv(0, 5), iv(10, 15), iv(20, 25), iv(30, 35)]
    );
}

#[test]
fn symmetric_difference_merges_adjacent_disjoint_coverage() {
    let left = build([(0, 10), (30, 40)]);
    let right = build([(10, 30)]);

    assert_eq!(
        left.symmetric_difference_with_set(&right).as_slice(),
        &[iv(0, 40)]
    );
}

#[test]
fn symmetric_difference_retains_gaps_between_removed_segments() {
    let left = build([(0, 50)]);
    let right = build([(10, 20), (30, 40)]);

    assert_eq!(
        left.symmetric_difference_with_set(&right).as_slice(),
        &[iv(0, 10), iv(20, 30), iv(40, 50)]
    );
}

#[test]
fn symmetric_difference_handles_crossing_long_intervals() {
    let left = build([(0, 10), (20, 30), (40, 50)]);
    let right = build([(5, 45)]);

    assert_eq!(
        left.symmetric_difference_with_set(&right).as_slice(),
        &[iv(0, 5), iv(10, 20), iv(30, 40), iv(45, 50)]
    );
}

#[test]
fn symmetric_difference_handles_domain_edges() {
    let left = build([(u8::MIN, u8::MIN + 10), (u8::MAX - 10, u8::MAX)]);
    let right = build([(u8::MIN + 5, u8::MAX - 5)]);

    assert_eq!(
        left.symmetric_difference_with_set(&right).as_slice(),
        &[
            iv(u8::MIN, u8::MIN + 5),
            iv(u8::MIN + 10, u8::MAX - 10),
            iv(u8::MAX - 5, u8::MAX),
        ]
    );
}

proptest! {
    #[test]
    fn prop_symmetric_difference_matches_pointwise_membership(
        xs in prop::collection::vec(arb_iv(), 0..64),
        ys in prop::collection::vec(arb_iv(), 0..64),
        x in any::<u8>(),
    ) {
        let left: U8COSet = xs.into_iter().collect();
        let right: U8COSet = ys.into_iter().collect();
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
        let left: U8COSet = xs.into_iter().collect();
        let right: U8COSet = ys.into_iter().collect();

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
        let left: U8COSet = xs.into_iter().collect();
        let right: U8COSet = ys.into_iter().collect();

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
        let left: U8COSet = xs.into_iter().collect();
        let right: U8COSet = ys.into_iter().collect();

        let symmetric = left.symmetric_difference_with_set(&right);
        let intersection = left.intersection_with_set(&right);

        prop_assert!(
            symmetric.intersection_with_set(&intersection).is_empty()
        );
    }
}
