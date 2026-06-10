use divan::{Bencher, black_box};
use int_interval::I32CO;
use int_interval_set::I32COSet;
use range_collections::RangeSet2;
use range_set_blaze::RangeSetBlaze;

fn main() {
    divan::main();
}

type Bounds = (i32, i32);

const N: usize = 64;

#[divan::bench(name = "contains_point/hit_first/int_interval_set")]
fn contains_point_hit_first_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, 1);
}

#[divan::bench(name = "contains_point/hit_first/range_set_blaze")]
fn contains_point_hit_first_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, 1);
}

#[divan::bench(name = "contains_point/hit_first/range_collections")]
fn contains_point_hit_first_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, 1);
}

#[divan::bench(name = "contains_point/gap_first/int_interval_set")]
fn contains_point_gap_first_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, 2);
}

#[divan::bench(name = "contains_point/gap_first/range_set_blaze")]
fn contains_point_gap_first_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, 2);
}

#[divan::bench(name = "contains_point/gap_first/range_collections")]
fn contains_point_gap_first_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, 2);
}

#[divan::bench(name = "contains_point/hit_middle/int_interval_set")]
fn contains_point_hit_middle_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, 129);
}

#[divan::bench(name = "contains_point/hit_middle/range_set_blaze")]
fn contains_point_hit_middle_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, 129);
}

#[divan::bench(name = "contains_point/hit_middle/range_collections")]
fn contains_point_hit_middle_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, 129);
}

#[divan::bench(name = "contains_point/gap_middle/int_interval_set")]
fn contains_point_gap_middle_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, 130);
}

#[divan::bench(name = "contains_point/gap_middle/range_set_blaze")]
fn contains_point_gap_middle_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, 130);
}

#[divan::bench(name = "contains_point/gap_middle/range_collections")]
fn contains_point_gap_middle_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, 130);
}

#[divan::bench(name = "contains_point/hit_last/int_interval_set")]
fn contains_point_hit_last_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, 253);
}

#[divan::bench(name = "contains_point/hit_last/range_set_blaze")]
fn contains_point_hit_last_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, 253);
}

#[divan::bench(name = "contains_point/hit_last/range_collections")]
fn contains_point_hit_last_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, 253);
}

#[divan::bench(name = "contains_point/gap_last/int_interval_set")]
fn contains_point_gap_last_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, 254);
}

#[divan::bench(name = "contains_point/gap_last/range_set_blaze")]
fn contains_point_gap_last_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, 254);
}

#[divan::bench(name = "contains_point/gap_last/range_collections")]
fn contains_point_gap_last_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, 254);
}

#[divan::bench(name = "contains_point/before_all/int_interval_set")]
fn contains_point_before_all_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, -1);
}

#[divan::bench(name = "contains_point/before_all/range_set_blaze")]
fn contains_point_before_all_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, -1);
}

#[divan::bench(name = "contains_point/before_all/range_collections")]
fn contains_point_before_all_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, -1);
}

#[divan::bench(name = "contains_point/after_all/int_interval_set")]
fn contains_point_after_all_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, 256);
}

#[divan::bench(name = "contains_point/after_all/range_set_blaze")]
fn contains_point_after_all_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, 256);
}

#[divan::bench(name = "contains_point/after_all/range_collections")]
fn contains_point_after_all_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, 256);
}

fn bench_int_interval_set(bencher: Bencher, point: i32) {
    let set = build_int_interval_set(&bounds());
    bencher.bench(|| black_box(&set).contains_point(black_box(point)));
}

fn bench_range_set_blaze(bencher: Bencher, point: i32) {
    let set = build_range_set_blaze(&bounds());
    bencher.bench(|| black_box(&set).contains(black_box(point)));
}

fn bench_range_collections(bencher: Bencher, point: i32) {
    let set = build_range_collections(&bounds());
    bencher.bench(|| black_box(&set).contains(black_box(&point)));
}

/// Produces 64 non-adjacent intervals: `[0, 2), [4, 6), ..., [252, 254)`.
fn bounds() -> Vec<Bounds> {
    (0..N)
        .map(|i| {
            let start = i as i32 * 4;
            (start, start + 2)
        })
        .collect()
}

#[inline]
fn build_int_interval_set(bounds: &[Bounds]) -> I32COSet {
    bounds
        .iter()
        .map(|&(start, end_excl)| I32CO::try_new(start, end_excl).unwrap())
        .collect()
}

#[inline]
fn build_range_set_blaze(bounds: &[Bounds]) -> RangeSetBlaze<i32> {
    bounds
        .iter()
        .map(|&(start, end_excl)| start..=(end_excl - 1))
        .collect()
}

#[inline]
fn build_range_collections(bounds: &[Bounds]) -> RangeSet2<i32> {
    let mut set = RangeSet2::empty();

    for &(start, end_excl) in bounds {
        set |= RangeSet2::from(start..end_excl);
    }

    set
}
