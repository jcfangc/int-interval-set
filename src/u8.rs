#[cfg(test)]
mod tests;

use std::sync::Arc;

use int_interval::U8CO;

/// Read-only canonical interval set for `U8CO`.
///
/// A `U8COSet` stores a normalized collection of half-open `U8CO` intervals:
///
/// - intervals are sorted by `start`;
/// - intervals are non-overlapping;
/// - adjacent intervals are merged;
/// - all queries are performed against the sealed immutable array.
///
/// Construction is intentionally restricted. Use `U8COSetBuilder` for normal
/// construction.
pub mod set {
    use super::*;

    /// Immutable canonical interval set.
    ///
    /// Internally this is an `Arc<[U8CO]>`, so cloning a `U8COSet` is cheap.
    ///
    /// Canonical invariant:
    ///
    /// ```text
    /// for every adjacent pair a, b:
    ///     a.end_excl() < b.start()
    /// ```
    ///
    /// The strict `<` means both overlap and adjacency have already been merged.
    #[repr(transparent)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct U8COSet {
        intervals: Arc<[U8CO]>,
    }

    // ------------------------------------------------------------
    // basic api: construction / accessors
    // ------------------------------------------------------------

    mod basic {
        use super::*;

        impl U8COSet {
            /// Builds a set from an already canonical interval vector.
            ///
            /// # Safety
            ///
            /// The caller must guarantee that `intervals` is canonical:
            ///
            /// - intervals are sorted by ascending `start`;
            /// - intervals are non-overlapping;
            /// - contiguous intervals have already been merged;
            /// - therefore, for every adjacent pair `a, b`,
            ///   `a.end_excl() < b.start()` holds.
            ///
            /// Violating this invariant can make binary-search based queries
            /// return incorrect results.
            #[inline]
            pub(in crate::u8) unsafe fn new_unchecked(intervals: Vec<U8CO>) -> Self {
                debug_assert!(Self::is_canonical(&intervals));

                Self {
                    intervals: Arc::from(intervals.into_boxed_slice()),
                }
            }

            /// Checks the canonical invariant used by binary-search queries.
            ///
            /// `U8CO` itself already guarantees single-interval validity. This
            /// function only checks the relationship between adjacent intervals.
            #[inline]
            fn is_canonical(intervals: &[U8CO]) -> bool {
                intervals.windows(2).all(|w| w[0].end_excl() < w[1].start())
            }
        }

        impl U8COSet {
            /// Returns the number of canonical intervals.
            ///
            /// For the `U8CO` domain, the maximum canonical interval count is
            /// 128, e.g. `[0, 1), [2, 3), ..., [254, 255)`.
            #[inline]
            pub fn interval_count(&self) -> u8 {
                self.intervals.len() as u8
            }

            /// Returns whether the set contains no intervals.
            #[inline]
            pub fn is_empty(&self) -> bool {
                self.intervals.is_empty()
            }

            /// Returns the canonical interval slice.
            ///
            /// The returned slice is sorted, non-overlapping, and contains no
            /// adjacent intervals.
            #[inline]
            pub fn as_slice(&self) -> &[U8CO] {
                &self.intervals
            }

            /// Iterates over canonical intervals by value.
            #[inline]
            pub fn iter_intervals(&self) -> impl Iterator<Item = U8CO> {
                self.intervals.iter().copied()
            }
        }
    }

    // ------------------------------------------------------------
    // predicate api: yes/no queries
    // ------------------------------------------------------------

    mod predicates {
        use super::*;

        impl U8COSet {
            /// Returns whether `x` is covered by any interval in the set.
            ///
            /// Complexity: `O(log n)`.
            #[inline]
            pub fn contains_point(&self, x: u8) -> bool {
                let i = self.intervals.partition_point(|iv| iv.start() <= x);
                i != 0 && self.intervals[i - 1].contains(x)
            }

            /// Returns whether `query` is fully contained by one interval.
            ///
            /// Since the set is canonical, a contained query interval can only
            /// be contained by the interval immediately preceding or starting
            /// at `query.start()`.
            ///
            /// Complexity: `O(log n)`.
            #[inline]
            pub fn contains_interval(&self, query: U8CO) -> bool {
                let i = self
                    .intervals
                    .partition_point(|iv| iv.start() <= query.start());

                i != 0 && self.intervals[i - 1].contains_interval(query)
            }

            /// Returns whether `query` intersects any interval in the set.
            ///
            /// Complexity: `O(log n)`.
            #[inline]
            pub fn intersects_interval(&self, query: U8CO) -> bool {
                let i = self
                    .intervals
                    .partition_point(|iv| iv.end_excl() <= query.start());

                self.intervals.get(i).is_some_and(|iv| iv.intersects(query))
            }
        }
    }

    // ------------------------------------------------------------
    // search api: returning matched intervals
    // ------------------------------------------------------------

    mod search {
        use super::*;

        /// Point-based search APIs.
        mod point {
            use super::*;

            impl U8COSet {
                /// Returns the unique interval containing `x`, if any.
                ///
                /// Because the set is canonical, at most one interval can
                /// contain a single point.
                ///
                /// Complexity: `O(log n)`.
                #[inline]
                pub fn interval_containing_point(&self, x: u8) -> Option<U8CO> {
                    let i = self.intervals.partition_point(|iv| iv.start() <= x);

                    if i == 0 {
                        return None;
                    }

                    let iv = self.intervals[i - 1];
                    iv.contains(x).then_some(iv)
                }
            }
        }

        /// Interval-based search APIs.
        mod interval {
            use super::*;

            impl U8COSet {
                /// Iterates over all canonical intervals intersecting `query`.
                ///
                /// The iterator yields original intervals stored in the set,
                /// not clipped intersection segments.
                ///
                /// Complexity: `O(log n + k)`, where `k` is the number of
                /// returned intervals.
                #[inline]
                pub fn intervals_intersecting(&self, query: U8CO) -> impl Iterator<Item = U8CO> {
                    let i = self
                        .intervals
                        .partition_point(|iv| iv.end_excl() <= query.start());

                    self.intervals[i..]
                        .iter()
                        .copied()
                        .take_while(move |iv| iv.start() < query.end_excl())
                }

                /// Iterates over clipped intersection segments with `query`.
                ///
                /// Example:
                ///
                /// ```text
                /// set:   [10, 20), [30, 40)
                /// query: [15, 35)
                /// out:   [15, 20), [30, 35)
                /// ```
                ///
                /// Complexity: `O(log n + k)`, where `k` is the number of
                /// intersecting intervals.
                #[inline]
                pub fn intersections(&self, query: U8CO) -> impl Iterator<Item = U8CO> {
                    self.intervals_intersecting(query)
                        .filter_map(move |iv| iv.intersection(query))
                }
            }
        }
    }

    // ------------------------------------------------------------
    // coverage api: covered length / uncovered length / ratio
    // ------------------------------------------------------------

    mod coverage {
        use super::*;

        impl U8COSet {
            /// Returns the covered length inside `query`.
            ///
            /// Since `U8COSet` is canonical, all intersection segments are
            /// disjoint, so summing their lengths is valid.
            ///
            /// The result is always `<= query.len()`.
            #[inline]
            pub fn covered_len(&self, query: U8CO) -> u8 {
                self.intersections(query).map(|iv| iv.len()).sum()
            }

            /// Returns the uncovered length inside `query`.
            #[inline]
            pub fn uncovered_len(&self, query: U8CO) -> u8 {
                query.len() - self.covered_len(query)
            }

            /// Returns `covered_len(query) / query.len()` as `f32`.
            ///
            /// `query.len()` is non-zero because `U8CO` cannot represent an
            /// empty interval.
            #[inline]
            pub fn coverage_ratio(&self, query: U8CO) -> f32 {
                self.covered_len(query) as f32 / query.len() as f32
            }
        }
    }
}

