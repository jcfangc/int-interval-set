# int-interval-set

[![Crates.io](https://img.shields.io/crates/v/int-interval-set.svg)](https://crates.io/crates/int-interval-set)
[![Documentation](https://docs.rs/int-interval-set/badge.svg)](https://docs.rs/int-interval-set)
[![License](https://img.shields.io/crates/l/int-interval-set.svg)](https://crates.io/crates/int-interval-set)
[![CodSpeed](https://github.com/jcfangc/int-interval-set/actions/workflows/codspeed.yml/badge.svg?branch=main)](https://github.com/jcfangc/int-interval-set/actions/workflows/codspeed.yml)


`int-interval-set` provides immutable canonical interval-set containers for
integer half-open intervals.

The crate is built around a single generic set type:

```rust
IntCOSet<I>
```

where:

```rust
I: IntCO
```

Concrete interval-set types such as `U8COSet` and `I32COSet` are now simple
type aliases.

The library no longer uses builders, concurrent mutable phases, or generated
set implementations.

Instead, interval sets are constructed directly from iterators and immediately
canonicalized into compact immutable arrays.

---

# Interval semantics

All intervals are half-open:

```text
[start, end_excl)
```

For example:

```text
[10, 20)
```

contains:

```text
10, 11, ..., 19
```

but does not contain:

```text
20
```

Intervals are canonicalized automatically during construction.

Adjacent intervals are merged:

```text
[0, 5) + [5, 10) -> [0, 10)
```

Overlapping intervals are also merged:

```text
[10, 20) + [15, 30) -> [10, 30)
```

The canonical invariant is:

```text
for every adjacent pair a, b:
    a.end_excl() < b.start()
```

This means canonical interval sets are:

- sorted;
- non-overlapping;
- non-adjacent;
- already merged.

---

# Core type

```rust
pub struct IntCOSet<I: IntCO>
```

Internally:

```rust
Arc<[I]>
```

is used as the immutable storage representation.

Cloning a set is therefore cheap.

---

# Provided aliases

Unsigned:

```rust
U8COSet
U16COSet
U32COSet
U64COSet
U128COSet
UsizeCOSet
```

Signed:

```rust
I8COSet
I16COSet
I32COSet
I64COSet
I128COSet
IsizeCOSet
```

These are aliases only:

```rust
pub type U8COSet = IntCOSet<U8CO>;
```

---

# Construction

Interval sets are usually constructed through `FromIterator`.

Example:

```rust
use int_interval::U8CO;
use int_interval_set::U8COSet;

let set: U8COSet = [
    U8CO::try_new(10, 20).unwrap(),
    U8CO::try_new(15, 30).unwrap(),
    U8CO::try_new(40, 50).unwrap(),
]
.into_iter()
.collect();

assert_eq!(
    set.as_slice(),
    &[
        U8CO::try_new(10, 30).unwrap(),
        U8CO::try_new(40, 50).unwrap(),
    ]
);
```

Parallel construction is also supported through Rayon:

```rust
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use int_interval::U8CO;
use int_interval_set::U8COSet;

let set: U8COSet = vec![
    U8CO::try_new(0, 10).unwrap(),
    U8CO::try_new(10, 20).unwrap(),
]
.into_par_iter()
.collect();

assert_eq!(
    set.as_slice(),
    &[U8CO::try_new(0, 20).unwrap()]
);
```

---

# Basic accessors

```rust
set.interval_count();
set.is_empty();
set.as_slice();
set.iter_intervals();
```

Example:

```rust
assert_eq!(set.interval_count(), 2);

for interval in set.iter_intervals() {
    println!("{interval:?}");
}
```

---

# Predicate APIs

Predicate APIs answer yes-or-no questions.

```rust
set.contains_point(x);
set.contains_interval(query);
set.intersects_interval(query);
```

Example:

```rust
use int_interval::U8CO;

let query = U8CO::try_new(15, 25).unwrap();

assert!(set.intersects_interval(query));
assert!(!set.contains_interval(query));
```

---

# Search APIs

## Find interval containing a point

```rust
let hit = set.interval_containing_point(18);
```

Because sets are canonical, at most one interval can contain a point.

---

## Iterate intersecting intervals

```rust
let hits: Vec<_> = set
    .intervals_intersecting(
        U8CO::try_new(15, 45).unwrap()
    )
    .collect();
```

Example:

```text
set:   [10, 20), [30, 40)
query: [15, 35)

out:
  [10, 20), [30, 40)
```

---

# Interval-vs-set algebra

## Intersection

```rust
set.intersection_with_interval(query);
```

Example:

```text
set:   [10, 20), [30, 40)
query: [15, 35)

out:
  [15, 20), [30, 35)
```

---

## Union

```rust
set.union_with_interval(query);
```

Example:

```text
set:   [10, 20), [30, 40)
query: [20, 30)

out:
  [10, 40)
```

---

## Difference

```rust
set.difference_with_interval(query);
```

Example:

```text
set:   [10, 20), [30, 40), [50, 60)
query: [15, 55)

out:
  [10, 15), [55, 60)
```

---

## Symmetric difference

```rust
set.symmetric_difference_with_interval(query);
```

Example:

```text
set:   [10, 20), [30, 40)
query: [15, 35)

out:
  [10, 15), [20, 30), [35, 40)
```

---

# Set-vs-set algebra

## Intersection

```rust
left.intersection_with_set(&right);
```

---

## Union

```rust
left.union_with_set(&right);
```

---

## Difference

```rust
left.difference_with_set(&right);
```

---

## Symmetric difference

```rust
left.symmetric_difference_with_set(&right);
```

Example:

```text
left:  [0, 10), [20, 30)
right: [5, 15), [25, 35)

xor:
  [0, 5), [10, 15), [20, 25), [30, 35)
```

---

# Coverage APIs

Coverage APIs measure how much of a query interval is covered by the set.

```rust
set.covered_len_of(query);
set.uncovered_len_of(query);

set.coverage_ratio_f32_of(query);
set.coverage_ratio_f64_of(query);
```

Example:

```text
set:   [10, 20), [30, 40)
query: [15, 35)

covered:
  [15, 20), [30, 35)

covered length:
  10

query length:
  20

coverage ratio:
  0.5
```

---

# Complexity

Most query operations are logarithmic or linear in the number of matching
intervals.

Representative costs:

| Operation | Complexity |
|---|---|
| `contains_point` | `O(log n)` |
| `contains_interval` | `O(log n)` |
| `intersects_interval` | `O(log n)` |
| `intervals_intersecting` | `O(log n + k)` |
| `intersection_with_interval` | `O(log n + k)` |
| `union_with_interval` | `O(log n + n)` |
| `difference_with_interval` | `O(log n)` to `O(n)` |
| `intersection_with_set` | `O(n + m)` |
| `union_with_set` | `O(n + m)` |
| `difference_with_set` | `O(n + m)` |
| `symmetric_difference_with_set` | `O(n + m)` |

where:

```text
n = self interval count
m = other interval count
k = number of matching intervals
```

---

# Design goals

The crate prioritizes:

- immutable canonical representations;
- predictable interval algebra;
- low-overhead binary-search queries;
- generic implementations over all integer interval types;
- zero code generation;
- simple semantics and explicit APIs.