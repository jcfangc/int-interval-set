use super::*;

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

#[cfg(test)]
mod tests;
