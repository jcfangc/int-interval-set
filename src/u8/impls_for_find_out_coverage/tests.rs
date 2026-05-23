use proptest::prelude::*;

use crate::{
    U8COSet,
    u8::test_support::{arb_iv, build, iv},
};

#[test]
fn empty_set_has_zero_covered_len() {
    let set = build([]);
    let query = iv(10, 20);

    assert_eq!(set.covered_len_of(query), 0);
    assert_eq!(set.uncovered_len_of(query), query.len());
    assert_eq!(set.coverage_ratio_of(query), 0.0);
}

#[test]
fn full_cover_has_full_covered_len() {
    let set = build([(10, 20)]);
    let query = iv(10, 20);

    assert_eq!(set.covered_len_of(query), 10);
    assert_eq!(set.uncovered_len_of(query), 0);
    assert_eq!(set.coverage_ratio_of(query), 1.0);
}

#[test]
fn partial_cover_inside_single_interval() {
    let set = build([(10, 20)]);
    let query = iv(15, 25);

    assert_eq!(set.covered_len_of(query), 5);
    assert_eq!(set.uncovered_len_of(query), 5);
    assert_eq!(set.coverage_ratio_of(query), 0.5);
}

#[test]
fn gap_only_query_has_zero_covered_len() {
    let set = build([(10, 20), (30, 40)]);
    let query = iv(20, 30);

    assert_eq!(set.covered_len_of(query), 0);
    assert_eq!(set.uncovered_len_of(query), query.len());
    assert_eq!(set.coverage_ratio_of(query), 0.0);
}

#[test]
fn query_across_multiple_intervals_sums_clipped_segments() {
    let set = build([(10, 20), (30, 40), (50, 60)]);
    let query = iv(15, 55);

    assert_eq!(
        set.intersections(query).collect::<Vec<_>>(),
        vec![iv(15, 20), iv(30, 40), iv(50, 55)]
    );
    assert_eq!(set.covered_len_of(query), 20);
    assert_eq!(set.uncovered_len_of(query), 20);
    assert_eq!(set.coverage_ratio_of(query), 0.5);
}

#[test]
fn coverage_is_computed_from_canonical_merged_intervals() {
    let set = build([(0, 5), (5, 10), (12, 20), (18, 30)]);
    let query = iv(5, 15);

    assert_eq!(set.as_slice(), &[iv(0, 10), iv(12, 30)]);
    assert_eq!(
        set.intersections(query).collect::<Vec<_>>(),
        vec![iv(5, 10), iv(12, 15)]
    );
    assert_eq!(set.covered_len_of(query), 8);
    assert_eq!(set.uncovered_len_of(query), 2);
    assert_eq!(set.coverage_ratio_of(query), 0.8);
}

#[test]
fn query_containing_all_intervals_sums_all_covered_segments() {
    let set = build([(10, 20), (30, 40)]);
    let query = iv(0, 50);

    assert_eq!(set.covered_len_of(query), 20);
    assert_eq!(set.uncovered_len_of(query), 30);
    assert_eq!(set.coverage_ratio_of(query), 0.4);
}

#[test]
fn coverage_handles_domain_edges() {
    let set = build([(u8::MIN, u8::MIN + 1), (u8::MAX - 1, u8::MAX)]);

    let left = iv(u8::MIN, u8::MIN + 2);
    assert_eq!(set.covered_len_of(left), 1);
    assert_eq!(set.uncovered_len_of(left), 1);
    assert_eq!(set.coverage_ratio_of(left), 0.5);

    let right = iv(u8::MAX - 2, u8::MAX);
    assert_eq!(set.covered_len_of(right), 1);
    assert_eq!(set.uncovered_len_of(right), 1);
    assert_eq!(set.coverage_ratio_of(right), 0.5);
}

#[test]
fn representative_queries_partition_into_covered_and_uncovered_lengths() {
    let set = build([(10, 20), (30, 40), (50, 60)]);

    for query in [
        iv(0, 10),
        iv(9, 11),
        iv(15, 35),
        iv(20, 30),
        iv(35, 55),
        iv(55, 61),
        iv(u8::MIN, u8::MIN + 1),
        iv(u8::MAX - 1, u8::MAX),
    ] {
        assert_eq!(
            set.covered_len_of(query) + set.uncovered_len_of(query),
            query.len(),
            "query = {query:?}"
        );
    }
}

#[test]
fn representative_ratios_match_covered_len_divided_by_query_len() {
    let set = build([(10, 20), (30, 40), (50, 60)]);

    for query in [
        iv(0, 10),
        iv(9, 11),
        iv(15, 35),
        iv(20, 30),
        iv(35, 55),
        iv(55, 61),
        iv(u8::MIN, u8::MIN + 1),
        iv(u8::MAX - 1, u8::MAX),
    ] {
        let expected = set.covered_len_of(query) as f32 / query.len() as f32;

        assert_eq!(set.coverage_ratio_of(query), expected, "query = {query:?}");
    }
}

proptest! {
    #[test]
    fn prop_lengths_partition_query(
        xs in prop::collection::vec(arb_iv(), 0..64),
        query in arb_iv(),
    ) {
        let set: U8COSet = xs.into_iter().collect();

        prop_assert_eq!(
            set.covered_len_of(query) + set.uncovered_len_of(query),
            query.len()
        );
    }

    #[test]
    fn prop_covered_len_is_positive_iff_query_intersects_set(
        xs in prop::collection::vec(arb_iv(), 0..64),
        query in arb_iv(),
    ) {
        let set: U8COSet = xs.into_iter().collect();

        prop_assert_eq!(
            set.covered_len_of(query) > 0,
            set.intersects_interval(query)
        );
    }

    #[test]
    fn prop_coverage_ratio_matches_covered_len_over_query_len(
        xs in prop::collection::vec(arb_iv(), 0..64),
        query in arb_iv(),
    ) {
        let set: U8COSet = xs.into_iter().collect();

        let expected = set.covered_len_of(query) as f32 / query.len() as f32;

        prop_assert_eq!(set.coverage_ratio_of(query), expected);
    }

    #[test]
    fn prop_covered_len_matches_sum_of_clipped_intersections(
        xs in prop::collection::vec(arb_iv(), 0..64),
        query in arb_iv(),
    ) {
        let set: U8COSet = xs.into_iter().collect();

        let expected: u8 = set.intersections(query).map(|iv| iv.len()).sum();

        prop_assert_eq!(set.covered_len_of(query), expected);
    }
}
