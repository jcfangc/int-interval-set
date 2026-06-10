use divan::{Bencher, black_box};
use int_interval::I32CO;
use int_interval_set::I32COSet;
use range_collections::range_set::{RangeSet, RangeSet2};
use range_set_blaze::RangeSetBlaze;

fn main() {
    divan::main();
}

type Bounds = (i32, i32);

const INTERVALS: usize = 64;

#[divan::bench(name = "intersects_interval/before_all/int_interval_set")]
fn intersects_interval_before_all_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (-16, -8));
}

#[divan::bench(name = "intersects_interval/before_all/range_set_blaze")]
fn intersects_interval_before_all_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, (-16, -8));
}

#[divan::bench(name = "intersects_interval/before_all/range_collections")]
fn intersects_interval_before_all_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, (-16, -8));
}

#[divan::bench(name = "intersects_interval/hit_first/int_interval_set")]
fn intersects_interval_hit_first_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (1, 3));
}

#[divan::bench(name = "intersects_interval/hit_first/range_set_blaze")]
fn intersects_interval_hit_first_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, (1, 3));
}

#[divan::bench(name = "intersects_interval/hit_first/range_collections")]
fn intersects_interval_hit_first_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, (1, 3));
}

#[divan::bench(name = "intersects_interval/adjacent_left_middle/int_interval_set")]
fn intersects_interval_adjacent_left_middle_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (252, 256));
}

#[divan::bench(name = "intersects_interval/adjacent_left_middle/range_set_blaze")]
fn intersects_interval_adjacent_left_middle_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, (252, 256));
}

#[divan::bench(name = "intersects_interval/adjacent_left_middle/range_collections")]
fn intersects_interval_adjacent_left_middle_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, (252, 256));
}

#[divan::bench(name = "intersects_interval/gap_middle/int_interval_set")]
fn intersects_interval_gap_middle_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (260, 264));
}

#[divan::bench(name = "intersects_interval/gap_middle/range_set_blaze")]
fn intersects_interval_gap_middle_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, (260, 264));
}

#[divan::bench(name = "intersects_interval/gap_middle/range_collections")]
fn intersects_interval_gap_middle_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, (260, 264));
}

#[divan::bench(name = "intersects_interval/hit_middle/int_interval_set")]
fn intersects_interval_hit_middle_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (257, 259));
}

#[divan::bench(name = "intersects_interval/hit_middle/range_set_blaze")]
fn intersects_interval_hit_middle_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, (257, 259));
}

#[divan::bench(name = "intersects_interval/hit_middle/range_collections")]
fn intersects_interval_hit_middle_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, (257, 259));
}

#[divan::bench(name = "intersects_interval/span_middle_gap/int_interval_set")]
fn intersects_interval_span_middle_gap_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (258, 266));
}

#[divan::bench(name = "intersects_interval/span_middle_gap/range_set_blaze")]
fn intersects_interval_span_middle_gap_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, (258, 266));
}

#[divan::bench(name = "intersects_interval/span_middle_gap/range_collections")]
fn intersects_interval_span_middle_gap_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, (258, 266));
}

#[divan::bench(name = "intersects_interval/adjacent_right_last/int_interval_set")]
fn intersects_interval_adjacent_right_last_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (508, 512));
}

#[divan::bench(name = "intersects_interval/adjacent_right_last/range_set_blaze")]
fn intersects_interval_adjacent_right_last_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, (508, 512));
}

#[divan::bench(name = "intersects_interval/adjacent_right_last/range_collections")]
fn intersects_interval_adjacent_right_last_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, (508, 512));
}

#[divan::bench(name = "intersects_interval/after_all/int_interval_set")]
fn intersects_interval_after_all_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (520, 528));
}

#[divan::bench(name = "intersects_interval/after_all/range_set_blaze")]
fn intersects_interval_after_all_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, (520, 528));
}

#[divan::bench(name = "intersects_interval/after_all/range_collections")]
fn intersects_interval_after_all_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, (520, 528));
}

fn bounds() -> Vec<Bounds> {
    (0..INTERVALS)
        .map(|i| {
            let start = i as i32 * 8;
            (start, start + 4)
        })
        .collect()
}

fn int_interval_set(bounds: &[Bounds]) -> I32COSet {
    bounds
        .iter()
        .map(|&(start, end_excl)| I32CO::try_new(start, end_excl).unwrap())
        .collect()
}

fn range_set_blaze(bounds: &[Bounds]) -> RangeSetBlaze<i32> {
    bounds
        .iter()
        .map(|&(start, end_excl)| start..=(end_excl - 1))
        .collect()
}

fn range_collections(bounds: &[Bounds]) -> RangeSet2<i32> {
    let mut set = RangeSet2::empty();

    for &(start, end_excl) in bounds {
        set |= RangeSet::from(start..end_excl);
    }

    set
}

fn bench_int_interval_set(bencher: Bencher, query: Bounds) {
    let source = int_interval_set(&bounds());
    let query = I32CO::try_new(query.0, query.1).unwrap();

    bencher.bench(|| black_box(&source).intersects_interval(black_box(query)));
}

fn bench_range_set_blaze(bencher: Bencher, query: Bounds) {
    let source = range_set_blaze(&bounds());
    let query = RangeSetBlaze::from_iter([query.0..=(query.1 - 1)]);

    bencher.bench(|| !black_box(&source).is_disjoint(black_box(&query)));
}

fn bench_range_collections(bencher: Bencher, query: Bounds) {
    let source = range_collections(&bounds());
    let query: RangeSet2<i32> = RangeSet::from(query.0..query.1);

    bencher.bench(|| black_box(&source).intersects(black_box(&query)));
}
