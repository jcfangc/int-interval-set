use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use int_interval::I32CO;
use int_interval_set::I32COSet;
use range_collections::RangeSet2;
use range_set_blaze::RangeSetBlaze;

type Bounds = (i32, i32);

const N: usize = 64;
const STRIDE: i32 = 8;

/// Produces the left-hand set: `[0, 4), [8, 12), ...`, 64 intervals total.
fn lhs_bounds() -> Vec<Bounds> {
    (0..N)
        .map(|i| {
            let start = i as i32 * STRIDE;
            (start, start + 4)
        })
        .collect()
}

/// Produces intervals that fill the gaps between left-hand intervals.
fn disjoint_rhs() -> Vec<Bounds> {
    (0..N)
        .map(|i| {
            let start = i as i32 * STRIDE + 4;
            (start, start + 4)
        })
        .collect()
}

/// Produces a right-hand set equal to the left-hand set.
fn equal_rhs() -> Vec<Bounds> {
    lhs_bounds()
}

/// Produces intervals that overlap half of each left-hand interval.
fn partial_overlap_rhs() -> Vec<Bounds> {
    (0..N)
        .map(|i| {
            let start = i as i32 * STRIDE + 2;
            (start, start + 4)
        })
        .collect()
}

/// Produces intervals that cover only even-indexed left-hand intervals.
fn alternating_rhs() -> Vec<Bounds> {
    (0..N)
        .step_by(2)
        .map(|i| {
            let start = i as i32 * STRIDE;
            (start, start + 4)
        })
        .collect()
}

/// Produces one continuous interval covering the middle portion.
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

fn bench_case(c: &mut Criterion, case: &str, lhs: &[Bounds], rhs: &[Bounds]) {
    let int_interval_lhs = build_int_interval_set(lhs);
    let int_interval_rhs = build_int_interval_set(rhs);

    let blaze_lhs = build_range_set_blaze(lhs);
    let blaze_rhs = build_range_set_blaze(rhs);

    let collections_lhs = build_range_collections(lhs);
    let collections_rhs = build_range_collections(rhs);

    let mut group = c.benchmark_group("symmetric_difference_with_set");
    group.throughput(Throughput::Elements((lhs.len() + rhs.len()) as u64));

    group.bench_function(BenchmarkId::new("int_interval_set", case), |b| {
        b.iter(|| {
            black_box(&int_interval_lhs).symmetric_difference_with_set(black_box(&int_interval_rhs))
        })
    });

    group.bench_function(BenchmarkId::new("range_set_blaze", case), |b| {
        b.iter(|| black_box(&blaze_lhs) ^ black_box(&blaze_rhs))
    });

    group.bench_function(BenchmarkId::new("range_collections", case), |b| {
        b.iter(|| black_box(&collections_lhs) ^ black_box(&collections_rhs))
    });

    group.finish();
}

fn bench_symmetric_difference_with_set(c: &mut Criterion) {
    let lhs = lhs_bounds();

    bench_case(c, "disjoint_64x64", &lhs, &disjoint_rhs());
    bench_case(c, "equal_64x64", &lhs, &equal_rhs());
    bench_case(c, "partial_overlap_64x64", &lhs, &partial_overlap_rhs());
    bench_case(c, "alternating_64x32", &lhs, &alternating_rhs());
    bench_case(c, "broad_middle_64x1", &lhs, &broad_middle_rhs());
}

mod support;

criterion_group! {
    name = benches;
    config = support::config();
    targets = bench_symmetric_difference_with_set
}

criterion_main!(benches);
