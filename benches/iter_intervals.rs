use divan::{Bencher, black_box};
use int_interval::I32CO;
use int_interval_set::I32COSet;
use range_collections::{RangeSet, RangeSet2};
use range_set_blaze::RangeSetBlaze;

fn main() {
    divan::main();
}

type Bounds = (i32, i32);

const N_SMALL: usize = 64;
const N_LARGE: usize = 1024;

#[divan::bench(name = "iter_intervals/merged_64_to_1/int_interval_set")]
fn iter_intervals_merged_64_to_1_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, &adjacent_bounds(N_SMALL));
}

#[divan::bench(name = "iter_intervals/merged_64_to_1/range_set_blaze")]
fn iter_intervals_merged_64_to_1_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, &adjacent_bounds(N_SMALL));
}

#[divan::bench(name = "iter_intervals/merged_64_to_1/range_collections")]
fn iter_intervals_merged_64_to_1_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, &adjacent_bounds(N_SMALL));
}

#[divan::bench(name = "iter_intervals/sparse_64/int_interval_set")]
fn iter_intervals_sparse_64_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, &sparse_bounds(N_SMALL));
}

#[divan::bench(name = "iter_intervals/sparse_64/range_set_blaze")]
fn iter_intervals_sparse_64_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, &sparse_bounds(N_SMALL));
}

#[divan::bench(name = "iter_intervals/sparse_64/range_collections")]
fn iter_intervals_sparse_64_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, &sparse_bounds(N_SMALL));
}

#[divan::bench(name = "iter_intervals/sparse_1024/int_interval_set")]
fn iter_intervals_sparse_1024_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, &sparse_bounds(N_LARGE));
}

#[divan::bench(name = "iter_intervals/sparse_1024/range_set_blaze")]
fn iter_intervals_sparse_1024_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, &sparse_bounds(N_LARGE));
}

#[divan::bench(name = "iter_intervals/sparse_1024/range_collections")]
fn iter_intervals_sparse_1024_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, &sparse_bounds(N_LARGE));
}

fn sparse_bounds(n: usize) -> Vec<Bounds> {
    (0..n)
        .map(|i| {
            let start = i as i32 * 8;
            (start, start + 4)
        })
        .collect()
}

fn adjacent_bounds(n: usize) -> Vec<Bounds> {
    (0..n)
        .map(|i| {
            let start = i as i32 * 4;
            (start, start + 4)
        })
        .collect()
}

fn bench_int_interval_set(bencher: Bencher, bounds: &[Bounds]) {
    let set = int_interval_set(bounds);

    bencher.bench(|| {
        for interval in black_box(&set).iter_intervals() {
            black_box(interval);
        }
    });
}

fn bench_range_set_blaze(bencher: Bencher, bounds: &[Bounds]) {
    let set = range_set_blaze(bounds);

    bencher.bench(|| {
        for interval in black_box(&set).ranges() {
            black_box(interval);
        }
    });
}

fn bench_range_collections(bencher: Bencher, bounds: &[Bounds]) {
    let set = range_collections(bounds);

    bencher.bench(|| {
        for interval in black_box(&set).iter() {
            black_box(interval);
        }
    });
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
    let mut set = RangeSet2::empty();

    for &(start, end_excl) in bounds {
        set |= RangeSet::from(start..end_excl);
    }

    set
}
