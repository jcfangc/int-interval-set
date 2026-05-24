use proptest::prelude::*;

use crate::{
    I8COSet,
    int_co_set::test_support::{arb_iv, build, iv},
};

#[test]
fn returns_none_on_empty_set() {
    let set = build([]);

    assert_eq!(set.interval_containing_point(i8::MIN), None);
    assert_eq!(set.interval_containing_point(0), None);
    assert_eq!(set.interval_containing_point(i8::MAX), None);
}

#[test]
fn respects_half_open_bounds() {
    let set = build([(-10, 10)]);

    assert_eq!(set.interval_containing_point(-11), None);
    assert_eq!(set.interval_containing_point(-10), Some(iv(-10, 10)));
    assert_eq!(set.interval_containing_point(0), Some(iv(-10, 10)));
    assert_eq!(set.interval_containing_point(9), Some(iv(-10, 10)));
    assert_eq!(set.interval_containing_point(10), None);
}

#[test]
fn returns_matching_interval_across_multiple_intervals() {
    let set = build([(-60, -50), (-20, 0), (30, 40)]);

    assert_eq!(set.interval_containing_point(-60), Some(iv(-60, -50)));
    assert_eq!(set.interval_containing_point(-10), Some(iv(-20, 0)));
    assert_eq!(set.interval_containing_point(35), Some(iv(30, 40)));

    assert_eq!(set.interval_containing_point(i8::MIN), None);
    assert_eq!(set.interval_containing_point(-50), None);
    assert_eq!(set.interval_containing_point(0), None);
    assert_eq!(set.interval_containing_point(20), None);
    assert_eq!(set.interval_containing_point(i8::MAX), None);
}

#[test]
fn returns_merged_interval_after_canonicalization() {
    let set = build([(-20, -10), (-10, 0), (5, 15), (10, 30)]);

    assert_eq!(set.as_slice(), &[iv(-20, 0), iv(5, 30)]);

    assert_eq!(set.interval_containing_point(-20), Some(iv(-20, 0)));
    assert_eq!(set.interval_containing_point(-1), Some(iv(-20, 0)));
    assert_eq!(set.interval_containing_point(0), None);
    assert_eq!(set.interval_containing_point(4), None);
    assert_eq!(set.interval_containing_point(5), Some(iv(5, 30)));
    assert_eq!(set.interval_containing_point(29), Some(iv(5, 30)));
    assert_eq!(set.interval_containing_point(30), None);
}

#[test]
fn handles_domain_edges() {
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
fn representative_points_match_contains_point_predicate() {
    let set = build([(-20, -10), (0, 10), (30, 40)]);

    for x in [i8::MIN, -21, -20, -11, -10, 0, 9, 10, 35, 40, i8::MAX] {
        assert_eq!(
            set.interval_containing_point(x).is_some(),
            set.contains_point(x),
            "point = {x}"
        );
    }
}

proptest! {
    #[test]
    fn prop_interval_containing_point_matches_slice_find(
        xs in prop::collection::vec(arb_iv(), 0..64),
        x in any::<i8>(),
    ) {
        let set: I8COSet = xs.into_iter().collect();

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
        xs in prop::collection::vec(arb_iv(), 0..64),
        x in any::<i8>(),
    ) {
        let set: I8COSet = xs.into_iter().collect();

        prop_assert_eq!(
            set.interval_containing_point(x).is_some(),
            set.contains_point(x)
        );
    }
}
