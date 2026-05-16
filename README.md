
# int-interval-set

`int-interval-set` provides canonical interval-set containers for integer half-open intervals.

It is designed around a two-phase model:

```text
build phase:
  concurrent inserts into a skip list

seal phase:
  sorted scan + merge

query phase:
  immutable canonical array
```

The core idea is:

```text
SkipList for concurrent writes.
Arc<[Interval]> for fast read-only queries.
```

## Features

* Supports integer closed-open intervals such as `[start, end_excl)`.
* Builds interval sets from `int-interval` interval types.
* Uses `crossbeam-skiplist` during the write phase.
* Produces immutable canonical interval sets after `seal()`.
* Query APIs are backed by binary search over compact arrays.
* Supports unsigned and signed integer interval-set types through code generation.
* Generated files are checked in and verified by codegen check mode.

## Interval semantics

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

Adjacent intervals are merged during sealing:

```text
[0, 5) + [5, 10) -> [0, 10)
```

Overlapping intervals are also merged:

```text
[10, 20) + [15, 30) -> [10, 30)
```

The sealed set is canonical:

```text
for every adjacent pair a, b:
    a.end_excl() < b.start()
```

That means sealed intervals are:

* sorted;
* non-overlapping;
* non-adjacent;
* already merged.

## Supported types

The crate is generated from hand-maintained templates.

Unsigned types:

```rust
U8COSet
U16COSet
U32COSet
U64COSet
U128COSet
UsizeCOSet
```

Signed types:

```rust
I8COSet
I16COSet
I32COSet
I64COSet
I128COSet
IsizeCOSet
```

Each type has a corresponding builder:

```rust
U8COSetBuilder
U16COSetBuilder
I8COSetBuilder
I16COSetBuilder
// ...
```

## Basic usage

```rust
use int_interval::U8CO;
use int_interval_set::U8COSetBuilder;

let builder = U8COSetBuilder::new();

builder.insert(U8CO::try_new(10, 20).unwrap());
builder.insert(U8CO::try_new(15, 30).unwrap());
builder.insert(U8CO::try_new(40, 50).unwrap());

let set = builder.seal();

assert_eq!(
    set.as_slice(),
    &[
        U8CO::try_new(10, 30).unwrap(),
        U8CO::try_new(40, 50).unwrap(),
    ]
);

assert!(set.contains_point(18));
assert!(!set.contains_point(30));
```

## Concurrent build phase

Builders accept shared-reference inserts:

```rust
use int_interval::U8CO;
use int_interval_set::U8COSetBuilder;

let builder = U8COSetBuilder::new();

std::thread::scope(|s| {
    let b = &builder;
    s.spawn(move || {
        b.insert(U8CO::try_new(0, 10).unwrap());
    });

    let b = &builder;
    s.spawn(move || {
        b.insert(U8CO::try_new(10, 20).unwrap());
    });
});

let set = builder.seal();

assert_eq!(set.as_slice(), &[U8CO::try_new(0, 20).unwrap()]);
```

`seal()` consumes the builder. After sealing, the set is immutable and cheap to clone.

## Query APIs

### Basic accessors

```rust
set.interval_count();
set.is_empty();
set.as_slice();
set.iter_intervals();
```

### Predicate APIs

Predicate APIs answer yes-or-no questions:

```rust
set.contains_point(x);
set.contains_interval(query);
set.intersects_interval(query);
```

Example:

```rust
let query = U8CO::try_new(15, 25).unwrap();

assert!(set.intersects_interval(query));
```

### Search APIs

Search APIs return matching intervals.

Find the unique interval containing a point:

```rust
let hit = set.interval_containing_point(18);
```

Return all stored intervals intersecting a query:

```rust
let hits: Vec<_> = set
    .intervals_intersecting(U8CO::try_new(15, 45).unwrap())
    .collect();
```

Return clipped intersection segments:

```rust
let intersections: Vec<_> = set
    .intersections(U8CO::try_new(15, 45).unwrap())
    .collect();
```

For example:

```text
set:   [10, 20), [30, 40)
query: [15, 35)

intervals_intersecting:
  [10, 20), [30, 40)

intersections:
  [15, 20), [30, 35)
```

### Coverage APIs

Coverage APIs compute how much of a query interval is covered by the set:

```rust
set.covered_len(query);
set.uncovered_len(query);
set.coverage_ratio(query);
```

Example:

```rust
use int_interval::U8CO;
use int_interval_set::U8COSetBuilder;

let builder = U8COSetBuilder::new();

builder.insert(U8CO::try_new(10, 20).unwrap());
builder.insert(U8CO::try_new(30, 40).unwrap());

let set = builder.seal();
let query = U8CO::try_new(15, 35).unwrap();

assert_eq!(set.covered_len(query), 10);
assert_eq!(set.uncovered_len(query), 10);
assert_eq!(set.coverage_ratio(query), 0.5);
```

## Complexity

Let `n` be the number of raw inserted intervals, and `m` be the number of canonical intervals after sealing.

| Operation                       |                             Complexity |
| ------------------------------- | -------------------------------------: |
| `builder.insert(interval)`      |                    expected `O(log n)` |
| `builder.seal()`                | `O(n)` over sorted skip-list iteration |
| `contains_point(x)`             |                             `O(log m)` |
| `contains_interval(query)`      |                             `O(log m)` |
| `intersects_interval(query)`    |                             `O(log m)` |
| `intervals_intersecting(query)` |                         `O(log m + k)` |
| `intersections(query)`          |                         `O(log m + k)` |
| `covered_len(query)`            |                         `O(log m + k)` |

`k` is the number of matched intervals.

## Why not query the skip list directly?

The builder stores raw intervals. Raw intervals may overlap:

```text
[0, 100)
[50, 60)
```

A direct predecessor-based query on raw intervals can be wrong unless the set is already canonical.

After sealing, the structure becomes:

```text
[0, 100)
```

This makes binary-search-based queries simple, correct, and cache-friendly.

## Design notes

The crate deliberately avoids doing interval merging during concurrent insertion.

Merging during insert would require a multi-node operation:

```text
find neighbors
delete old intervals
insert merged interval
handle races
```

That turns the operation into a small transaction over a concurrent ordered set.

Instead, this crate uses a simpler and safer model:

```text
insert raw intervals concurrently
seal once
query immutable canonical set
```

This fits build-then-query workloads well.

### Trade-offs

This design is not universally optimal.

During the build phase, the builder stores raw intervals rather than canonical intervals. That means repeated, overlapping, or adjacent inserts are not compacted immediately. If many inserted intervals overlap heavily, the builder may temporarily hold significantly more entries than the final sealed set.

Queries are intentionally not provided on the builder. The raw skip-list is not canonical, so predecessor-based interval queries can be incorrect without additional indexing. Users must call `seal()` before using the efficient query APIs.

`seal()` is a required synchronization point. It consumes the builder and performs a full sorted scan to produce the immutable set. This is suitable for build-then-query workloads, but not for workloads that require continuous writes and reads at the same time.

The sealed set is immutable. Updating it requires creating a new builder or constructing a new set from intervals. This avoids complicated concurrent mutation logic, but it is less convenient for online dynamic interval indexes.

The write path also pays the overhead of a concurrent skip list. For small single-threaded workloads, a simple `Vec` followed by sort-and-merge may be faster and simpler. The builder is most useful when many threads insert intervals before a single seal step.

In short:

```text
Better for:
  concurrent build
  one-time sealing
  many read-only queries

Less suitable for:
  continuous online updates
  querying before seal
  tiny single-threaded inputs
  workloads requiring immediate canonicalization
```



## License

MIT OR Apache-2.0
