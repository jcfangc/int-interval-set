#[cfg(test)]
mod test_support;

use std::sync::Arc;

use int_interval::U8CO;

/// Immutable canonical interval set.
///
/// Internally this is an `Arc<[U8CO]>`, so cloning a `U8COSet` is cheap.
///
/// Canonical invariant:
///
/// ```text
/// for every adjacent pair a, b:
///     a.end_excl() < b.start()
/// ```
///
/// The strict `<` means both overlap and adjacency have already been merged.
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct U8COSet {
    intervals: Arc<[U8CO]>,
}

mod impls_for_accessors;
mod impls_for_algebra;
mod impls_for_construction;
mod impls_for_find_out_coverage;
mod impls_for_predicates;
mod impls_for_searching;

mod funcs_for_canonicalization;
