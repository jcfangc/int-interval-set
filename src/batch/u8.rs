#[cfg(test)]
mod construction_tests;
#[cfg(test)]
mod membership_queries_tests;
#[cfg(test)]
mod statistics_tests;

use int_interval::U8CO;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct U8COBatchSet(Vec<U8CO>);

// ============================================================
// conversions / views
// ============================================================

impl U8COBatchSet {
    #[inline]
    pub fn into_inner(self) -> Vec<U8CO> {
        self.0
    }

    #[inline]
    pub fn as_slice(&self) -> &[U8CO] {
        &self.0
    }
}

// ============================================================
// membership queries
// ============================================================

impl U8COBatchSet {
    #[inline]
    pub fn contains_interval(&self, iv: U8CO) -> bool {
        let slice = self.as_slice();
        let i = slice.partition_point(|seg| seg.start() <= iv.start());
        i > 0 && slice[i - 1].contains_interval(iv)
    }

    #[inline]
    pub fn contains_point(&self, x: u8) -> bool {
        let slice = self.as_slice();
        let i = slice.partition_point(|seg| seg.start() <= x);
        i > 0 && slice[i - 1].contains(x)
    }

    #[inline]
    pub fn interval_containing_point(&self, x: u8) -> Option<U8CO> {
        let slice = self.as_slice();
        let i = slice.partition_point(|seg| seg.start() <= x);

        if i == 0 {
            return None;
        }

        let iv = slice[i - 1];
        iv.contains(x).then_some(iv)
    }

    #[inline]
    pub fn intersects(&self, q: U8CO) -> bool {
        let slice = self.as_slice();
        let i = slice.partition_point(|seg| seg.start() < q.end_excl());

        if i == 0 {
            return false;
        }

        let mut j = i - 1;
        while j < slice.len() {
            let iv = slice[j];
            if iv.start() >= q.end_excl() {
                break;
            }
            if iv.intersects(q) {
                return true;
            }
            j += 1;
        }

        false
    }
}

// ============================================================
// iteration
// ============================================================

impl U8COBatchSet {
    #[inline]
    pub fn iter_intervals(&self) -> impl Iterator<Item = U8CO> {
        self.as_slice().iter().copied()
    }

    #[inline]
    pub fn iter_points(&self) -> impl Iterator<Item = u8> {
        self.iter_intervals().flat_map(U8CO::iter)
    }
}

// ============================================================
// statistics / coverage
// ============================================================

impl U8COBatchSet {
    #[inline]
    pub fn interval_count(&self) -> u8 {
        self.0.len() as u8
    }

    #[inline]
    pub fn point_count(&self) -> u8 {
        self.0.iter().map(|iv| iv.len()).sum()
    }

    #[inline]
    pub fn coverage_len_of(&self, q: U8CO) -> u8 {
        let slice = self.as_slice();
        let mut i = slice.partition_point(|seg| seg.start() < q.start());
        i = i.saturating_sub(1);

        let mut acc = 0u8;
        while i < slice.len() {
            let iv = slice[i];
            if iv.start() >= q.end_excl() {
                break;
            }

            if let Some(overlap) = iv.intersection(q) {
                acc += overlap.len();
            }

            i += 1;
        }

        acc
    }

    #[inline]
    pub fn coverage_ratio_of(&self, q: U8CO) -> f32 {
        self.coverage_len_of(q) as f32 / q.len() as f32
    }
}

// ============================================================
// normalization / construction
// ============================================================

mod construction {
    use super::*;

    #[inline]
    fn normalize(mut xs: Vec<U8CO>) -> Vec<U8CO> {
        if xs.len() <= 1 {
            return xs;
        }

        xs.sort_unstable_by(|a, b| {
            a.start()
                .cmp(&b.start())
                .then(a.end_excl().cmp(&b.end_excl()))
        });

        let mut out = Vec::with_capacity(xs.len());

        for iv in xs {
            match out.last_mut() {
                None => out.push(iv),
                Some(last) => {
                    if last.is_contiguous_with(iv) {
                        *last = last.convex_hull(iv);
                    } else {
                        out.push(iv);
                    }
                }
            }
        }

        out
    }

    impl From<Vec<U8CO>> for U8COBatchSet {
        #[inline]
        fn from(value: Vec<U8CO>) -> Self {
            Self(normalize(value))
        }
    }

    impl<const N: usize> From<[U8CO; N]> for U8COBatchSet {
        #[inline]
        fn from(value: [U8CO; N]) -> Self {
            Self::from(Vec::from(value))
        }
    }

    impl FromIterator<U8CO> for U8COBatchSet {
        #[inline]
        fn from_iter<T: IntoIterator<Item = U8CO>>(iter: T) -> Self {
            Self::from(iter.into_iter().collect::<Vec<_>>())
        }
    }
}
