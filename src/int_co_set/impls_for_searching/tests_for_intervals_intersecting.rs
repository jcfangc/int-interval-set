use proptest::prelude::*;

use crate::{
    I8COSet,
    int_co_set::test_support::{arb_iv, build, iv},
};

#[test]
fn returns_empty_on_empty_set() {
    let set = build([]);
    let query = iv(0, 1);

    assert_eq!(
        set.intervals_intersecting(query).collect::<Vec<_>>(),
        vec![]
    );
}

#[test]
fn respects_half_open_bounds() {
    let set = build([(-20, -10), (10, 20)]);

    assert_eq!(
        set.intervals_intersecting(iv(-30, -20)).collect::<Vec<_>>(),
        vec![]
    );
    assert_eq!(
        set.intervals_intersecting(iv(-10, 10)).collect::<Vec<_>>(),
        vec![]
    );
    assert_eq!(
        set.intervals_intersecting(iv(20, 30)).collect::<Vec<_>>(),
        vec![]
    );

    assert_eq!(
        set.intervals_intersecting(iv(-21, -19)).collect::<Vec<_>>(),
        vec![iv(-20, -10)]
    );
    assert_eq!(
        set.intervals_intersecting(iv(19, 21)).collect::<Vec<_>>(),
        vec![iv(10, 20)]
    );
}

#[test]
fn returns_original_canonical_intervals() {
    let set = build([(-30, -20), (-10, 0), (10, 20)]);
    let query = iv(-25, 15);

    assert_eq!(
        set.intervals_intersecting(query).collect::<Vec<_>>(),
        vec![iv(-30, -20), iv(-10, 0), iv(10, 20)]
    );
}

#[test]
fn uses_canonical_merged_intervals() {
    let set = build([(-20, -10), (-10, 0), (5, 15), (10, 20)]);
    let query = iv(-5, 10);

    assert_eq!(set.as_slice(), &[iv(-20, 0), iv(5, 20)]);
    assert_eq!(
        set.intervals_intersecting(query).collect::<Vec<_>>(),
        vec![iv(-20, 0), iv(5, 20)]
    );
}

#[test]
fn returns_empty_for_gap_only_query() {
    let set = build([(-20, -10), (10, 20)]);
    let query = iv(-10, 10);

    assert_eq!(
        set.intervals_intersecting(query).collect::<Vec<_>>(),
        vec![]
    );
}

#[test]
fn handles_domain_edges() {
    let set = build([(i8::MIN, i8::MIN + 1), (i8::MAX - 1, i8::MAX)]);

    assert_eq!(
        set.intervals_intersecting(iv(i8::MIN, i8::MIN + 1))
            .collect::<Vec<_>>(),
        vec![iv(i8::MIN, i8::MIN + 1)]
    );
    assert_eq!(
        set.intervals_intersecting(iv(i8::MAX - 1, i8::MAX))
            .collect::<Vec<_>>(),
        vec![iv(i8::MAX - 1, i8::MAX)]
    );
}

#[test]
fn representative_queries_match_intersection_predicate() {
    let set = build([(-30, -20), (-10, 0), (10, 20)]);

    for query in [
        iv(i8::MIN, i8::MIN + 1),
        iv(-40, -30),
        iv(-31, -29),
        iv(-25, -5),
        iv(-20, -10),
        iv(-1, 11),
        iv(0, 10),
        iv(15, 21),
        iv(i8::MAX - 1, i8::MAX),
    ] {
        assert_eq!(
            set.intervals_intersecting(query).next().is_some(),
            set.intersects_interval(query),
            "query = {query:?}"
        );
    }
}

proptest! {
    #[test]
    fn prop_matches_slice_filter(
        xs in prop::collection::vec(arb_iv(), 0..64),
        query in arb_iv(),
    ) {
        let set: I8COSet = xs.into_iter().collect();

        let got = set.intervals_intersecting(query).collect::<Vec<_>>();
        let expected = set
            .as_slice()
            .iter()
            .copied()
            .filter(|iv| iv.intersects(query))
            .collect::<Vec<_>>();

        prop_assert_eq!(got, expected);
    }

    #[test]
    fn prop_non_empty_result_matches_predicate(
        xs in prop::collection::vec(arb_iv(), 0..64),
        query in arb_iv(),
    ) {
        let set: I8COSet = xs.into_iter().collect();

        prop_assert_eq!(
            set.intervals_intersecting(query).next().is_some(),
            set.intersects_interval(query)
        );
    }
}
