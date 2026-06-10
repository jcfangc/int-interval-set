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
const STRIDE: i32 = 8;
const WIDTH: i32 = 4;

#[divan::bench(name = "difference_with_interval/disjoint_before/int_interval_set")]
fn difference_with_interval_disjoint_before_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (-16, -8));
}

#[divan::bench(name = "difference_with_interval/disjoint_before/range_set_blaze")]
fn difference_with_interval_disjoint_before_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, (-16, -8));
}

#[divan::bench(name = "difference_with_interval/disjoint_before/range_collections")]
fn difference_with_interval_disjoint_before_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, (-16, -8));
}

#[divan::bench(name = "difference_with_interval/disjoint_gap_middle/int_interval_set")]
fn difference_with_interval_disjoint_gap_middle_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (260, 264));
}

#[divan::bench(name = "difference_with_interval/disjoint_gap_middle/range_set_blaze")]
fn difference_with_interval_disjoint_gap_middle_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, (260, 264));
}

#[divan::bench(name = "difference_with_interval/disjoint_gap_middle/range_collections")]
fn difference_with_interval_disjoint_gap_middle_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, (260, 264));
}

#[divan::bench(name = "difference_with_interval/remove_first_exact/int_interval_set")]
fn difference_with_interval_remove_first_exact_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (0, 4));
}

#[divan::bench(name = "difference_with_interval/remove_first_exact/range_set_blaze")]
fn difference_with_interval_remove_first_exact_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, (0, 4));
}

#[divan::bench(name = "difference_with_interval/remove_first_exact/range_collections")]
fn difference_with_interval_remove_first_exact_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, (0, 4));
}

#[divan::bench(name = "difference_with_interval/trim_middle_left/int_interval_set")]
fn difference_with_interval_trim_middle_left_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (256, 258));
}

#[divan::bench(name = "difference_with_interval/trim_middle_left/range_set_blaze")]
fn difference_with_interval_trim_middle_left_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, (256, 258));
}

#[divan::bench(name = "difference_with_interval/trim_middle_left/range_collections")]
fn difference_with_interval_trim_middle_left_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, (256, 258));
}

#[divan::bench(name = "difference_with_interval/split_middle/int_interval_set")]
fn difference_with_interval_split_middle_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (257, 259));
}

#[divan::bench(name = "difference_with_interval/split_middle/range_set_blaze")]
fn difference_with_interval_split_middle_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, (257, 259));
}

#[divan::bench(name = "difference_with_interval/split_middle/range_collections")]
fn difference_with_interval_split_middle_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, (257, 259));
}

#[divan::bench(name = "difference_with_interval/remove_middle_span/int_interval_set")]
fn difference_with_interval_remove_middle_span_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (16 * STRIDE, 48 * STRIDE));
}

#[divan::bench(name = "difference_with_interval/remove_middle_span/range_set_blaze")]
fn difference_with_interval_remove_middle_span_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, (16 * STRIDE, 48 * STRIDE));
}

#[divan::bench(name = "difference_with_interval/remove_middle_span/range_collections")]
fn difference_with_interval_remove_middle_span_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, (16 * STRIDE, 48 * STRIDE));
}

#[divan::bench(name = "difference_with_interval/clip_middle_span/int_interval_set")]
fn difference_with_interval_clip_middle_span_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (16 * STRIDE + 2, 48 * STRIDE + 2));
}

#[divan::bench(name = "difference_with_interval/clip_middle_span/range_set_blaze")]
fn difference_with_interval_clip_middle_span_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, (16 * STRIDE + 2, 48 * STRIDE + 2));
}

#[divan::bench(name = "difference_with_interval/clip_middle_span/range_collections")]
fn difference_with_interval_clip_middle_span_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, (16 * STRIDE + 2, 48 * STRIDE + 2));
}

fn bench_int_interval_set(bencher: Bencher, query: Bounds) {
    let bounds = set_bounds();
    let set = build_int_interval_set(&bounds);
    let query = I32CO::try_new(query.0, query.1).unwrap();

    bencher.bench(|| black_box(&set).difference_with_interval(black_box(query)));
}

fn bench_range_set_blaze(bencher: Bencher, query: Bounds) {
    let bounds = set_bounds();
    let set = build_range_set_blaze(&bounds);
    let query = RangeSetBlaze::from(query.0..=(query.1 - 1));

    bencher.bench(|| black_box(&set) - black_box(&query));
}

fn bench_range_collections(bencher: Bencher, query: Bounds) {
    let bounds = set_bounds();
    let set = build_range_collections(&bounds);
    let query = RangeSet2::from(query.0..query.1);

    bencher.bench(|| black_box(&set) - black_box(&query));
}

/// Produces `[0, 4), [8, 12), ..., [504, 508)`.
fn set_bounds() -> Vec<Bounds> {
    (0..N)
        .map(|i| {
            let start = i as i32 * STRIDE;
            (start, start + WIDTH)
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
