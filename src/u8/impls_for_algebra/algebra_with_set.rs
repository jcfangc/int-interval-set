use crate::u8::funcs_for_canonicalization::{canonicalize_sorted, merge_sorted};

use super::*;

impl U8COSet {
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
}

#[cfg(test)]
mod tests_for_intersection;
#[cfg(test)]
mod tests_for_union;
