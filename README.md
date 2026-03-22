
# int-interval-set

Integer half-open interval set structures built on top of [`int-interval`](https://crates.io/crates/int-interval).

This crate provides efficient representations of integer interval sets (`[start, end_excl)`) with a focus on predictable semantics and low-overhead abstractions.

## Features

- Half-open interval semantics (`[start, end)`)
- Automatic normalization (merge overlapping / adjacent intervals)
- Efficient point and interval queries
- Codegen-based implementations for zero-cost abstractions

## Implementation Status

Currently implemented:

- **Batch (unsigned)**: optimized for bulk construction and read-heavy workloads

Planned (not yet implemented):

- **Online**: incremental updates with dynamic structure
- **Signed types** support

## Example

```rust
use int_interval::U8CO;
use int_interval_set::U8COBatchSet;

let a = U8CO::try_new(10, 20).unwrap();
let b = U8CO::try_new(15, 25).unwrap();

let set = U8COBatchSet::from([a, b]);

assert!(set.contains_point(18));
assert!(set.contains_interval(U8CO::try_new(12, 18).unwrap()));

assert_eq!(set.interval_count(), 1);
```

## Design

* Built on top of `int-interval`
* Separation of concerns:

  * interval representation (`int-interval`)
  * interval set structure (this crate)

## Status

Early-stage. APIs may evolve.

## License

MIT OR Apache-2.0