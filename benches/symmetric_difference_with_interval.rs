use divan::{Bencher, black_box};
use int_interval::I32CO;
use int_interval_set::I32COSet;
use range_collections::RangeSet2;
use range_set_blaze::RangeSetBlaze;

fn main() {
    divan::main();
}

type Bounds = (i32, i32);

const INTERVAL_COUNT: i32 = 64;

#[divan::bench(name = "symmetric_difference_with_interval/disjoint_left/int_interval_set")]
fn symmetric_difference_with_interval_disjoint_left_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (-32, -16));
}

#[divan::bench(name = "symmetric_difference_with_interval/disjoint_left/range_set_blaze")]
fn symmetric_difference_with_interval_disjoint_left_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, (-32, -16));
}

#[divan::bench(name = "symmetric_difference_with_interval/disjoint_left/range_collections")]
fn symmetric_difference_with_interval_disjoint_left_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, (-32, -16));
}

#[divan::bench(name = "symmetric_difference_with_interval/equal_middle/int_interval_set")]
fn symmetric_difference_with_interval_equal_middle_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (512, 524));
}

#[divan::bench(name = "symmetric_difference_with_interval/equal_middle/range_set_blaze")]
fn symmetric_difference_with_interval_equal_middle_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, (512, 524));
}

#[divan::bench(name = "symmetric_difference_with_interval/equal_middle/range_collections")]
fn symmetric_difference_with_interval_equal_middle_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, (512, 524));
}

#[divan::bench(name = "symmetric_difference_with_interval/contained_middle/int_interval_set")]
fn symmetric_difference_with_interval_contained_middle_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (514, 522));
}

#[divan::bench(name = "symmetric_difference_with_interval/contained_middle/range_set_blaze")]
fn symmetric_difference_with_interval_contained_middle_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, (514, 522));
}

#[divan::bench(name = "symmetric_difference_with_interval/contained_middle/range_collections")]
fn symmetric_difference_with_interval_contained_middle_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, (514, 522));
}

#[divan::bench(name = "symmetric_difference_with_interval/crosses_gap_middle/int_interval_set")]
fn symmetric_difference_with_interval_crosses_gap_middle_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (520, 530));
}

#[divan::bench(name = "symmetric_difference_with_interval/crosses_gap_middle/range_set_blaze")]
fn symmetric_difference_with_interval_crosses_gap_middle_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, (520, 530));
}

#[divan::bench(name = "symmetric_difference_with_interval/crosses_gap_middle/range_collections")]
fn symmetric_difference_with_interval_crosses_gap_middle_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, (520, 530));
}

#[divan::bench(name = "symmetric_difference_with_interval/covers_middle_16/int_interval_set")]
fn symmetric_difference_with_interval_covers_middle_16_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (384, 636));
}

#[divan::bench(name = "symmetric_difference_with_interval/covers_middle_16/range_set_blaze")]
fn symmetric_difference_with_interval_covers_middle_16_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, (384, 636));
}

#[divan::bench(name = "symmetric_difference_with_interval/covers_middle_16/range_collections")]
fn symmetric_difference_with_interval_covers_middle_16_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, (384, 636));
}

#[divan::bench(name = "symmetric_difference_with_interval/covers_all/int_interval_set")]
fn symmetric_difference_with_interval_covers_all_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (-16, 1032));
}

#[divan::bench(name = "symmetric_difference_with_interval/covers_all/range_set_blaze")]
fn symmetric_difference_with_interval_covers_all_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, (-16, 1032));
}

#[divan::bench(name = "symmetric_difference_with_interval/covers_all/range_collections")]
fn symmetric_difference_with_interval_covers_all_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, (-16, 1032));
}

fn source_bounds() -> Vec<Bounds> {
    (0..INTERVAL_COUNT)
        .map(|i| {
            let start = i * 16;
            (start, start + 12)
        })
        .collect()
}

fn bench_int_interval_set(bencher: Bencher, query: Bounds) {
    let bounds = source_bounds();
    let set = int_interval_set(&bounds);
    let query = I32CO::try_new(query.0, query.1).unwrap();

    bencher
        .bench(|| black_box(black_box(&set).symmetric_difference_with_interval(black_box(query))));
}

fn bench_range_set_blaze(bencher: Bencher, query: Bounds) {
    let bounds = source_bounds();
    let set = range_set_blaze(&bounds);
    let query = RangeSetBlaze::from(query.0..=(query.1 - 1));

    bencher.bench(|| black_box(black_box(&set) ^ black_box(&query)));
}

fn bench_range_collections(bencher: Bencher, query: Bounds) {
    let bounds = source_bounds();
    let set = range_collections(&bounds);
    let query = RangeSet2::from(query.0..query.1);

    bencher.bench(|| black_box(black_box(&set) ^ black_box(&query)));
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
