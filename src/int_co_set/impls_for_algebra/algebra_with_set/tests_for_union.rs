use proptest::prelude::*;

use crate::{
    I8COSet,
    int_co_set::test_support::{arb_iv, build, iv},
};

#[test]
fn union_with_empty_set_returns_other_set() {
    let empty = build([]);
    let set = build([(-20, -10), (10, 20)]);

    assert_eq!(empty.union_with_set(&set), set);
    assert_eq!(set.union_with_set(&empty), set);
}

#[test]
fn union_of_disjoint_sets_preserves_all_intervals_in_order() {
    let left = build([(-80, -70), (10, 20)]);
    let right = build([(-40, -30), (50, 60)]);

    assert_eq!(
        left.union_with_set(&right).as_slice(),
        &[iv(-80, -70), iv(-40, -30), iv(10, 20), iv(50, 60)]
    );
}

#[test]
fn union_merges_overlapping_intervals_from_both_sets() {
    let left = build([(-50, -40), (-20, -10), (20, 30)]);
    let right = build([(-45, -15), (25, 40)]);

    assert_eq!(
        left.union_with_set(&right).as_slice(),
        &[iv(-50, -10), iv(20, 40)]
    );
}

#[test]
fn union_merges_adjacent_intervals_across_sets() {
    let left = build([(-40, -30), (0, 10)]);
    let right = build([(-30, -20), (-20, 0)]);

    assert_eq!(left.union_with_set(&right).as_slice(), &[iv(-40, 10)]);
}

#[test]
fn union_bridges_multiple_intervals_from_both_sets() {
    let left = build([(-60, -50), (-20, -10), (20, 30)]);
    let right = build([(-50, -20), (-10, 20)]);

    assert_eq!(left.union_with_set(&right).as_slice(), &[iv(-60, 30)]);
}

#[test]
fn union_returns_covering_set_when_other_is_fully_contained() {
    let left = build([(-50, 50)]);
    let right = build([(-30, -20), (10, 20)]);

    assert_eq!(left.union_with_set(&right), left);
    assert_eq!(right.union_with_set(&left), left);
}

#[test]
fn union_uses_canonical_source_intervals() {
    let left = build([(-40, -30), (-30, -20), (20, 30)]);
    let right = build([(-25, 0), (0, 20)]);

    assert_eq!(left.as_slice(), &[iv(-40, -20), iv(20, 30)]);
    assert_eq!(right.as_slice(), &[iv(-25, 20)]);
    assert_eq!(left.union_with_set(&right).as_slice(), &[iv(-40, 30)]);
}

#[test]
fn union_handles_domain_edges() {
    let left = build([(i8::MIN, i8::MIN + 10), (i8::MAX - 10, i8::MAX)]);
    let right = build([(i8::MIN + 10, i8::MAX - 10)]);

    assert_eq!(
        left.union_with_set(&right).as_slice(),
        &[iv(i8::MIN, i8::MAX)]
    );
}

proptest! {
    #[test]
    fn prop_union_matches_pointwise_membership(
        xs in prop::collection::vec(arb_iv(), 0..64),
        ys in prop::collection::vec(arb_iv(), 0..64),
        x in any::<i8>(),
    ) {
        let left: I8COSet = xs.into_iter().collect();
        let right: I8COSet = ys.into_iter().collect();
        let result = left.union_with_set(&right);

        prop_assert_eq!(
            result.contains_point(x),
            left.contains_point(x) || right.contains_point(x)
        );
    }

    #[test]
    fn prop_union_is_commutative(
        xs in prop::collection::vec(arb_iv(), 0..64),
        ys in prop::collection::vec(arb_iv(), 0..64),
    ) {
        let left: I8COSet = xs.into_iter().collect();
        let right: I8COSet = ys.into_iter().collect();

        prop_assert_eq!(
            left.union_with_set(&right),
            right.union_with_set(&left)
        );
    }

    #[test]
    fn prop_union_contains_both_inputs(
        xs in prop::collection::vec(arb_iv(), 0..64),
        ys in prop::collection::vec(arb_iv(), 0..64),
    ) {
        let left: I8COSet = xs.into_iter().collect();
        let right: I8COSet = ys.into_iter().collect();
        let result = left.union_with_set(&right);

        for interval in left.iter_intervals() {
            prop_assert!(result.contains_interval(interval));
        }

        for interval in right.iter_intervals() {
            prop_assert!(result.contains_interval(interval));
        }
    }

    #[test]
    fn prop_union_matches_collect_reference_model(
        xs in prop::collection::vec(arb_iv(), 0..64),
        ys in prop::collection::vec(arb_iv(), 0..64),
    ) {
        let left: I8COSet = xs.into_iter().collect();
        let right: I8COSet = ys.into_iter().collect();

        let expected: I8COSet = left
            .iter_intervals()
            .chain(right.iter_intervals())
            .collect();

        prop_assert_eq!(left.union_with_set(&right), expected);
    }
}
