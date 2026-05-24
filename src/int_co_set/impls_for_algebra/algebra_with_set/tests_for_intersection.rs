use proptest::prelude::*;

use crate::{
    I8COSet,
    int_co_set::test_support::{arb_iv, build, iv},
};

#[test]
fn intersection_with_empty_set_is_empty() {
    let empty = build([]);
    let set = build([(-20, -10), (10, 20)]);

    assert!(empty.intersection_with_set(&set).is_empty());
    assert!(set.intersection_with_set(&empty).is_empty());
}

#[test]
fn intersection_of_disjoint_sets_is_empty() {
    let left = build([(-20, -10), (10, 20)]);
    let right = build([(-30, -20), (-10, 10), (20, 30)]);

    assert!(left.intersection_with_set(&right).is_empty());
}

#[test]
fn intersection_returns_clipped_segments_from_both_sets() {
    let left = build([(-50, -40), (-30, -20), (10, 20)]);
    let right = build([(-45, -25), (15, 30)]);

    assert_eq!(
        left.intersection_with_set(&right).as_slice(),
        &[iv(-45, -40), iv(-30, -25), iv(15, 20)]
    );
}

#[test]
fn intersection_handles_one_interval_overlapping_multiple_intervals() {
    let left = build([(-45, 45)]);
    let right = build([(-50, -40), (-30, -20), (20, 30), (40, 50)]);

    assert_eq!(
        left.intersection_with_set(&right).as_slice(),
        &[iv(-45, -40), iv(-30, -20), iv(20, 30), iv(40, 45)]
    );
}

#[test]
fn intersection_preserves_fully_shared_intervals() {
    let left = build([(-50, -40), (-10, 10), (40, 50)]);
    let right = build([(-50, -40), (-10, 10), (40, 50)]);

    assert_eq!(left.intersection_with_set(&right), left);
}

#[test]
fn intersection_uses_canonical_source_intervals() {
    let left = build([(-20, -15), (-15, -10), (10, 15), (14, 20)]);
    let right = build([(-12, 12)]);

    assert_eq!(left.as_slice(), &[iv(-20, -10), iv(10, 20)]);
    assert_eq!(
        left.intersection_with_set(&right).as_slice(),
        &[iv(-12, -10), iv(10, 12)]
    );
}

#[test]
fn intersection_handles_domain_edges() {
    let left = build([(i8::MIN, i8::MIN + 10), (i8::MAX - 10, i8::MAX)]);
    let right = build([(i8::MIN + 5, i8::MIN + 20), (i8::MAX - 5, i8::MAX)]);

    assert_eq!(
        left.intersection_with_set(&right).as_slice(),
        &[iv(i8::MIN + 5, i8::MIN + 10), iv(i8::MAX - 5, i8::MAX)]
    );
}

proptest! {
    #[test]
    fn prop_intersection_matches_pointwise_membership(
        xs in prop::collection::vec(arb_iv(), 0..64),
        ys in prop::collection::vec(arb_iv(), 0..64),
        x in any::<i8>(),
    ) {
        let left: I8COSet = xs.into_iter().collect();
        let right: I8COSet = ys.into_iter().collect();
        let result = left.intersection_with_set(&right);

        prop_assert_eq!(
            result.contains_point(x),
            left.contains_point(x) && right.contains_point(x)
        );
    }

    #[test]
    fn prop_intersection_is_commutative(
        xs in prop::collection::vec(arb_iv(), 0..64),
        ys in prop::collection::vec(arb_iv(), 0..64),
    ) {
        let left: I8COSet = xs.into_iter().collect();
        let right: I8COSet = ys.into_iter().collect();

        prop_assert_eq!(
            left.intersection_with_set(&right),
            right.intersection_with_set(&left)
        );
    }

    #[test]
    fn prop_intersection_output_is_contained_by_both_inputs(
        xs in prop::collection::vec(arb_iv(), 0..64),
        ys in prop::collection::vec(arb_iv(), 0..64),
    ) {
        let left: I8COSet = xs.into_iter().collect();
        let right: I8COSet = ys.into_iter().collect();
        let result = left.intersection_with_set(&right);

        for interval in result.iter_intervals() {
            prop_assert!(left.contains_interval(interval));
            prop_assert!(right.contains_interval(interval));
        }
    }
}
