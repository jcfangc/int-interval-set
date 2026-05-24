use proptest::prelude::*;

use crate::{
    I8COSet,
    int_co_set::test_support::{arb_iv, build, iv},
};

#[test]
fn intersections_returns_empty_on_empty_set() {
    let set = build([]);
    let query = iv(0, 1);

    assert_eq!(
        set.intersection_with_interval(query).as_slice().to_vec(),
        vec![]
    );
}

#[test]
fn intersections_respects_half_open_bounds() {
    let set = build([(-20, -10), (10, 20)]);

    assert_eq!(
        set.intersection_with_interval(iv(-30, -20))
            .as_slice()
            .to_vec(),
        vec![]
    );
    assert_eq!(
        set.intersection_with_interval(iv(-10, 10))
            .as_slice()
            .to_vec(),
        vec![]
    );
    assert_eq!(
        set.intersection_with_interval(iv(20, 30))
            .as_slice()
            .to_vec(),
        vec![]
    );

    assert_eq!(
        set.intersection_with_interval(iv(-21, -19))
            .as_slice()
            .to_vec(),
        vec![iv(-20, -19)]
    );
    assert_eq!(
        set.intersection_with_interval(iv(19, 21))
            .as_slice()
            .to_vec(),
        vec![iv(19, 20)]
    );
}

#[test]
fn intersections_returns_clipped_segments() {
    let set = build([(-30, -20), (-10, 0), (10, 20)]);
    let query = iv(-25, 15);

    assert_eq!(
        set.intersection_with_interval(query).as_slice().to_vec(),
        vec![iv(-25, -20), iv(-10, 0), iv(10, 15)]
    );
}

#[test]
fn intersections_returns_full_intervals_when_query_contains_them() {
    let set = build([(-20, -10), (10, 20)]);
    let query = iv(-21, 21);

    assert_eq!(
        set.intersection_with_interval(query).as_slice().to_vec(),
        vec![iv(-20, -10), iv(10, 20)]
    );
}

#[test]
fn intersections_returns_query_when_fully_contained() {
    let set = build([(-20, 20)]);
    let query = iv(-5, 5);

    assert_eq!(
        set.intersection_with_interval(query).as_slice().to_vec(),
        vec![query]
    );
}

#[test]
fn intersections_uses_canonical_merged_intervals() {
    let set = build([(-20, -10), (-10, 0), (5, 15), (10, 20)]);
    let query = iv(-5, 10);

    assert_eq!(set.as_slice(), &[iv(-20, 0), iv(5, 20)]);
    assert_eq!(
        set.intersection_with_interval(query).as_slice().to_vec(),
        vec![iv(-5, 0), iv(5, 10)]
    );
}

#[test]
fn intersections_returns_empty_for_gap_only_query() {
    let set = build([(-20, -10), (10, 20)]);
    let query = iv(-10, 10);

    assert_eq!(
        set.intersection_with_interval(query).as_slice().to_vec(),
        vec![]
    );
}

#[test]
fn intersections_handles_domain_edges() {
    let set = build([(i8::MIN, i8::MIN + 1), (i8::MAX - 1, i8::MAX)]);

    assert_eq!(
        set.intersection_with_interval(iv(i8::MIN, i8::MIN + 1))
            .as_slice()
            .to_vec(),
        vec![iv(i8::MIN, i8::MIN + 1)]
    );
    assert_eq!(
        set.intersection_with_interval(iv(i8::MAX - 1, i8::MAX))
            .as_slice()
            .to_vec(),
        vec![iv(i8::MAX - 1, i8::MAX)]
    );
}

proptest! {
    #[test]
    fn prop_intersections_match_slice_filter_map(
        xs in prop::collection::vec(arb_iv(), 0..64),
        query in arb_iv(),
    ) {
        let set: I8COSet = xs.into_iter().collect();

        let got = set.intersection_with_interval(query).as_slice().to_vec();
        let expected = set
            .as_slice()
            .iter()
            .copied()
            .filter_map(|iv| iv.intersection(query))
            .collect::<Vec<_>>();

        prop_assert_eq!(got, expected);
    }

    #[test]
    fn prop_intersections_are_clipped_from_intersecting_intervals(
        xs in prop::collection::vec(arb_iv(), 0..64),
        query in arb_iv(),
    ) {
        let set: I8COSet = xs.into_iter().collect();

        let got = set.intersection_with_interval(query).as_slice().to_vec();
        let expected = set
            .intervals_intersecting(query)
            .map(|iv| iv.intersection(query).unwrap())
            .collect::<Vec<_>>();

        prop_assert_eq!(got, expected);
    }
}