/// Concurrent builder for `U8COSet`.
///
/// The builder accepts concurrent inserts into a skip list. During `seal`,
/// raw intervals are scanned in sorted order and merged into a canonical
/// immutable `U8COSet`.
pub mod builder {
    use std::cmp::Ordering;

    use crossbeam_skiplist::SkipSet;

    use super::set::U8COSet;
    use super::*;

    /// Private ordering key for storing `U8CO` inside `SkipSet`.
    ///
    /// `U8CO` does not need to implement `Ord` in its own crate. This wrapper
    /// defines the ordering locally:
    ///
    /// ```text
    /// (start, end_excl)
    /// ```
    ///
    /// Because this is a set key, duplicate identical intervals are naturally
    /// deduplicated during the build phase. That is correct for interval-set
    /// semantics.
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    struct Key(U8CO);

    impl Ord for Key {
        #[inline]
        fn cmp(&self, other: &Self) -> Ordering {
            self.0
                .start()
                .cmp(&other.0.start())
                .then_with(|| self.0.end_excl().cmp(&other.0.end_excl()))
        }
    }

    impl PartialOrd for Key {
        #[inline]
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    /// Concurrent write-side builder for `U8COSet`.
    ///
    /// Insertions are expected `O(log n)` through `crossbeam_skiplist`.
    /// No merging is performed during insertion; normalization happens once
    /// in `seal`.
    #[repr(transparent)]
    #[derive(Debug, Default)]
    pub struct U8COSetBuilder {
        raw: SkipSet<Key>,
    }

    impl U8COSetBuilder {
        /// Creates an empty builder.
        #[inline]
        pub fn new() -> Self {
            Self::default()
        }

        /// Inserts one interval into the builder.
        ///
        /// This method is safe to call concurrently through shared references.
        /// Identical intervals are deduplicated by the underlying `SkipSet`.
        #[inline]
        pub fn insert(&self, iv: U8CO) {
            self.raw.insert(Key(iv));
        }

        /// Consumes the builder and returns a canonical immutable set.
        ///
        /// The merge process is linear over the sorted skip-list iterator:
        ///
        /// - maintain one pending interval `cur`;
        /// - if the next interval overlaps or is adjacent, replace `cur` with
        ///   its convex hull;
        /// - otherwise, push `cur` and start a new pending interval;
        /// - finally, push the last pending interval.
        pub fn seal(self) -> U8COSet {
            let mut iter = self.raw.iter().map(|entry| entry.value().0);

            let Some(mut cur) = iter.next() else {
                // SAFETY:
                // The empty interval list is canonical.
                return unsafe { U8COSet::new_unchecked(Vec::new()) };
            };

            let mut out = Vec::new();

            for iv in iter {
                if cur.is_contiguous_with(iv) {
                    cur = cur.convex_hull(iv);
                } else {
                    out.push(cur);
                    cur = iv;
                }
            }

            out.push(cur);

            // SAFETY:
            // `self.raw` iterates keys in ascending `(start, end_excl)` order.
            // The loop maintains one pending merged interval `cur`.
            //
            // If `iv` is contiguous with or overlaps `cur`, both are replaced
            // by their convex hull, so no overlap or adjacency is emitted.
            //
            // If `iv` is disjoint from `cur`, then sorted order plus the failed
            // contiguity test imply `cur.end_excl() < iv.start()`. Pushing
            // `cur` therefore preserves the canonical invariant.
            //
            // After the loop, the final pending interval is pushed. Therefore
            // `out` is sorted, non-overlapping, and contains no adjacent
            // intervals.
            unsafe { U8COSet::new_unchecked(out) }
        }
    }
}
