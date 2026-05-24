use int_interval::traits::IntPrimitive;
use proptest::prelude::*;

use crate::{
    I8COSet,
    int_co_set::test_support::{arb_iv, build, iv},
};

#[test]
fn empty_set_has_zero_covered_len() {
    let set = build([]);
    let query = iv(-10, 10);

    assert_eq!(set.covered_len_of(query), 0);
    assert_eq!(set.uncovered_len_of(query), query.len());
    assert_eq!(set.coverage_ratio_f32_of(query), 0.0);
    assert_eq!(set.coverage_ratio_f64_of(query), 0.0);
}

#[test]
fn full_cover_has_full_covered_len() {
    let set = build([(-10, 10)]);
    let query = iv(-10, 10);

    assert_eq!(set.covered_len_of(query), 20);
    assert_eq!(set.uncovered_len_of(query), 0);
    assert_eq!(set.coverage_ratio_f32_of(query), 1.0);
    assert_eq!(set.coverage_ratio_f64_of(query), 1.0);
}

#[test]
fn partial_cover_inside_single_interval() {
    let set = build([(-10, 10)]);
    let query = iv(0, 20);

    assert_eq!(set.covered_len_of(query), 10);
    assert_eq!(set.uncovered_len_of(query), 10);
    assert_eq!(set.coverage_ratio_f32_of(query), 0.5);
    assert_eq!(set.coverage_ratio_f64_of(query), 0.5);
}

#[test]
fn gap_only_query_has_zero_covered_len() {
    let set = build([(-20, -10), (10, 20)]);
    let query = iv(-10, 10);

    assert_eq!(set.covered_len_of(query), 0);
    assert_eq!(set.uncovered_len_of(query), query.len());
    assert_eq!(set.coverage_ratio_f32_of(query), 0.0);
    assert_eq!(set.coverage_ratio_f64_of(query), 0.0);
}

#[test]
fn query_across_multiple_intervals_sums_clipped_segments() {
    let set = build([(-50, -40), (-20, -10), (10, 20)]);
    let query = iv(-45, 15);

    assert_eq!(
        set.intersection_with_interval(query).as_slice().to_vec(),
        vec![iv(-45, -40), iv(-20, -10), iv(10, 15)]
    );
    assert_eq!(set.covered_len_of(query), 20);
    assert_eq!(set.uncovered_len_of(query), 40);
    assert_eq!(set.coverage_ratio_f32_of(query), 20.0 / 60.0);
    assert_eq!(set.coverage_ratio_f64_of(query), 20.0 / 60.0);
}

#[test]
fn coverage_is_computed_from_canonical_merged_intervals() {
    let set = build([(-20, -10), (-10, 0), (5, 15), (10, 20)]);
    let query = iv(-5, 10);

    assert_eq!(set.as_slice(), &[iv(-20, 0), iv(5, 20)]);
    assert_eq!(
        set.intersection_with_interval(query).as_slice().to_vec(),
        vec![iv(-5, 0), iv(5, 10)]
    );
    assert_eq!(set.covered_len_of(query), 10);
    assert_eq!(set.uncovered_len_of(query), 5);
    assert_eq!(set.coverage_ratio_f32_of(query), 10.0 / 15.0);
    assert_eq!(set.coverage_ratio_f64_of(query), 10.0 / 15.0);
}

#[test]
fn query_containing_all_intervals_sums_all_covered_segments() {
    let set = build([(-40, -30), (10, 20)]);
    let query = iv(-50, 50);

    assert_eq!(set.covered_len_of(query), 20);
    assert_eq!(set.uncovered_len_of(query), 80);
    assert_eq!(set.coverage_ratio_f32_of(query), 0.2);
    assert_eq!(set.coverage_ratio_f64_of(query), 0.2);
}

#[test]
fn coverage_handles_domain_edges() {
    let set = build([(i8::MIN, i8::MIN + 1), (i8::MAX - 1, i8::MAX)]);

    let left = iv(i8::MIN, i8::MIN + 2);
    assert_eq!(set.covered_len_of(left), 1);
    assert_eq!(set.uncovered_len_of(left), 1);
    assert_eq!(set.coverage_ratio_f32_of(left), 0.5);
    assert_eq!(set.coverage_ratio_f64_of(left), 0.5);

    let right = iv(i8::MAX - 2, i8::MAX);
    assert_eq!(set.covered_len_of(right), 1);
    assert_eq!(set.uncovered_len_of(right), 1);
    assert_eq!(set.coverage_ratio_f32_of(right), 0.5);
    assert_eq!(set.coverage_ratio_f64_of(right), 0.5);
}

#[test]
fn representative_queries_partition_into_covered_and_uncovered_lengths() {
    let set = build([(-50, -40), (-20, -10), (10, 20)]);

    for query in [
        iv(-60, -50),
        iv(-51, -49),
        iv(-45, -15),
        iv(-10, 10),
        iv(-15, 15),
        iv(15, 21),
        iv(i8::MIN, i8::MIN + 1),
        iv(i8::MAX - 1, i8::MAX),
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
    let set = build([(-50, -40), (-20, -10), (10, 20)]);

    for query in [
        iv(-60, -50),
        iv(-51, -49),
        iv(-45, -15),
        iv(-10, 10),
        iv(-15, 15),
        iv(15, 21),
        iv(i8::MIN, i8::MIN + 1),
        iv(i8::MAX - 1, i8::MAX),
    ] {
        let expected_f32 = set.covered_len_of(query).as_f32() / query.len().as_f32();
        let expected_f64 = set.covered_len_of(query).as_f64() / query.len().as_f64();

        assert_eq!(
            set.coverage_ratio_f32_of(query),
            expected_f32,
            "query = {query:?}"
        );
        assert_eq!(
            set.coverage_ratio_f64_of(query),
            expected_f64,
            "query = {query:?}"
        );
    }
}

proptest! {
    #[test]
    fn prop_lengths_partition_query(
        xs in prop::collection::vec(arb_iv(), 0..64),
        query in arb_iv(),
    ) {
        let set: I8COSet = xs.into_iter().collect();

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
        let set: I8COSet = xs.into_iter().collect();

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
        let set: I8COSet = xs.into_iter().collect();

        let expected_f32 = set.covered_len_of(query).as_f32() / query.len().as_f32();
        let expected_f64 = set.covered_len_of(query).as_f64() / query.len().as_f64();

        prop_assert_eq!(set.coverage_ratio_f32_of(query), expected_f32);
        prop_assert_eq!(set.coverage_ratio_f64_of(query), expected_f64);
    }

    #[test]
    fn prop_covered_len_matches_sum_of_clipped_intersections(
        xs in prop::collection::vec(arb_iv(), 0..64),
        query in arb_iv(),
    ) {
        let set: I8COSet = xs.into_iter().collect();

        let expected = set
            .intersection_with_interval(query)
            .iter_intervals()
            .map(|iv| iv.len())
            .sum();

        prop_assert_eq!(set.covered_len_of(query), expected);
    }
}
