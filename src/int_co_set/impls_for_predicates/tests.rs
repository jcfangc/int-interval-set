use proptest::prelude::*;

use crate::{
    I8COSet,
    int_co_set::test_support::{arb_iv, build, iv},
};

#[test]
fn contains_point_respects_half_open_bounds() {
    let set = build([(-10, 10)]);

    assert!(!set.contains_point(-11));
    assert!(set.contains_point(-10));
    assert!(set.contains_point(9));
    assert!(!set.contains_point(10));
}

#[test]
fn contains_point_works_across_multiple_intervals() {
    let set = build([(-60, -50), (-20, -10), (10, 20)]);

    assert!(set.contains_point(-60));
    assert!(set.contains_point(-15));
    assert!(set.contains_point(19));

    assert!(!set.contains_point(i8::MIN));
    assert!(!set.contains_point(-50));
    assert!(!set.contains_point(0));
    assert!(!set.contains_point(i8::MAX));
}

#[test]
fn contains_point_works_on_empty_set() {
    let set = build([]);

    assert!(!set.contains_point(i8::MIN));
    assert!(!set.contains_point(0));
    assert!(!set.contains_point(i8::MAX));
}

#[test]
fn contains_interval_accepts_fully_covered_query() {
    let set = build([(-20, -10), (10, 20)]);

    assert!(set.contains_interval(iv(-20, -10)));
    assert!(set.contains_interval(iv(-18, -12)));
    assert!(set.contains_interval(iv(10, 20)));
}

#[test]
fn contains_interval_rejects_partial_or_gap_crossing_query() {
    let set = build([(-20, -10), (10, 20)]);

    assert!(!set.contains_interval(iv(-21, -19)));
    assert!(!set.contains_interval(iv(-12, -8)));
    assert!(!set.contains_interval(iv(-10, 10)));
    assert!(!set.contains_interval(iv(-12, 12)));
    assert!(!set.contains_interval(iv(0, 15)));
}

#[test]
fn contains_interval_uses_canonical_merged_intervals() {
    let set = build([(-30, -20), (-20, -10), (0, 10), (8, 20)]);

    assert_eq!(set.as_slice(), &[iv(-30, -10), iv(0, 20)]);

    assert!(set.contains_interval(iv(-30, -10)));
    assert!(set.contains_interval(iv(0, 20)));
    assert!(set.contains_interval(iv(5, 15)));

    assert!(!set.contains_interval(iv(-15, 5)));
    assert!(!set.contains_interval(iv(-10, 0)));
}

#[test]
fn contains_interval_works_on_empty_set() {
    let set = build([]);

    assert!(!set.contains_interval(iv(-1, 1)));
    assert!(!set.contains_interval(iv(10, 11)));
    assert!(!set.contains_interval(iv(i8::MAX - 1, i8::MAX)));
}

#[test]
fn intersects_interval_accepts_overlap_on_left_middle_and_right() {
    let set = build([(-20, -10), (10, 20)]);

    assert!(set.intersects_interval(iv(-25, -19)));
    assert!(set.intersects_interval(iv(-18, -12)));
    assert!(set.intersects_interval(iv(-11, -5)));

    assert!(set.intersects_interval(iv(5, 11)));
    assert!(set.intersects_interval(iv(12, 18)));
    assert!(set.intersects_interval(iv(19, 25)));
}

#[test]
fn intersects_interval_rejects_adjacent_or_disjoint_query() {
    let set = build([(-20, -10), (10, 20)]);

    assert!(!set.intersects_interval(iv(-30, -20)));
    assert!(!set.intersects_interval(iv(-10, 10)));
    assert!(!set.intersects_interval(iv(20, 30)));
    assert!(!set.intersects_interval(iv(30, 40)));
}

#[test]
fn intersects_interval_accepts_query_spanning_gap() {
    let set = build([(-20, -10), (10, 20)]);

    assert!(set.intersects_interval(iv(-15, 15)));
    assert!(set.intersects_interval(iv(-21, 21)));
}

#[test]
fn intersects_interval_works_on_empty_set() {
    let set = build([]);

    assert!(!set.intersects_interval(iv(-1, 1)));
    assert!(!set.intersects_interval(iv(10, 11)));
    assert!(!set.intersects_interval(iv(i8::MAX - 1, i8::MAX)));
}

#[test]
fn predicates_handle_domain_edges() {
    let set = build([(i8::MIN, i8::MIN + 1), (i8::MAX - 1, i8::MAX)]);

    assert!(set.contains_point(i8::MIN));
    assert!(!set.contains_point(i8::MIN + 1));

    assert!(set.contains_point(i8::MAX - 1));
    assert!(!set.contains_point(i8::MAX));

    assert!(set.contains_interval(iv(i8::MIN, i8::MIN + 1)));
    assert!(set.contains_interval(iv(i8::MAX - 1, i8::MAX)));

    assert!(set.intersects_interval(iv(i8::MIN, i8::MIN + 1)));
    assert!(set.intersects_interval(iv(i8::MAX - 1, i8::MAX)));
}

proptest! {
    #[test]
    fn prop_contains_point_matches_point_search(
        xs in prop::collection::vec(arb_iv(), 0..64),
        x in any::<i8>(),
    ) {
        let set: I8COSet = xs.into_iter().collect();

        prop_assert_eq!(
            set.contains_point(x),
            set.interval_containing_point(x).is_some()
        );
    }

    #[test]
    fn prop_intersects_interval_matches_interval_search(
        xs in prop::collection::vec(arb_iv(), 0..64),
        query in arb_iv(),
    ) {
        let set: I8COSet = xs.into_iter().collect();

        prop_assert_eq!(
            set.intersects_interval(query),
            set.intervals_intersecting(query).next().is_some()
        );
    }

    #[test]
    fn prop_contains_interval_implies_intersection(
        xs in prop::collection::vec(arb_iv(), 0..64),
        query in arb_iv(),
    ) {
        let set: I8COSet = xs.into_iter().collect();

        if set.contains_interval(query) {
            prop_assert!(set.intersects_interval(query));
        }
    }

    #[test]
    fn prop_contains_interval_matches_single_covering_canonical_interval(
        xs in prop::collection::vec(arb_iv(), 0..64),
        query in arb_iv(),
    ) {
        let set: I8COSet = xs.into_iter().collect();

        let expected = set
            .as_slice()
            .iter()
            .copied()
            .any(|interval| interval.contains_interval(query));

        prop_assert_eq!(set.contains_interval(query), expected);
    }
}
