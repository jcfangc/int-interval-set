use super::*;

mod prop_tests {
    use super::*;
    use proptest::prelude::*;

    #[cfg(test)]
    mod construction {
        use super::*;

        // ------------------------------------------------------------
        // strategies
        // ------------------------------------------------------------

        prop_compose! {
            fn point_strategy()(
                x in prop_oneof![
                    Just(i8::MIN),
                    Just(i8::MIN.saturating_add(1)),
                    Just(0i8),
                    Just(1i8),
                    Just(2i8),
                    Just(i8::MAX.saturating_sub(2)),
                    Just(i8::MAX.saturating_sub(1)),
                    Just(i8::MAX),
                    any::<i8>(),
                ]
            ) -> i8 {
                x
            }
        }

        prop_compose! {
            fn interval_strategy()(
                a in point_strategy(),
                b in point_strategy(),
            ) -> I8CO {
                let start = a.min(b);
                let end_excl = a.max(b).saturating_add(1);

                // end_excl 至少比 start 大 1；若 a.max(b) == MAX，则 saturating_add(1) == MAX，
                // 这时可能 start == end_excl，需要手动修正成一个合法非空区间。
                if let Some(iv) = I8CO::try_new(start, end_excl) {
                    iv
                } else {
                    // 唯一会落到这里的典型情形是 start == end_excl == MAX。
                    // 构造一个贴近上界的最小非空区间。
                    I8CO::try_new(i8::MAX - 1, i8::MAX).unwrap()
                }
            }
        }

        fn intervals_strategy() -> impl Strategy<Value = Vec<I8CO>> {
            prop::collection::vec(interval_strategy(), 0..48)
        }

        fn probe_points_strategy() -> impl Strategy<Value = Vec<i8>> {
            prop::collection::vec(point_strategy(), 0..96)
        }

        prop_compose! {
            fn interval_array_8_strategy()(
                xs in prop::array::uniform8(interval_strategy())
            ) -> [I8CO; 8] {
                xs
            }
        }

        // ------------------------------------------------------------
        // helpers
        // ------------------------------------------------------------

        #[inline]
        fn raw_contains_point(raw: &[I8CO], x: i8) -> bool {
            raw.iter().any(|iv| iv.contains(x))
        }

        #[inline]
        fn is_strictly_normalized(xs: &[I8CO]) -> bool {
            xs.windows(2).all(|w| {
                let a = w[0];
                let b = w[1];

                a.start() < b.start() && !a.is_contiguous_with(b)
            })
        }

        #[inline]
        fn sum_len(xs: &[I8CO]) -> u8 {
            xs.iter().fold(0u8, |acc, iv| acc.saturating_add(iv.len()))
        }

        fn enrich_probes(raw: &[I8CO], extra: &[i8]) -> Vec<i8> {
            #[inline]
            fn midpoint_i8(a: i8, b: i8) -> i8 {
                (a & b) + ((a ^ b) >> 1)
            }

            let mut out = Vec::with_capacity(extra.len() + raw.len() * 6 + 6);

            out.push(i8::MIN);
            out.push(i8::MIN.saturating_add(1));
            out.push(0);
            out.push(1);
            out.push(i8::MAX.saturating_sub(1));
            out.push(i8::MAX);

            out.extend_from_slice(extra);

            for &iv in raw {
                let s = iv.start();
                let e = iv.end_excl();
                let ei = iv.end_incl();

                out.push(s);
                out.push(ei);
                out.push(e);

                out.push(s.saturating_sub(1));
                out.push(ei.saturating_add(1));

                out.push(midpoint_i8(s, ei));
            }

            out.sort_unstable();
            out.dedup();
            out
        }

        // ------------------------------------------------------------
        // deterministic smoke tests
        // ------------------------------------------------------------

        #[test]
        fn empty_input_builds_empty_set() {
            let set = I8COBatchSet::default();
            assert!(set.as_slice().is_empty());
            assert_eq!(set.interval_count(), 0);
            assert_eq!(set.point_count(), 0);

            let set = I8COBatchSet::from(Vec::<I8CO>::new());
            assert!(set.as_slice().is_empty());
            assert_eq!(set.interval_count(), 0);
            assert_eq!(set.point_count(), 0);

            let set: I8COBatchSet = core::iter::empty::<I8CO>().collect();
            assert!(set.as_slice().is_empty());
            assert_eq!(set.interval_count(), 0);
            assert_eq!(set.point_count(), 0);
        }

        // ------------------------------------------------------------
        // main properties
        // ------------------------------------------------------------

        proptest! {
            #[test]
            fn prop_vec_constructor_normalizes_structure_and_preserves_point_membership(
                raw in intervals_strategy(),
                extra_probes in probe_points_strategy(),
            ) {
                let set = I8COBatchSet::from(raw.clone());

                prop_assert!(is_strictly_normalized(set.as_slice()));

                let probes = enrich_probes(&raw, &extra_probes);
                for x in probes {
                    prop_assert_eq!(
                        set.contains_point(x),
                        raw_contains_point(&raw, x),
                        "membership mismatch at point {:?}, raw={:?}, set={:?}",
                        x,
                        raw,
                        set,
                    );
                }
            }

            #[test]
            fn prop_vec_constructor_is_order_invariant(
                raw in intervals_strategy(),
            ) {
                let mut sorted = raw.clone();
                sorted.sort_unstable_by(|a, b| {
                    a.start()
                        .cmp(&b.start())
                        .then(a.end_excl().cmp(&b.end_excl()))
                });

                let a = I8COBatchSet::from(raw);
                let b = I8COBatchSet::from(sorted);

                prop_assert_eq!(a, b);
            }

            #[test]
            fn prop_interval_count_matches_internal_slice_len(
                raw in intervals_strategy(),
            ) {
                let set = I8COBatchSet::from(raw);

                prop_assert_eq!(set.interval_count(), set.as_slice().len() as i8);
            }

            #[test]
            fn prop_point_count_matches_sum_of_normalized_interval_lengths(
                raw in intervals_strategy(),
            ) {
                let set = I8COBatchSet::from(raw);

                prop_assert_eq!(set.point_count(), sum_len(set.as_slice()));
            }

            #[test]
            fn prop_from_iter_matches_vec_constructor(
                raw in intervals_strategy(),
            ) {
                let a = I8COBatchSet::from(raw.clone());
                let b: I8COBatchSet = raw.into_iter().collect();

                prop_assert_eq!(a, b);
            }

            #[test]
            fn prop_array_constructor_matches_vec_constructor(
                raw in interval_array_8_strategy(),
            ) {
                let a = I8COBatchSet::from(raw);
                let b = I8COBatchSet::from(Vec::from(raw));

                prop_assert_eq!(a, b);
            }
        }
    }
}
