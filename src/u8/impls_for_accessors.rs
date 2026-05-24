use super::*;

impl U8COSet {
    /// Returns the number of canonical intervals.
    #[inline]
    pub fn interval_count(&self) -> usize {
        self.intervals.len()
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
