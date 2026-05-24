use proptest::prelude::*;

use crate::{
    I8COSet,
    int_co_set::test_support::{arb_iv, build, iv},
};

#[test]
fn difference_of_empty_set_is_empty() {
    let empty = build([]);
    let other = build([(10, 20)]);

    assert!(empty.difference_with_set(&other).is_empty());
}

#[test]
fn difference_with_empty_set_returns_source_set() {
    let set = build([(-20, -10), (10, 20)]);
    let empty = build([]);

    assert_eq!(set.difference_with_set(&empty), set);
}

#[test]
fn difference_with_disjoint_set_returns_source_content() {
    let left = build([(-20, -10), (10, 20)]);
    let right = build([(-30, -20), (-10, 10), (20, 30)]);

    assert_eq!(left.difference_with_set(&right), left);
}

#[test]
fn difference_removes_fully_covered_intervals() {
    let left = build([(-40, -30), (-20, -10), (10, 20)]);
    let right = build([(-45, -5), (5, 30)]);

    assert!(left.difference_with_set(&right).is_empty());
}

#[test]
fn difference_clips_boundary_intervals() {
    let left = build([(-40, -30), (-20, -10), (10, 20)]);
    let right = build([(-35, -15), (15, 30)]);

    assert_eq!(
        left.difference_with_set(&right).as_slice(),
        &[iv(-40, -35), iv(-15, -10), iv(10, 15)]
    );
}

#[test]
fn difference_splits_one_source_interval_multiple_times() {
    let left = build([(-60, 60)]);
    let right = build([(-50, -40), (-10, 10), (40, 50)]);

    assert_eq!(
        left.difference_with_set(&right).as_slice(),
        &[iv(-60, -50), iv(-40, -10), iv(10, 40), iv(50, 60)]
    );
}

#[test]
fn difference_reuses_long_removing_interval_across_source_intervals() {
    let left = build([(-50, -40), (-20, -10), (10, 20)]);
    let right = build([(-45, 15)]);

    assert_eq!(
        left.difference_with_set(&right).as_slice(),
        &[iv(-50, -45), iv(15, 20)]
    );
}

#[test]
fn difference_removes_identical_set() {
    let set = build([(-40, -30), (-10, 10), (30, 40)]);

    assert!(set.difference_with_set(&set).is_empty());
}

#[test]
fn difference_uses_canonical_source_intervals() {
    let left = build([(-30, -20), (-20, -10), (10, 15), (14, 20)]);
    let right = build([(-25, -15), (-15, 12)]);

    assert_eq!(left.as_slice(), &[iv(-30, -10), iv(10, 20)]);
    assert_eq!(right.as_slice(), &[iv(-25, 12)]);
    assert_eq!(
        left.difference_with_set(&right).as_slice(),
        &[iv(-30, -25), iv(12, 20)]
    );
}

#[test]
fn difference_handles_domain_edges() {
    let left = build([(i8::MIN, i8::MIN + 10), (i8::MAX - 10, i8::MAX)]);
    let right = build([(i8::MIN, i8::MIN + 5), (i8::MAX - 5, i8::MAX)]);

    assert_eq!(
        left.difference_with_set(&right).as_slice(),
        &[iv(i8::MIN + 5, i8::MIN + 10), iv(i8::MAX - 10, i8::MAX - 5)]
    );
}

proptest! {
    #[test]
    fn prop_difference_matches_pointwise_membership(
        xs in prop::collection::vec(arb_iv(), 0..64),
        ys in prop::collection::vec(arb_iv(), 0..64),
        x in any::<i8>(),
    ) {
        let left: I8COSet = xs.into_iter().collect();
        let right: I8COSet = ys.into_iter().collect();
        let result = left.difference_with_set(&right);

        prop_assert_eq!(
            result.contains_point(x),
            left.contains_point(x) && !right.contains_point(x)
        );
    }

    #[test]
    fn prop_difference_is_disjoint_from_removed_set(
        xs in prop::collection::vec(arb_iv(), 0..64),
        ys in prop::collection::vec(arb_iv(), 0..64),
    ) {
        let left: I8COSet = xs.into_iter().collect();
        let right: I8COSet = ys.into_iter().collect();
        let result = left.difference_with_set(&right);

        prop_assert!(
            result.intersection_with_set(&right).is_empty()
        );
    }

    #[test]
    fn prop_difference_is_contained_by_source_set(
        xs in prop::collection::vec(arb_iv(), 0..64),
        ys in prop::collection::vec(arb_iv(), 0..64),
    ) {
        let left: I8COSet = xs.into_iter().collect();
        let right: I8COSet = ys.into_iter().collect();
        let result = left.difference_with_set(&right);

        for interval in result.iter_intervals() {
            prop_assert!(left.contains_interval(interval));
        }
    }

    #[test]
    fn prop_difference_matches_repeated_interval_removal_reference_model(
        xs in prop::collection::vec(arb_iv(), 0..64),
        ys in prop::collection::vec(arb_iv(), 0..64),
    ) {
        let left: I8COSet = xs.into_iter().collect();
        let right: I8COSet = ys.into_iter().collect();

        let expected = right
            .iter_intervals()
            .fold(left.clone(), |set, interval| {
                set.difference_with_interval(interval)
            });

        prop_assert_eq!(left.difference_with_set(&right), expected);
    }
}
