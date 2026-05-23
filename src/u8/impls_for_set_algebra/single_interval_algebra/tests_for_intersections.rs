use proptest::prelude::*;

use crate::{
    U8COSet,
    u8::test_support::{arb_iv, build, iv},
};

#[test]
fn intersections_returns_empty_on_empty_set() {
    let set = build([]);
    let query = iv(0, 1);

    assert_eq!(set.intersections(query).collect::<Vec<_>>(), vec![]);
}

#[test]
fn intersections_respects_half_open_bounds() {
    let set = build([(10, 20), (30, 40)]);

    assert_eq!(set.intersections(iv(0, 10)).collect::<Vec<_>>(), vec![]);
    assert_eq!(set.intersections(iv(20, 30)).collect::<Vec<_>>(), vec![]);
    assert_eq!(set.intersections(iv(40, 50)).collect::<Vec<_>>(), vec![]);

    assert_eq!(
        set.intersections(iv(9, 11)).collect::<Vec<_>>(),
        vec![iv(10, 11)]
    );
    assert_eq!(
        set.intersections(iv(19, 21)).collect::<Vec<_>>(),
        vec![iv(19, 20)]
    );
}

#[test]
fn intersections_returns_clipped_segments() {
    let set = build([(10, 20), (30, 40), (50, 60)]);
    let query = iv(15, 55);

    assert_eq!(
        set.intersections(query).collect::<Vec<_>>(),
        vec![iv(15, 20), iv(30, 40), iv(50, 55)]
    );
}

#[test]
fn intersections_returns_full_intervals_when_query_contains_them() {
    let set = build([(10, 20), (30, 40)]);
    let query = iv(9, 41);

    assert_eq!(
        set.intersections(query).collect::<Vec<_>>(),
        vec![iv(10, 20), iv(30, 40)]
    );
}

#[test]
fn intersections_returns_query_when_fully_contained() {
    let set = build([(10, 40)]);
    let query = iv(15, 25);

    assert_eq!(set.intersections(query).collect::<Vec<_>>(), vec![query]);
}

#[test]
fn intersections_uses_canonical_merged_intervals() {
    let set = build([(0, 5), (5, 10), (12, 20), (18, 30)]);
    let query = iv(9, 13);

    assert_eq!(set.as_slice(), &[iv(0, 10), iv(12, 30)]);
    assert_eq!(
        set.intersections(query).collect::<Vec<_>>(),
        vec![iv(9, 10), iv(12, 13)]
    );
}

#[test]
fn intersections_returns_empty_for_gap_only_query() {
    let set = build([(10, 20), (30, 40)]);
    let query = iv(20, 30);

    assert_eq!(set.intersections(query).collect::<Vec<_>>(), vec![]);
}

#[test]
fn intersections_handles_domain_edges() {
    let set = build([(u8::MIN, u8::MIN + 1), (u8::MAX - 1, u8::MAX)]);

    assert_eq!(
        set.intersections(iv(u8::MIN, u8::MIN + 1))
            .collect::<Vec<_>>(),
        vec![iv(u8::MIN, u8::MIN + 1)]
    );
    assert_eq!(
        set.intersections(iv(u8::MAX - 1, u8::MAX))
            .collect::<Vec<_>>(),
        vec![iv(u8::MAX - 1, u8::MAX)]
    );
}

proptest! {
    #[test]
    fn prop_intersections_match_slice_filter_map(
        xs in prop::collection::vec(arb_iv(), 0..64),
        query in arb_iv(),
    ) {
        let set: U8COSet = xs.into_iter().collect();

        let got = set.intersections(query).collect::<Vec<_>>();
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
        let set: U8COSet = xs.into_iter().collect();

        let got = set.intersections(query).collect::<Vec<_>>();
        let expected = set
            .intervals_intersecting(query)
            .map(|iv| iv.intersection(query).unwrap())
            .collect::<Vec<_>>();

        prop_assert_eq!(got, expected);
    }
}
