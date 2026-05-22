#[cfg(test)]
mod tests;

use std::sync::Arc;

use int_interval::I8CO;

/// Read-only canonical interval set for `I8CO`.
///
/// A `I8COSet` stores a normalized collection of half-open `I8CO` intervals:
///
/// - intervals are sorted by `start`;
/// - intervals are non-overlapping;
/// - adjacent intervals are merged;
/// - all queries are performed against the sealed immutable array.
///
/// Construction is intentionally restricted. Use `I8COSetBuilder` for normal
/// construction.
pub mod set {
    use super::*;

    /// Immutable canonical interval set.
    ///
    /// Internally this is an `Arc<[I8CO]>`, so cloning a `I8COSet` is cheap.
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
    pub struct I8COSet {
        intervals: Arc<[I8CO]>,
    }

    // ------------------------------------------------------------
    // basic api: construction / accessors
    // ------------------------------------------------------------

    mod basic {
        use super::*;

        impl I8COSet {
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
            pub(in crate::i8) unsafe fn new_unchecked(intervals: Vec<I8CO>) -> Self {
                debug_assert!(Self::is_canonical(&intervals));

                Self {
                    intervals: Arc::from(intervals.into_boxed_slice()),
                }
            }

            /// Checks the canonical invariant used by binary-search queries.
            ///
            /// `I8CO` itself already guarantees single-interval validity. This
            /// function only checks the relationship between adjacent intervals.
            #[inline]
            fn is_canonical(intervals: &[I8CO]) -> bool {
                intervals.windows(2).all(|w| w[0].end_excl() < w[1].start())
            }
        }

        impl I8COSet {
            /// Returns the number of canonical intervals.
            ///
            /// For the `I8CO` domain, the maximum canonical interval count is 128,
            /// e.g. alternating single-point intervals across `[i8::MIN, i8::MAX)`.
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
            pub fn as_slice(&self) -> &[I8CO] {
                &self.intervals
            }

            /// Iterates over canonical intervals by value.
            #[inline]
            pub fn iter_intervals(&self) -> impl Iterator<Item = I8CO> {
                self.intervals.iter().copied()
            }
        }
    }

    // ------------------------------------------------------------
    // predicate api: yes/no queries
    // ------------------------------------------------------------

    mod predicates {
        use super::*;

        impl I8COSet {
            /// Returns whether `x` is covered by any interval in the set.
            ///
            /// Complexity: `O(log n)`.
            #[inline]
            pub fn contains_point(&self, x: i8) -> bool {
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
            pub fn contains_interval(&self, query: I8CO) -> bool {
                let i = self
                    .intervals
                    .partition_point(|iv| iv.start() <= query.start());

                i != 0 && self.intervals[i - 1].contains_interval(query)
            }

            /// Returns whether `query` intersects any interval in the set.
            ///
            /// Complexity: `O(log n)`.
            #[inline]
            pub fn intersects_interval(&self, query: I8CO) -> bool {
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

            impl I8COSet {
                /// Returns the unique interval containing `x`, if any.
                ///
                /// Because the set is canonical, at most one interval can
                /// contain a single point.
                ///
                /// Complexity: `O(log n)`.
                #[inline]
                pub fn interval_containing_point(&self, x: i8) -> Option<I8CO> {
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

            impl I8COSet {
                /// Iterates over all canonical intervals intersecting `query`.
                ///
                /// The iterator yields original intervals stored in the set,
                /// not clipped intersection segments.
                ///
                /// Complexity: `O(log n + k)`, where `k` is the number of
                /// returned intervals.
                #[inline]
                pub fn intervals_intersecting(&self, query: I8CO) -> impl Iterator<Item = I8CO> {
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
                pub fn intersections(&self, query: I8CO) -> impl Iterator<Item = I8CO> {
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

        impl I8COSet {
            /// Returns the covered length inside `query`.
            ///
            /// Since `I8COSet` is canonical, all intersection segments are
            /// disjoint, so summing their lengths is valid.
            ///
            /// The result is always `<= query.len()`.
            #[inline]
            pub fn covered_len(&self, query: I8CO) -> u8 {
                self.intersections(query).map(|iv| iv.len()).sum()
            }

            /// Returns the uncovered length inside `query`.
            #[inline]
            pub fn uncovered_len(&self, query: I8CO) -> u8 {
                query.len() - self.covered_len(query)
            }

            /// Returns `covered_len(query) / query.len()` as `f32`.
            ///
            /// `query.len()` is non-zero because `I8CO` cannot represent an
            /// empty interval.
            #[inline]
            pub fn coverage_ratio(&self, query: I8CO) -> f32 {
                self.covered_len(query) as f32 / query.len() as f32
            }
        }
    }
}

/// Concurrent builder for `I8COSet`.
///
/// The builder accepts concurrent inserts into a skip list. During `seal`,
/// raw intervals are scanned in sorted order and merged into a canonical
/// immutable `I8COSet`.
pub mod builder {
    use crossbeam_skiplist::SkipSet;

    use super::set::I8COSet;
    use super::*;

    /// Concurrent write-side builder for `I8COSet`.
    ///
    /// Insertions are expected `O(log n)` through `crossbeam_skiplist`.
    /// No merging is performed during insertion; normalization happens once
    /// in `seal`.
    #[repr(transparent)]
    #[derive(Debug, Default)]
    pub struct I8COSetBuilder {
        raw: SkipSet<I8CO>,
    }

    impl I8COSetBuilder {
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
        pub fn insert(&self, iv: I8CO) {
            self.raw.insert(iv);
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
        pub fn seal(self) -> I8COSet {
            let mut iter = self.raw.into_iter();

            let Some(mut cur) = iter.next() else {
                // SAFETY:
                // The empty interval list is canonical.
                return unsafe { I8COSet::new_unchecked(Vec::new()) };
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
            unsafe { I8COSet::new_unchecked(out) }
        }
    }
}
