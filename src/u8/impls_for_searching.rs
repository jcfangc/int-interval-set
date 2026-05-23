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
}

#[cfg(test)]
mod tests_for_intervals_intersecting;

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

#[cfg(test)]
mod tests_for_interval_containing_point;
