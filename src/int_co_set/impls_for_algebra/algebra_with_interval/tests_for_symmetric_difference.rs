use proptest::prelude::*;

use crate::{
    I8COSet,
    int_co_set::test_support::{arb_iv, build, iv},
};

#[test]
fn symmetric_difference_with_empty_set_returns_query() {
    let set = build([]);
    let query = iv(-10, 10);

    assert_eq!(
        set.symmetric_difference_with_interval(query).as_slice(),
        &[query]
    );
}

#[test]
fn symmetric_difference_with_disjoint_query_is_union() {
    let set = build([(-30, -20), (10, 20)]);
    let query = iv(-10, 0);

    assert_eq!(
        set.symmetric_difference_with_interval(query).as_slice(),
        &[iv(-30, -20), iv(-10, 0), iv(10, 20)]
    );
}

#[test]
fn symmetric_difference_merges_adjacent_disjoint_pieces_canonically() {
    let set = build([(-30, -20), (0, 10)]);
    let query = iv(-20, 0);

    assert_eq!(
        set.symmetric_difference_with_interval(query).as_slice(),
        &[iv(-30, 10)]
    );
}

#[test]
fn symmetric_difference_removes_equal_interval() {
    let set = build([(-10, 10)]);
    let query = iv(-10, 10);

    assert!(set.symmetric_difference_with_interval(query).is_empty());
}

#[test]
fn symmetric_difference_splits_partially_overlapping_interval() {
    let set = build([(-10, 10)]);
    let query = iv(0, 20);

    assert_eq!(
        set.symmetric_difference_with_interval(query).as_slice(),
        &[iv(-10, 0), iv(10, 20)]
    );
}

#[test]
fn symmetric_difference_preserves_query_gaps_between_multiple_hits() {
    let set = build([(-40, -30), (-10, 0), (20, 30)]);
    let query = iv(-35, 25);

    assert_eq!(
        set.symmetric_difference_with_interval(query).as_slice(),
        &[iv(-40, -35), iv(-30, -10), iv(0, 20), iv(25, 30)]
    );
}

#[test]
fn symmetric_difference_uses_canonical_source_intervals() {
    let set = build([(-20, -10), (-10, 0), (5, 15), (10, 20)]);
    let query = iv(-5, 10);

    assert_eq!(set.as_slice(), &[iv(-20, 0), iv(5, 20)]);
    assert_eq!(
        set.symmetric_difference_with_interval(query).as_slice(),
        &[iv(-20, -5), iv(0, 5), iv(10, 20)]
    );
}

#[test]
fn symmetric_difference_handles_domain_edges() {
    let set = build([(i8::MIN, i8::MIN + 10), (i8::MAX - 10, i8::MAX)]);

    let query = iv(i8::MIN + 5, i8::MAX - 5);

    assert_eq!(
        set.symmetric_difference_with_interval(query).as_slice(),
        &[
            iv(i8::MIN, i8::MIN + 5),
            iv(i8::MIN + 10, i8::MAX - 10),
            iv(i8::MAX - 5, i8::MAX),
        ]
    );
}

#[test]
fn symmetric_difference_retains_query_tail_when_query_contains_source_interval() {
    let set = build([(-20, 0)]);
    let query = iv(-20, 127);

    assert_eq!(
        set.symmetric_difference_with_interval(query).as_slice(),
        &[iv(0, 127)]
    );
}

#[test]
fn symmetric_difference_retains_query_head_when_query_extends_left_of_source_interval() {
    let set = build([(20, 40)]);
    let query = iv(10, 30);

    assert_eq!(
        set.symmetric_difference_with_interval(query).as_slice(),
        &[iv(10, 20), iv(30, 40)]
    );
}

#[test]
fn symmetric_difference_merges_query_residual_adjacent_to_retained_suffix() {
    let set = build([(-20, -15), (-10, -5)]);
    let query = iv(-18, -10);

    assert_eq!(
        set.symmetric_difference_with_interval(query).as_slice(),
        &[iv(-20, -18), iv(-15, -5)]
    );
}

#[test]
fn symmetric_difference_merges_query_residual_adjacent_to_retained_prefix() {
    let set = build([(-20, -15), (-10, -5)]);
    let query = iv(-15, -8);

    assert_eq!(
        set.symmetric_difference_with_interval(query).as_slice(),
        &[iv(-20, -10), iv(-8, -5)]
    );
}

proptest! {
    #[test]
    fn prop_symmetric_difference_matches_exclusive_membership(
        xs in prop::collection::vec(arb_iv(), 0..64),
        query in arb_iv(),
        x in any::<i8>(),
    ) {
        let set: I8COSet = xs.into_iter().collect();
        let result = set.symmetric_difference_with_interval(query);

        prop_assert_eq!(
            result.contains_point(x),
            set.contains_point(x) ^ query.contains(x)
        );
    }

    #[test]
    fn prop_symmetric_difference_with_disjoint_query_matches_union(
        xs in prop::collection::vec(arb_iv(), 0..64),
        query in arb_iv(),
    ) {
        let set: I8COSet = xs.into_iter().collect();

        if !set.intersects_interval(query) {
            prop_assert_eq!(
                set.symmetric_difference_with_interval(query),
                set.union_with_interval(query)
            );
        }
    }

    #[test]
    fn prop_symmetric_difference_is_disjoint_from_intersection(
        xs in prop::collection::vec(arb_iv(), 0..64),
        query in arb_iv(),
    ) {
        let set: I8COSet = xs.into_iter().collect();

        let symmetric = set.symmetric_difference_with_interval(query);
        let intersection = set.intersection_with_interval(query);

        for left in symmetric.iter_intervals() {
            for right in intersection.iter_intervals() {
                prop_assert!(!left.intersects(right));
            }
        }
    }
}
