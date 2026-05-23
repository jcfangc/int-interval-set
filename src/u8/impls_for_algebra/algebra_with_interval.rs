use super::*;

impl U8COSet {
    /// Returns the intersection of this set with `query`.
    ///
    /// The returned set contains the clipped segments of all canonical
    /// intervals intersecting `query`.
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
    pub fn intersection_with_interval(&self, query: U8CO) -> Self {
        let intervals = self
            .intervals_intersecting(query)
            .filter_map(|iv| iv.intersection(query))
            .collect::<Vec<_>>();

        // SAFETY:
        // `self.intervals_intersecting(query)` yields intervals from the
        // canonical source set in ascending order. Intersecting each one
        // with the same `query` preserves ordering, cannot create overlap,
        // and cannot create adjacency between originally separated intervals.
        unsafe { Self::new_unchecked(intervals) }
    }

    /// Returns the union of this set with `query`.
    ///
    /// Intervals intersecting or adjacent to `query` are merged with it.
    /// If `query` is disjoint from all existing intervals, it is inserted
    /// at its canonical position.
    ///
    /// Example:
    ///
    /// ```text
    /// set:   [10, 20), [30, 40)
    /// query: [20, 30)
    /// out:   [10, 40)
    /// ```
    ///
    /// Complexity: `O(log n + n)` because the returned immutable interval
    /// slice must be allocated and populated.
    #[inline]
    pub fn union_with_interval(&self, query: U8CO) -> Self {
        let merge_start = self
            .intervals
            .partition_point(|iv| iv.end_excl() < query.start());

        let merge_end = merge_start
            + self.intervals[merge_start..].partition_point(|iv| iv.start() <= query.end_excl());

        let merged = if merge_start == merge_end {
            query
        } else {
            self.intervals[merge_start]
                .convex_hull(query)
                .convex_hull(self.intervals[merge_end - 1])
        };

        let mut intervals =
            Vec::with_capacity(self.intervals.len() - (merge_end - merge_start) + 1);

        intervals.extend_from_slice(&self.intervals[..merge_start]);
        intervals.push(merged);
        intervals.extend_from_slice(&self.intervals[merge_end..]);

        // SAFETY:
        // - Prefix and suffix are unchanged canonical slices.
        // - `merge_start..merge_end` contains exactly the intervals contiguous
        //   with `query`, including adjacency.
        // - They are replaced by their single convex hull with `query`.
        // - The remaining prefix and suffix are strictly separated from
        //   `merged`, so the resulting slice is canonical.
        unsafe { Self::new_unchecked(intervals) }
    }

    /// Returns the difference of this set and `query`.
    ///
    /// The operation removes every point covered by `query` from this set.
    /// Intervals outside `query` are retained unchanged; intersecting boundary
    /// intervals may be clipped into left and right residual segments.
    ///
    /// Example:
    ///
    /// ```text
    /// set:   [10, 20), [30, 40), [50, 60)
    /// query: [15, 55)
    /// out:   [10, 15), [55, 60)
    /// ```
    ///
    /// Complexity: `O(log n)` if `query` is disjoint from the set; otherwise
    /// `O(n)` because the returned immutable interval slice must be copied.
    #[inline]
    pub fn difference_with_interval(&self, query: U8CO) -> Self {
        let hit_start = self
            .intervals
            .partition_point(|iv| iv.end_excl() <= query.start());

        let hit_end = hit_start
            + self.intervals[hit_start..].partition_point(|iv| iv.start() < query.end_excl());

        if hit_start == hit_end {
            return self.clone();
        }

        let first = self.intervals[hit_start];
        let last = self.intervals[hit_end - 1];

        let left = U8CO::try_new(first.start(), query.start());
        let right = U8CO::try_new(query.end_excl(), last.end_excl());

        let mut intervals = Vec::with_capacity(
            hit_start
                + usize::from(left.is_some())
                + usize::from(right.is_some())
                + (self.intervals.len() - hit_end),
        );

        intervals.extend_from_slice(&self.intervals[..hit_start]);
        intervals.extend(left);
        intervals.extend(right);
        intervals.extend_from_slice(&self.intervals[hit_end..]);

        // SAFETY:
        // - Prefix and suffix are unchanged canonical subsequences from `self`.
        // - Every interval in `hit_start..hit_end` intersects `query`.
        // - Any fully covered interior intervals are removed.
        // - `left`, when present, is a strict prefix of the first hit interval.
        // - `right`, when present, is a strict suffix of the last hit interval.
        // - Removing or shrinking canonical intervals cannot introduce overlap
        //   or adjacency with retained neighbors.
        unsafe { Self::new_unchecked(intervals) }
    }

    /// Returns the symmetric difference `self △ query`.
    ///
    /// The returned set contains points covered by exactly one of `self` and
    /// `query`.
    ///
    /// Equivalently:
    ///
    /// ```text
    /// self △ query = (self ∪ query) \ (self ∩ query)
    /// ```
    ///
    /// Example:
    ///
    /// ```text
    /// self:  [10, 20), [30, 40)
    /// query: [15, 35)
    /// out:   [10, 15), [20, 30), [35, 40)
    /// ```
    ///
    /// Complexity: `O(log n + k + n)`, where `k` is the number of canonical
    /// intervals in the union component affected by `query`.
    #[inline]
    pub fn symmetric_difference_with_interval(&self, query: U8CO) -> Self {
        let component_start = self
            .intervals
            .partition_point(|interval| interval.end_excl() < query.start());

        let component_end = component_start
            + self.intervals[component_start..]
                .partition_point(|interval| interval.start() <= query.end_excl());

        if component_start == component_end {
            return self.union_with_interval(query);
        }

        let component = self.intervals[component_start]
            .convex_hull(query)
            .convex_hull(self.intervals[component_end - 1]);

        let mut intervals = Vec::with_capacity(self.intervals.len() + 1);

        intervals.extend_from_slice(&self.intervals[..component_start]);

        let mut cursor = component.start();

        for source in self.intervals[component_start..component_end]
            .iter()
            .copied()
        {
            let Some(overlap) = source.intersection(query) else {
                continue;
            };

            if let Some(residual) = U8CO::try_new(cursor, overlap.start()) {
                intervals.push(residual);
            }

            cursor = overlap.end_excl();
        }

        if let Some(residual) = U8CO::try_new(cursor, component.end_excl()) {
            intervals.push(residual);
        }

        intervals.extend_from_slice(&self.intervals[component_end..]);

        // SAFETY:
        // - Prefix and suffix are unchanged canonical subsequences.
        // - `component` is the canonical union component containing `query`
        //   and every source interval intersecting or adjacent to it.
        // - The emitted residuals are exactly `component` with all positive
        //   intersections with `query` removed.
        // - Because adjacency was included in `component`, emitted residuals
        //   cannot become adjacent to retained prefix or suffix intervals.
        // - All emitted intervals are ordered, non-overlapping, and
        //   non-adjacent.
        unsafe { Self::new_unchecked(intervals) }
    }
}

#[cfg(test)]
mod tests_for_difference;
#[cfg(test)]
mod tests_for_intersections;
#[cfg(test)]
mod tests_for_symmetric_difference;
#[cfg(test)]
mod tests_for_union;
