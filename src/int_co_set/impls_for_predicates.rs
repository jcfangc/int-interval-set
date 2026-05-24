use super::*;

impl<I: IntCO> IntCOSet<I> {
    /// Returns whether `x` is covered by any interval in the set.
    ///
    /// Complexity: `O(log n)`.
    #[inline]
    pub fn contains_point(&self, x: I::CoordType) -> bool {
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
    pub fn contains_interval(&self, query: I) -> bool {
        let i = self
            .intervals
            .partition_point(|iv| iv.start() <= query.start());

        i != 0 && self.intervals[i - 1].contains_interval(query)
    }

    /// Returns whether `query` intersects any interval in the set.
    ///
    /// Complexity: `O(log n)`.
    #[inline]
    pub fn intersects_interval(&self, query: I) -> bool {
        let i = self
            .intervals
            .partition_point(|iv| iv.end_excl() <= query.start());

        self.intervals.get(i).is_some_and(|iv| iv.intersects(query))
    }
}

#[cfg(test)]
mod tests;
