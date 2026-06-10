use divan::{Bencher, black_box};
use int_interval::I32CO;
use int_interval_set::I32COSet;
use range_collections::{RangeSet, RangeSet2};
use range_set_blaze::RangeSetBlaze;

fn main() {
    divan::main();
}

type Bounds = (i32, i32);

const N: usize = 64;

#[divan::bench(name = "union_with_interval/disjoint_before/int_interval_set")]
fn union_with_interval_disjoint_before_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (-16, -8));
}

#[divan::bench(name = "union_with_interval/disjoint_before/range_set_blaze")]
fn union_with_interval_disjoint_before_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, (-16, -8));
}

#[divan::bench(name = "union_with_interval/disjoint_before/range_collections")]
fn union_with_interval_disjoint_before_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, (-16, -8));
}

#[divan::bench(name = "union_with_interval/adjacent_before_first/int_interval_set")]
fn union_with_interval_adjacent_before_first_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (-8, 0));
}

#[divan::bench(name = "union_with_interval/adjacent_before_first/range_set_blaze")]
fn union_with_interval_adjacent_before_first_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, (-8, 0));
}

#[divan::bench(name = "union_with_interval/adjacent_before_first/range_collections")]
fn union_with_interval_adjacent_before_first_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, (-8, 0));
}

#[divan::bench(name = "union_with_interval/contained_middle/int_interval_set")]
fn union_with_interval_contained_middle_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (514, 518));
}

#[divan::bench(name = "union_with_interval/contained_middle/range_set_blaze")]
fn union_with_interval_contained_middle_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, (514, 518));
}

#[divan::bench(name = "union_with_interval/contained_middle/range_collections")]
fn union_with_interval_contained_middle_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, (514, 518));
}

#[divan::bench(name = "union_with_interval/bridge_middle_gap/int_interval_set")]
fn union_with_interval_bridge_middle_gap_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (504, 512));
}

#[divan::bench(name = "union_with_interval/bridge_middle_gap/range_set_blaze")]
fn union_with_interval_bridge_middle_gap_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, (504, 512));
}

#[divan::bench(name = "union_with_interval/bridge_middle_gap/range_collections")]
fn union_with_interval_bridge_middle_gap_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, (504, 512));
}

#[divan::bench(name = "union_with_interval/bridge_many_middle/int_interval_set")]
fn union_with_interval_bridge_many_middle_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (498, 566));
}

#[divan::bench(name = "union_with_interval/bridge_many_middle/range_set_blaze")]
fn union_with_interval_bridge_many_middle_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, (498, 566));
}

#[divan::bench(name = "union_with_interval/bridge_many_middle/range_collections")]
fn union_with_interval_bridge_many_middle_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, (498, 566));
}

fn bounds() -> Vec<Bounds> {
    (0..N)
        .map(|i| {
            let start = i as i32 * 16;
            (start, start + 8)
        })
        .collect()
}

fn bench_int_interval_set(bencher: Bencher, query: Bounds) {
    let bounds = bounds();
    let set = int_interval_set(&bounds);
    let query = I32CO::try_new(query.0, query.1).unwrap();

    bencher.bench(|| black_box(black_box(&set).union_with_interval(black_box(query))));
}

fn bench_range_set_blaze(bencher: Bencher, query: Bounds) {
    let bounds = bounds();
    let set = range_set_blaze(&bounds);
    let query = RangeSetBlaze::from_iter([query.0..=(query.1 - 1)]);

    bencher.bench(|| black_box(black_box(&set) | black_box(&query)));
}

fn bench_range_collections(bencher: Bencher, query: Bounds) {
    let bounds = bounds();
    let set = range_collections(&bounds);
    let query: RangeSet2<i32> = RangeSet::from(query.0..query.1);

    bencher.bench(|| black_box(black_box(&set).union::<[i32; 2]>(black_box(&query))));
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
