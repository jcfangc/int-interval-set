use super::*;

impl U8COSet {
    /// Returns the covered length inside `query`.
    ///
    /// Since `U8COSet` is canonical, all intersection segments are
    /// disjoint, so summing their lengths is valid.
    ///
    /// The result is always `<= query.len()`.
    #[inline]
    pub fn covered_len_of(&self, query: U8CO) -> u8 {
        self.intersections(query).map(|iv| iv.len()).sum()
    }

    /// Returns the uncovered length inside `query`.
    #[inline]
    pub fn uncovered_len_of(&self, query: U8CO) -> u8 {
        query.len() - self.covered_len_of(query)
    }

    /// Returns `covered_len(query) / query.len()` as `f32`.
    ///
    /// `query.len()` is non-zero because `U8CO` cannot represent an
    /// empty interval.
    #[inline]
    pub fn coverage_ratio_of(&self, query: U8CO) -> f32 {
        self.covered_len_of(query) as f32 / query.len() as f32
    }
}

#[cfg(test)]
mod tests;
