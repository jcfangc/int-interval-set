use criterion::{Criterion, Throughput, black_box, criterion_group, criterion_main};
use int_interval::I32CO;
use int_interval_set::I32COSet;
use range_collections::{RangeSet, RangeSet2};
use range_set_blaze::RangeSetBlaze;

type Bounds = (i32, i32);

const N: usize = 64;

/// Canonical left-hand set:
///
/// ```text
/// [0, 8), [16, 24), ..., [1008, 1016)
/// ```
fn lhs_bounds() -> Vec<Bounds> {
    (0..N)
        .map(|i| {
            let start = i as i32 * 16;
            (start, start + 8)
        })
        .collect()
}

/// Right-hand intervals lie in the gaps between left-hand intervals.
/// The result is identical to `lhs`, but both sets must be scanned.
fn interleaved_disjoint() -> Vec<Bounds> {
    (0..N)
        .map(|i| {
            let start = i as i32 * 16 + 10;
            (start, start + 4)
        })
        .collect()
}

/// Removing an equal set produces an empty result.
fn equal() -> Vec<Bounds> {
    lhs_bounds()
}

/// Removes the right half of each left-hand interval:
///
/// ```text
/// [0, 8) - [4, 12) = [0, 4)
/// ```
fn trim_right() -> Vec<Bounds> {
    (0..N)
        .map(|i| {
            let start = i as i32 * 16 + 4;
            (start, start + 8)
        })
        .collect()
}

/// Punches a hole inside every left-hand interval:
///
/// ```text
/// [0, 8) - [2, 6) = [0, 2), [6, 8)
/// ```
///
/// This doubles the output interval count.
fn punch_middle() -> Vec<Bounds> {
    (0..N)
        .map(|i| {
            let start = i as i32 * 16 + 2;
            (start, start + 4)
        })
        .collect()
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

fn bench_case(c: &mut Criterion, case: &str, rhs_bounds: &[Bounds]) {
    let lhs_bounds = lhs_bounds();

    let int_lhs = int_interval_set(&lhs_bounds);
    let int_rhs = int_interval_set(rhs_bounds);

    let blaze_lhs = range_set_blaze(&lhs_bounds);
    let blaze_rhs = range_set_blaze(rhs_bounds);

    let collections_lhs = range_collections(&lhs_bounds);
    let collections_rhs = range_collections(rhs_bounds);

    let mut group = c.benchmark_group(format!("difference_with_set/{case}"));

    group.throughput(Throughput::Elements(
        (lhs_bounds.len() + rhs_bounds.len()) as u64,
    ));

    group.bench_function("int_interval_set", |b| {
        b.iter(|| black_box(black_box(&int_lhs).difference_with_set(black_box(&int_rhs))));
    });

    group.bench_function("range_set_blaze", |b| {
        b.iter(|| black_box(black_box(&blaze_lhs) - black_box(&blaze_rhs)));
    });

    group.bench_function("range_collections", |b| {
        b.iter(|| {
            black_box(
                black_box(&collections_lhs).difference::<[i32; 2]>(black_box(&collections_rhs)),
            )
        });
    });

    group.finish();
}

fn bench_difference_with_set(c: &mut Criterion) {
    let interleaved_disjoint = interleaved_disjoint();
    let equal = equal();
    let trim_right = trim_right();
    let punch_middle = punch_middle();

    bench_case(c, "interleaved_disjoint_64", &interleaved_disjoint);
    bench_case(c, "equal_64", &equal);
    bench_case(c, "trim_right_64", &trim_right);
    bench_case(c, "punch_middle_64", &punch_middle);
}

mod support;

criterion_group! {
    name = benches;
    config = support::config();
    targets = bench_difference_with_set
}

criterion_main!(benches);
