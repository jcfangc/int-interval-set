use divan::{Bencher, black_box};
use int_interval::I32CO;
use int_interval_set::I32COSet;
use range_collections::RangeSet2;
use range_set_blaze::RangeSetBlaze;

fn main() {
    divan::main();
}

type Bounds = (i32, i32);

const N: i32 = 64;

#[divan::bench(name = "union_with_set/equal_64/int_interval_set")]
fn union_with_set_equal_64_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, &lhs_bounds(), &equal_rhs());
}

#[divan::bench(name = "union_with_set/equal_64/range_set_blaze")]
fn union_with_set_equal_64_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, &lhs_bounds(), &equal_rhs());
}

#[divan::bench(name = "union_with_set/equal_64/range_collections")]
fn union_with_set_equal_64_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, &lhs_bounds(), &equal_rhs());
}

#[divan::bench(name = "union_with_set/contained_64/int_interval_set")]
fn union_with_set_contained_64_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, &lhs_bounds(), &contained_rhs());
}

#[divan::bench(name = "union_with_set/contained_64/range_set_blaze")]
fn union_with_set_contained_64_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, &lhs_bounds(), &contained_rhs());
}

#[divan::bench(name = "union_with_set/contained_64/range_collections")]
fn union_with_set_contained_64_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, &lhs_bounds(), &contained_rhs());
}

#[divan::bench(name = "union_with_set/overlapping_64/int_interval_set")]
fn union_with_set_overlapping_64_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, &lhs_bounds(), &overlapping_rhs());
}

#[divan::bench(name = "union_with_set/overlapping_64/range_set_blaze")]
fn union_with_set_overlapping_64_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, &lhs_bounds(), &overlapping_rhs());
}

#[divan::bench(name = "union_with_set/overlapping_64/range_collections")]
fn union_with_set_overlapping_64_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, &lhs_bounds(), &overlapping_rhs());
}

#[divan::bench(name = "union_with_set/interleaved_disjoint_64/int_interval_set")]
fn union_with_set_interleaved_disjoint_64_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, &lhs_bounds(), &interleaved_disjoint_rhs());
}

#[divan::bench(name = "union_with_set/interleaved_disjoint_64/range_set_blaze")]
fn union_with_set_interleaved_disjoint_64_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, &lhs_bounds(), &interleaved_disjoint_rhs());
}

#[divan::bench(name = "union_with_set/interleaved_disjoint_64/range_collections")]
fn union_with_set_interleaved_disjoint_64_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, &lhs_bounds(), &interleaved_disjoint_rhs());
}

#[divan::bench(name = "union_with_set/adjacent_bridge_64/int_interval_set")]
fn union_with_set_adjacent_bridge_64_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, &lhs_bounds(), &adjacent_bridge_rhs());
}

#[divan::bench(name = "union_with_set/adjacent_bridge_64/range_set_blaze")]
fn union_with_set_adjacent_bridge_64_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, &lhs_bounds(), &adjacent_bridge_rhs());
}

#[divan::bench(name = "union_with_set/adjacent_bridge_64/range_collections")]
fn union_with_set_adjacent_bridge_64_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, &lhs_bounds(), &adjacent_bridge_rhs());
}

fn bench_int_interval_set(bencher: Bencher, lhs: &[Bounds], rhs: &[Bounds]) {
    let lhs = int_interval_set(lhs);
    let rhs = int_interval_set(rhs);

    bencher.bench(|| black_box(black_box(&lhs).union_with_set(black_box(&rhs))));
}

fn bench_range_set_blaze(bencher: Bencher, lhs: &[Bounds], rhs: &[Bounds]) {
    let lhs = range_set_blaze(lhs);
    let rhs = range_set_blaze(rhs);

    bencher.bench(|| black_box(black_box(&lhs) | black_box(&rhs)));
}

fn bench_range_collections(bencher: Bencher, lhs: &[Bounds], rhs: &[Bounds]) {
    let lhs = range_collections(lhs);
    let rhs = range_collections(rhs);

    bencher.bench(|| black_box(black_box(&lhs) | black_box(&rhs)));
}

/// Produces the left-hand set: 64 intervals of length 8 separated by gaps of length 8.
///
/// Layout: `[0, 8), [16, 24), ..., [1008, 1016)`.
fn lhs_bounds() -> Vec<Bounds> {
    (0..N)
        .map(|i| {
            let start = i * 16;
            (start, start + 8)
        })
        .collect()
}

fn equal_rhs() -> Vec<Bounds> {
    lhs_bounds()
}

fn contained_rhs() -> Vec<Bounds> {
    (0..N)
        .map(|i| {
            let start = i * 16 + 2;
            (start, start + 4)
        })
        .collect()
}

fn overlapping_rhs() -> Vec<Bounds> {
    (0..N)
        .map(|i| {
            let start = i * 16 + 4;
            (start, start + 8)
        })
        .collect()
}

fn interleaved_disjoint_rhs() -> Vec<Bounds> {
    (0..N)
        .map(|i| {
            let start = i * 16 + 10;
            (start, start + 4)
        })
        .collect()
}

fn adjacent_bridge_rhs() -> Vec<Bounds> {
    (0..N)
        .map(|i| {
            let start = i * 16 + 8;
            (start, start + 8)
        })
        .collect()
}

#[inline]
fn int_interval_set(bounds: &[Bounds]) -> I32COSet {
    bounds
        .iter()
        .map(|&(start, end_excl)| I32CO::try_new(start, end_excl).unwrap())
        .collect()
}

#[inline]
fn range_set_blaze(bounds: &[Bounds]) -> RangeSetBlaze<i32> {
    bounds
        .iter()
        .map(|&(start, end_excl)| start..=(end_excl - 1))
        .collect()
}

#[inline]
fn range_collections(bounds: &[Bounds]) -> RangeSet2<i32> {
    let (&(start, end_excl), rest) = bounds.split_first().unwrap();
    let mut set = RangeSet2::from(start..end_excl);

    for &(start, end_excl) in rest {
        set |= RangeSet2::from(start..end_excl);
    }

    set
}
