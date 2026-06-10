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

#[divan::bench(name = "difference_with_set/interleaved_disjoint_64/int_interval_set")]
fn difference_with_set_interleaved_disjoint_64_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, &interleaved_disjoint());
}

#[divan::bench(name = "difference_with_set/interleaved_disjoint_64/range_set_blaze")]
fn difference_with_set_interleaved_disjoint_64_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, &interleaved_disjoint());
}

#[divan::bench(name = "difference_with_set/interleaved_disjoint_64/range_collections")]
fn difference_with_set_interleaved_disjoint_64_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, &interleaved_disjoint());
}

#[divan::bench(name = "difference_with_set/equal_64/int_interval_set")]
fn difference_with_set_equal_64_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, &equal());
}

#[divan::bench(name = "difference_with_set/equal_64/range_set_blaze")]
fn difference_with_set_equal_64_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, &equal());
}

#[divan::bench(name = "difference_with_set/equal_64/range_collections")]
fn difference_with_set_equal_64_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, &equal());
}

#[divan::bench(name = "difference_with_set/trim_right_64/int_interval_set")]
fn difference_with_set_trim_right_64_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, &trim_right());
}

#[divan::bench(name = "difference_with_set/trim_right_64/range_set_blaze")]
fn difference_with_set_trim_right_64_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, &trim_right());
}

#[divan::bench(name = "difference_with_set/trim_right_64/range_collections")]
fn difference_with_set_trim_right_64_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, &trim_right());
}

#[divan::bench(name = "difference_with_set/punch_middle_64/int_interval_set")]
fn difference_with_set_punch_middle_64_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, &punch_middle());
}

#[divan::bench(name = "difference_with_set/punch_middle_64/range_set_blaze")]
fn difference_with_set_punch_middle_64_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, &punch_middle());
}

#[divan::bench(name = "difference_with_set/punch_middle_64/range_collections")]
fn difference_with_set_punch_middle_64_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, &punch_middle());
}

fn lhs_bounds() -> Vec<Bounds> {
    (0..N)
        .map(|i| {
            let start = i as i32 * 16;
            (start, start + 8)
        })
        .collect()
}

fn interleaved_disjoint() -> Vec<Bounds> {
    (0..N)
        .map(|i| {
            let start = i as i32 * 16 + 10;
            (start, start + 4)
        })
        .collect()
}

fn equal() -> Vec<Bounds> {
    lhs_bounds()
}

fn trim_right() -> Vec<Bounds> {
    (0..N)
        .map(|i| {
            let start = i as i32 * 16 + 4;
            (start, start + 8)
        })
        .collect()
}

fn punch_middle() -> Vec<Bounds> {
    (0..N)
        .map(|i| {
            let start = i as i32 * 16 + 2;
            (start, start + 4)
        })
        .collect()
}

fn bench_int_interval_set(bencher: Bencher, rhs_bounds: &[Bounds]) {
    let lhs_bounds = lhs_bounds();
    let lhs = int_interval_set(&lhs_bounds);
    let rhs = int_interval_set(rhs_bounds);

    bencher.bench(|| black_box(black_box(&lhs).difference_with_set(black_box(&rhs))));
}

fn bench_range_set_blaze(bencher: Bencher, rhs_bounds: &[Bounds]) {
    let lhs_bounds = lhs_bounds();
    let lhs = range_set_blaze(&lhs_bounds);
    let rhs = range_set_blaze(rhs_bounds);

    bencher.bench(|| black_box(black_box(&lhs) - black_box(&rhs)));
}

fn bench_range_collections(bencher: Bencher, rhs_bounds: &[Bounds]) {
    let lhs_bounds = lhs_bounds();
    let lhs = range_collections(&lhs_bounds);
    let rhs = range_collections(rhs_bounds);

    bencher.bench(|| black_box(black_box(&lhs).difference::<[i32; 2]>(black_box(&rhs))));
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
