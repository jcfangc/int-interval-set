use crate::int_co_set::funcs_for_canonicalization::{canonicalize_sorted, merge_sorted};

use super::*;

impl<I: IntCO> IntCOSet<I> {
    /// Returns the intersection of this set and `other`.
    ///
    /// Both sets are canonical, so their sorted interval slices can be
    /// intersected with a two-pointer scan.
    ///
    /// Example:
    ///
    /// ```text
    /// self:  [0, 10), [20, 30), [40, 50)
    /// other: [5, 25), [45, 60)
    /// out:   [5, 10), [20, 25), [45, 50)
    /// ```
    ///
    /// Complexity: `O(n + m)`, where `n` and `m` are the canonical
    /// interval counts of the two input sets.
    #[inline]
    pub fn intersection_with_set(&self, other: &Self) -> Self {
        if self.is_empty() {
            return self.clone();
        }

        if other.is_empty() {
            return other.clone();
        }

        let mut left = 0;
        let mut right = 0;
        let mut intervals = Vec::with_capacity(self.intervals.len().min(other.intervals.len()));

        while left < self.intervals.len() && right < other.intervals.len() {
            let a = self.intervals[left];
            let b = other.intervals[right];

            if let Some(intersection) = a.intersection(b) {
                intervals.push(intersection);
            }

            match a.end_excl().cmp(&b.end_excl()) {
                std::cmp::Ordering::Less => left += 1,
                std::cmp::Ordering::Greater => right += 1,
                std::cmp::Ordering::Equal => {
                    left += 1;
                    right += 1;
                }
            }
        }

        // SAFETY:
        // - Both source slices are canonical and sorted.
        // - Each emitted interval is an intersection of one source interval
        //   from each set.
        // - Advancing the interval with the smaller end preserves output order.
        // - Distinct emitted intervals cannot overlap or become adjacent,
        //   because adjacency has already been merged in both source sets.
        unsafe { Self::new_unchecked(intervals) }
    }

    #[inline]
    pub fn union_with_set(&self, other: &Self) -> Self {
        if self.is_empty() {
            return other.clone();
        }

        if other.is_empty() {
            return self.clone();
        }

        let intervals = canonicalize_sorted(merge_sorted(
            self.intervals.iter().copied(),
            other.intervals.iter().copied(),
        ));

        // SAFETY:
        // - Both source sets yield sorted canonical interval sequences.
        // - `merge_sorted` preserves ascending order.
        // - `canonicalize_sorted` merges every overlap or adjacency.
        // - Therefore the resulting interval slice is canonical.
        unsafe { Self::new_unchecked(intervals) }
    }

    /// Returns the difference of this set and `other`.
    ///
    /// The returned set contains every point covered by `self` but not by
    /// `other`.
    ///
    /// Both source sets are canonical, so intervals from `other` are scanned
    /// monotonically while residual segments from `self` are emitted.
    ///
    /// Example:
    ///
    /// ```text
    /// self:  [0, 10), [20, 30), [40, 50)
    /// other: [5, 25), [45, 60)
    /// out:   [0, 5), [25, 30), [40, 45)
    /// ```
    ///
    /// Complexity: `O(n + m)`, where `n` and `m` are the canonical
    /// interval counts of the two input sets.
    #[inline]
    pub fn difference_with_set(&self, other: &Self) -> Self {
        if self.is_empty() || other.is_empty() {
            return self.clone();
        }

        // Index of the earliest removal interval that may still affect the
        // current or a later source interval.
        let mut remove_start_idx = 0;
        let mut intervals = Vec::with_capacity(self.intervals.len());

        for source in self.intervals.iter().copied() {
            // Discard removal intervals entirely to the left of `source`.
            // They cannot intersect this source or any later source.
            while remove_start_idx < other.intervals.len()
                && other.intervals[remove_start_idx].end_excl() <= source.start()
            {
                remove_start_idx += 1;
            }

            // Left boundary of the not-yet-processed portion of `source`.
            let mut cursor = source.start();

            // Temporary removal scan for this source interval.
            let mut scan = remove_start_idx;

            while scan < other.intervals.len() {
                let removed = other.intervals[scan];

                // No later removal interval can intersect this source.
                if removed.start() >= source.end_excl() {
                    break;
                }

                // Retain the uncovered segment preceding `removed`, if non-empty.
                if let Some(residual) = I::try_new(cursor, removed.start()) {
                    intervals.push(residual);
                }

                // Advance beyond the points covered by `removed`.
                cursor = cursor.max(removed.end_excl());

                // This source has been removed through its right boundary.
                // Keep `scan` unchanged because this removal interval may continue
                // into later source intervals.
                if cursor >= source.end_excl() {
                    break;
                }

                // `removed` ended within this source; inspect the next removal.
                scan += 1;
            }

            // Retain the trailing uncovered segment, if any.
            if let Some(residual) = I::try_new(cursor, source.end_excl()) {
                intervals.push(residual);
            }

            // The next source can resume from this position. The current removal
            // interval may still overlap it, so `scan` must not be incremented
            // unconditionally.
            remove_start_idx = scan;
        }
        // SAFETY:
        // - Source intervals are visited in canonical order.
        // - Each emitted interval is a residual segment of one source interval.
        // - Residual segments from the same source are separated by removed
        //   coverage; distinct source intervals were already strictly separated.
        unsafe { Self::new_unchecked(intervals) }
    }

