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

#[divan::bench(name = "contains_interval/contained_first/int_interval_set")]
fn contains_interval_contained_first_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (2, 10));
}

#[divan::bench(name = "contains_interval/contained_first/range_set_blaze")]
fn contains_interval_contained_first_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, (2, 10));
}

#[divan::bench(name = "contains_interval/contained_first/range_collections")]
fn contains_interval_contained_first_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, (2, 10));
}

#[divan::bench(name = "contains_interval/contained_middle/int_interval_set")]
fn contains_interval_contained_middle_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (514, 522));
}

#[divan::bench(name = "contains_interval/contained_middle/range_set_blaze")]
fn contains_interval_contained_middle_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, (514, 522));
}

#[divan::bench(name = "contains_interval/contained_middle/range_collections")]
fn contains_interval_contained_middle_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, (514, 522));
}

#[divan::bench(name = "contains_interval/contained_last/int_interval_set")]
fn contains_interval_contained_last_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (1010, 1018));
}

#[divan::bench(name = "contains_interval/contained_last/range_set_blaze")]
fn contains_interval_contained_last_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, (1010, 1018));
}

#[divan::bench(name = "contains_interval/contained_last/range_collections")]
fn contains_interval_contained_last_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, (1010, 1018));
}

#[divan::bench(name = "contains_interval/crosses_gap/int_interval_set")]
fn contains_interval_crosses_gap_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (520, 530));
}

#[divan::bench(name = "contains_interval/crosses_gap/range_set_blaze")]
fn contains_interval_crosses_gap_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, (520, 530));
}

#[divan::bench(name = "contains_interval/crosses_gap/range_collections")]
fn contains_interval_crosses_gap_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, (520, 530));
}

#[divan::bench(name = "contains_interval/inside_gap/int_interval_set")]
fn contains_interval_inside_gap_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (524, 528));
}

#[divan::bench(name = "contains_interval/inside_gap/range_set_blaze")]
fn contains_interval_inside_gap_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, (524, 528));
}

#[divan::bench(name = "contains_interval/inside_gap/range_collections")]
fn contains_interval_inside_gap_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, (524, 528));
}

#[divan::bench(name = "contains_interval/outside_right/int_interval_set")]
fn contains_interval_outside_right_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (1020, 1028));
}

#[divan::bench(name = "contains_interval/outside_right/range_set_blaze")]
fn contains_interval_outside_right_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, (1020, 1028));
}

#[divan::bench(name = "contains_interval/outside_right/range_collections")]
fn contains_interval_outside_right_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, (1020, 1028));
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

    bencher.bench(|| black_box(&set).contains_interval(black_box(query)));
}

fn bench_range_set_blaze(bencher: Bencher, query: Bounds) {
    let bounds = source_bounds();
    let set = range_set_blaze(&bounds);
    let query = RangeSetBlaze::from(query.0..=(query.1 - 1));

    bencher.bench(|| black_box(&set).is_superset(black_box(&query)));
}

fn bench_range_collections(bencher: Bencher, query: Bounds) {
    let bounds = source_bounds();
    let set = range_collections(&bounds);
    let query = RangeSet2::from(query.0..query.1);

    bencher.bench(|| black_box(&set).is_superset(black_box(query.as_ref())));
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
