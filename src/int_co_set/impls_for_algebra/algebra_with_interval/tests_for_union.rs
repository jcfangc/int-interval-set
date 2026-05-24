use proptest::prelude::*;

use crate::{
    I8COSet,
    int_co_set::test_support::{arb_iv, build, iv},
};

#[test]
fn union_inserts_into_empty_set() {
    let set = build([]);
    let query = iv(-10, 10);

    assert_eq!(set.union_with_interval(query).as_slice(), &[query]);
}

#[test]
fn union_merges_intersecting_interval() {
    let set = build([(-20, -10), (10, 20)]);
    let query = iv(-15, 0);

    assert_eq!(
        set.union_with_interval(query).as_slice(),
        &[iv(-20, 0), iv(10, 20)]
    );
}

#[test]
fn union_merges_adjacent_intervals() {
    let set = build([(-20, -10), (10, 20)]);
    let query = iv(-10, 10);

    assert_eq!(set.union_with_interval(query).as_slice(), &[iv(-20, 20)]);
}

#[test]
fn union_inserts_disjoint_interval_without_removing_existing_intervals() {
    let set = build([(-20, -10), (10, 20)]);
    let query = iv(-5, 5);

    assert_eq!(
        set.union_with_interval(query).as_slice(),
        &[iv(-20, -10), iv(-5, 5), iv(10, 20)]
    );
}

#[test]
fn union_bridges_multiple_intervals() {
    let set = build([(-60, -50), (-40, -30), (-20, -10), (10, 20)]);
    let query = iv(-50, 10);

    assert_eq!(set.union_with_interval(query).as_slice(), &[iv(-60, 20)]);
}

#[test]
fn union_returns_same_set_when_query_is_contained() {
    let set = build([(-20, -10), (10, 20)]);
    let query = iv(-18, -12);

    assert_eq!(set.union_with_interval(query), set);
}

#[test]
fn union_handles_domain_edges() {
    let set = build([(i8::MIN, i8::MIN + 1), (i8::MAX - 1, i8::MAX)]);
    let query = iv(i8::MIN + 1, i8::MAX - 1);

    assert_eq!(
        set.union_with_interval(query).as_slice(),
        &[iv(i8::MIN, i8::MAX)]
    );
}

proptest! {
    #[test]
    fn prop_union_matches_insert_then_canonicalize_reference_model(
        xs in prop::collection::vec(arb_iv(), 0..64),
        query in arb_iv(),
    ) {
        let set: I8COSet = xs.into_iter().collect();

        let expected: I8COSet = set
            .iter_intervals()
            .chain(std::iter::once(query))
            .collect();

        prop_assert_eq!(set.union_with_interval(query), expected);
    }

    #[test]
    fn prop_union_contains_query_and_all_source_intervals(
        xs in prop::collection::vec(arb_iv(), 0..64),
        query in arb_iv(),
    ) {
        let set: I8COSet = xs.into_iter().collect();
        let result = set.union_with_interval(query);

        prop_assert!(result.contains_interval(query));

        for interval in set.iter_intervals() {
            prop_assert!(result.contains_interval(interval));
        }
    }
}
