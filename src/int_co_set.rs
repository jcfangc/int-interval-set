#[cfg(test)]
mod test_support;

use std::sync::Arc;

use int_interval::traits::IntCO;

/// Immutable canonical closed-open integer interval set.
///
/// Internally this is an `Arc<[I]>`, so cloning an `IntCOSet<I>` is cheap.
///
/// Canonical invariant:
///
/// ```text
/// for every adjacent pair a, b:
///     a.end_excl() < b.start()
/// ```
///
/// The strict `<` means both overlap and adjacency have already been merged.
///
/// `I::Ord` is expected to follow interval boundary ordering, consistent with
/// the primitive interval implementations provided by `int_interval`.
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IntCOSet<I: IntCO> {
    intervals: Arc<[I]>,
}

mod impls_for_accessors;
mod impls_for_algebra;
mod impls_for_construction;
mod impls_for_find_out_coverage;
mod impls_for_predicates;
mod impls_for_searching;

mod funcs_for_canonicalization;
