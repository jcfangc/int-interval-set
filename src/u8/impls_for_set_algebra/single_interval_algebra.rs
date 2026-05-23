use super::*;

impl U8COSet {
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
