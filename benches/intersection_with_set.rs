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

#[divan::bench(name = "intersection_with_set/disjoint_64x64/int_interval_set")]
fn intersection_with_set_disjoint_64x64_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, &lhs_bounds(), &disjoint_rhs());
}

#[divan::bench(name = "intersection_with_set/disjoint_64x64/range_set_blaze")]
fn intersection_with_set_disjoint_64x64_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, &lhs_bounds(), &disjoint_rhs());
}

#[divan::bench(name = "intersection_with_set/disjoint_64x64/range_collections")]
fn intersection_with_set_disjoint_64x64_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, &lhs_bounds(), &disjoint_rhs());
}

#[divan::bench(name = "intersection_with_set/equal_64x64/int_interval_set")]
fn intersection_with_set_equal_64x64_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, &lhs_bounds(), &equal_rhs());
}

#[divan::bench(name = "intersection_with_set/equal_64x64/range_set_blaze")]
fn intersection_with_set_equal_64x64_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, &lhs_bounds(), &equal_rhs());
}

#[divan::bench(name = "intersection_with_set/equal_64x64/range_collections")]
fn intersection_with_set_equal_64x64_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, &lhs_bounds(), &equal_rhs());
}

#[divan::bench(name = "intersection_with_set/partial_overlap_64x64/int_interval_set")]
fn intersection_with_set_partial_overlap_64x64_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, &lhs_bounds(), &partial_overlap_rhs());
}

#[divan::bench(name = "intersection_with_set/partial_overlap_64x64/range_set_blaze")]
fn intersection_with_set_partial_overlap_64x64_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, &lhs_bounds(), &partial_overlap_rhs());
}

#[divan::bench(name = "intersection_with_set/partial_overlap_64x64/range_collections")]
fn intersection_with_set_partial_overlap_64x64_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, &lhs_bounds(), &partial_overlap_rhs());
}

#[divan::bench(name = "intersection_with_set/alternating_64x32/int_interval_set")]
fn intersection_with_set_alternating_64x32_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, &lhs_bounds(), &alternating_rhs());
}

#[divan::bench(name = "intersection_with_set/alternating_64x32/range_set_blaze")]
fn intersection_with_set_alternating_64x32_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, &lhs_bounds(), &alternating_rhs());
}

#[divan::bench(name = "intersection_with_set/alternating_64x32/range_collections")]
fn intersection_with_set_alternating_64x32_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, &lhs_bounds(), &alternating_rhs());
}

#[divan::bench(name = "intersection_with_set/broad_middle_64x1/int_interval_set")]
fn intersection_with_set_broad_middle_64x1_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, &lhs_bounds(), &broad_middle_rhs());
}

#[divan::bench(name = "intersection_with_set/broad_middle_64x1/range_set_blaze")]
fn intersection_with_set_broad_middle_64x1_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, &lhs_bounds(), &broad_middle_rhs());
}

#[divan::bench(name = "intersection_with_set/broad_middle_64x1/range_collections")]
fn intersection_with_set_broad_middle_64x1_range_collections(bencher: Bencher) {
    bench_range_collections(bencher, &lhs_bounds(), &broad_middle_rhs());
}

fn bench_int_interval_set(bencher: Bencher, lhs: &[Bounds], rhs: &[Bounds]) {
    let lhs = build_int_interval_set(lhs);
    let rhs = build_int_interval_set(rhs);

    bencher.bench(|| black_box(&lhs).intersection_with_set(black_box(&rhs)));
}

fn bench_range_set_blaze(bencher: Bencher, lhs: &[Bounds], rhs: &[Bounds]) {
    let lhs = build_range_set_blaze(lhs);
    let rhs = build_range_set_blaze(rhs);

    bencher.bench(|| black_box(&lhs) & black_box(&rhs));
}

fn bench_range_collections(bencher: Bencher, lhs: &[Bounds], rhs: &[Bounds]) {
    let lhs = build_range_collections(lhs);
    let rhs = build_range_collections(rhs);

    bencher.bench(|| black_box(&lhs) & black_box(&rhs));
}

fn lhs_bounds() -> Vec<Bounds> {
    (0..N)
        .map(|i| {
            let start = i as i32 * STRIDE;
            (start, start + 4)
        })
        .collect()
}

fn disjoint_rhs() -> Vec<Bounds> {
    (0..N)
        .map(|i| {
            let start = i as i32 * STRIDE + 4;
            (start, start + 4)
        })
        .collect()
}

fn equal_rhs() -> Vec<Bounds> {
    lhs_bounds()
}

fn partial_overlap_rhs() -> Vec<Bounds> {
    (0..N)
        .map(|i| {
            let start = i as i32 * STRIDE + 2;
            (start, start + 4)
        })
        .collect()
}

fn alternating_rhs() -> Vec<Bounds> {
    (0..N)
        .step_by(2)
        .map(|i| {
            let start = i as i32 * STRIDE;
            (start, start + 4)
        })
        .collect()
}

fn broad_middle_rhs() -> Vec<Bounds> {
    vec![(16 * STRIDE, 48 * STRIDE)]
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