    /// Returns the symmetric difference of this set and `other`.
    ///
    /// The returned set contains every point covered by exactly one of the two
    /// input sets.
    ///
    /// Both source sets are canonical. This method performs a two-way sweep over
    /// their virtual boundary events while tracking whether each side currently
    /// covers the sweep position.
    ///
    /// Unlike an event-materializing implementation, the boundary event stream is
    /// represented by interval indices plus per-side coverage states:
    ///
    /// - if a side is currently outside its current interval, the next event is
    ///   that interval's start;
    /// - if a side is currently inside its current interval, the next event is
    ///   that interval's exclusive end.
    ///
    /// Example:
    ///
    /// ```text
    /// self:  [0, 10), [20, 30)
    /// other: [5, 15), [25, 35)
    /// out:   [0, 5), [10, 15), [20, 25), [30, 35)
    /// ```
    ///
    /// Complexity: `O(n + m)`, where `n` and `m` are the canonical interval
    /// counts of the two input sets.
    #[inline]
    pub fn symmetric_difference_with_set(&self, other: &Self) -> Self {
        if self.is_empty() {
            return other.clone();
        }
        if other.is_empty() {
            return self.clone();
        }

        // A set has an empty symmetric difference with itself. This also avoids
        // scanning the same backing interval slice twice for the common self/self
        // case.
        if core::ptr::eq(self, other) {
            return Self::default();
        }

        let left: &[I] = self.intervals.as_ref();
        let right: &[I] = other.intervals.as_ref();

        // `li` and `ri` point to the current interval of each side.
        //
        // A side advances its index only after processing the current interval's
        // end event. Its start event only flips the coverage state to `true`.
        let mut li = 0usize;
        let mut ri = 0usize;

        let mut left_covered = false;
        let mut right_covered = false;

        // Start coordinate of the currently open XOR-covered output interval.
        let mut opened_at = None;

        // The symmetric difference of two canonical sets contains at most `n + m`
        // canonical intervals.
        let mut intervals = Vec::with_capacity(left.len() + right.len());

        while li < left.len() || ri < right.len() {
            // Compute the next virtual boundary coordinate from each side.
            //
            // If the side is outside its current interval, the next boundary is
            // `start`; if it is inside, the next boundary is `end_excl`.
            let left_x = left.get(li).copied().map(|iv| {
                if left_covered {
                    iv.end_excl()
                } else {
                    iv.start()
                }
            });

            let right_x = right.get(ri).copied().map(|iv| {
                if right_covered {
                    iv.end_excl()
                } else {
                    iv.start()
                }
            });

            // Advance to the earliest pending boundary coordinate.
            let x = match (left_x, right_x) {
                (Some(a), Some(b)) => a.min(b),
                (Some(a), None) => a,
                (None, Some(b)) => b,
                (None, None) => unreachable!(),
            };

            let before = left_covered ^ right_covered;

            // Process all left-side boundary events at `x`.
            //
            // For a canonical non-empty interval, its start and end cannot both be
            // at the same coordinate. Therefore one loop iteration flips either
            // outside -> inside or inside -> outside. The index advances only when
            // leaving the interval.
            while let Some(iv) = left.get(li).copied() {
                let event_x = if left_covered {
                    iv.end_excl()
                } else {
                    iv.start()
                };

                if event_x != x {
                    break;
                }

                if left_covered {
                    left_covered = false;
                    li += 1;
                } else {
                    left_covered = true;
                }
            }

            // Process all right-side boundary events at the same coordinate before
            // observing the new XOR state. Grouping equal-coordinate events keeps
            // the output canonical when one side ends exactly where the other side
            // starts.
            while let Some(iv) = right.get(ri).copied() {
                let event_x = if right_covered {
                    iv.end_excl()
                } else {
                    iv.start()
                };

                if event_x != x {
                    break;
                }

                if right_covered {
                    right_covered = false;
                    ri += 1;
                } else {
                    right_covered = true;
                }
            }

            let after = left_covered ^ right_covered;

            match (before, after) {
                // Enter a region covered by exactly one input set.
                (false, true) => opened_at = Some(x),

                // Leave such a region and emit its canonical interval.
                (true, false) => {
                    let start = opened_at
                        .take()
                        .expect("symmetric difference region must have a start");

                    intervals.push(
                        I::try_new(start, x)
                            .expect("symmetric difference region must be non-empty"),
                    );
                }

                // XOR state did not change:
                // - false -> false: remain outside the result;
                // - true -> true: remain inside one continuous result interval.
                _ => {}
            }
        }

        debug_assert!(opened_at.is_none());

        // SAFETY:
        // - Both input slices are canonical, so their virtual boundary events are
        //   ordered within each side.
        // - The sweep always consumes the smallest pending boundary coordinate.
        // - Equal-coordinate events from both sides are processed together before
        //   the new XOR state is evaluated.
        // - An output interval is emitted exactly when XOR coverage changes from
        //   true to false.
        // - Therefore emitted intervals are sorted, non-empty, non-overlapping, and
        //   non-adjacent.
        unsafe { Self::new_unchecked(intervals) }
    }
}

#[cfg(test)]
mod tests_for_difference;
#[cfg(test)]
mod tests_for_intersection;
#[cfg(test)]
mod tests_for_symmetric_difference;
#[cfg(test)]
mod tests_for_union;
