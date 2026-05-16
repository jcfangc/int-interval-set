use crate::i8::tests::{
    set::test_support::build,
    test_support::{MID_VALUE, build_from_vec, interval_pair, iv},
};

#[test]
fn search_point_returns_none_on_empty_set() {
    let set = build([]);

    assert_eq!(set.interval_containing_point(i8::MIN), None);
    assert_eq!(set.interval_containing_point(MID_VALUE), None);
    assert_eq!(set.interval_containing_point(i8::MAX), None);
}

#[test]
fn search_point_respects_half_open_bounds() {
    let set = build([(10, 20)]);

    assert_eq!(set.interval_containing_point(9), None);
    assert_eq!(set.interval_containing_point(10), Some(iv(10, 20)));
    assert_eq!(set.interval_containing_point(19), Some(iv(10, 20)));
    assert_eq!(set.interval_containing_point(20), None);
}

#[test]
fn search_point_returns_matching_interval_across_multiple_intervals() {
    let set = build([(-50, -40), (-10, 10), (30, 40)]);

    assert_eq!(set.interval_containing_point(-50), Some(iv(-50, -40)));
    assert_eq!(set.interval_containing_point(0), Some(iv(-10, 10)));
    assert_eq!(set.interval_containing_point(39), Some(iv(30, 40)));

    assert_eq!(set.interval_containing_point(-51), None);
    assert_eq!(set.interval_containing_point(-40), None);
    assert_eq!(set.interval_containing_point(10), None);
    assert_eq!(set.interval_containing_point(i8::MAX), None);
}

#[test]
fn search_point_returns_merged_interval_after_canonicalization() {
    let set = build([(-20, -10), (-10, 0), (5, 15), (10, 20)]);

    assert_eq!(set.as_slice(), &[iv(-20, 0), iv(5, 20)]);

    assert_eq!(set.interval_containing_point(-20), Some(iv(-20, 0)));
    assert_eq!(set.interval_containing_point(-1), Some(iv(-20, 0)));
    assert_eq!(set.interval_containing_point(0), None);
    assert_eq!(set.interval_containing_point(4), None);
    assert_eq!(set.interval_containing_point(5), Some(iv(5, 20)));
    assert_eq!(set.interval_containing_point(19), Some(iv(5, 20)));
    assert_eq!(set.interval_containing_point(20), None);
}

#[test]
fn search_point_handles_domain_edges() {
    let set = build([(i8::MIN, i8::MIN + 1), (i8::MAX - 1, i8::MAX)]);

    assert_eq!(
        set.interval_containing_point(i8::MIN),
        Some(iv(i8::MIN, i8::MIN + 1))
    );
    assert_eq!(set.interval_containing_point(i8::MIN + 1), None);

    assert_eq!(
        set.interval_containing_point(i8::MAX - 1),
        Some(iv(i8::MAX - 1, i8::MAX))
    );
    assert_eq!(set.interval_containing_point(i8::MAX), None);
}

#[test]
fn search_point_matches_contains_point_predicate() {
    let set = build([(10, 20), (30, 40), (50, 60)]);

    for x in [i8::MIN, 9, 10, 19, 20, 30, 39, 40, 55, 60, i8::MAX] {
        assert_eq!(
            set.interval_containing_point(x).is_some(),
            set.contains_point(x),
            "point = {x}"
        );
    }
}

use proptest::prelude::*;

proptest! {
    #[test]
    fn prop_interval_containing_point_matches_slice_find(
        xs in prop::collection::vec(interval_pair(), 0..64),
        x in any::<i8>(),
    ) {
        let set = build_from_vec(xs);

        let got = set.interval_containing_point(x);

        let expected = set
            .as_slice()
            .iter()
            .copied()
            .find(|iv| iv.contains(x));

        prop_assert_eq!(got, expected);
    }

    #[test]
    fn prop_interval_containing_point_matches_contains_point(
        xs in prop::collection::vec(interval_pair(), 0..64),
        x in any::<i8>(),
    ) {
        let set = build_from_vec(xs);

        prop_assert_eq!(
            set.interval_containing_point(x).is_some(),
            set.contains_point(x)
        );
    }
}
