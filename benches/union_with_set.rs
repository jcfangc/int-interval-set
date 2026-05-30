use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use int_interval::I32CO;
use int_interval_set::I32COSet;
use range_collections::RangeSet2;
use range_set_blaze::RangeSetBlaze;

type Bounds = (i32, i32);

const N: i32 = 64;

/// ```text
/// [0, 8), [16, 24), ..., [1008, 1016)
/// ```
/// Produces the left-hand set: 64 intervals of length 8 separated by gaps of length 8.
///
/// Layout: `[0, 8), [16, 24), ..., [1008, 1016)`.
fn lhs_bounds() -> Vec<Bounds> {
    (0..N)
        .map(|i| {
            let start = i * 16;
            (start, start + 8)
        })
        .collect()
}

/// Produces a right-hand set equal to the left-hand set.
fn equal_rhs() -> Vec<Bounds> {
    lhs_bounds()
}

/// Produces intervals fully contained inside the left-hand intervals.
fn contained_rhs() -> Vec<Bounds> {
    (0..N)
        .map(|i| {
            let start = i * 16 + 2;
            (start, start + 4)
        })
        .collect()
}

/// Produces intervals that partially overlap the right side of each left-hand interval.
fn overlapping_rhs() -> Vec<Bounds> {
    (0..N)
        .map(|i| {
            let start = i * 16 + 4;
            (start, start + 8)
        })
        .collect()
}

/// Produces intervals in the gaps between left-hand intervals without adjacency.
fn interleaved_disjoint_rhs() -> Vec<Bounds> {
    (0..N)
        .map(|i| {
            let start = i * 16 + 10;
            (start, start + 4)
        })
        .collect()
}

/// Produces intervals that bridge all gaps and canonicalize the union into one interval.
fn adjacent_bridge_rhs() -> Vec<Bounds> {
    (0..N)
        .map(|i| {
            let start = i * 16 + 8;
            (start, start + 8)
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
    let (&(start, end_excl), rest) = bounds.split_first().unwrap();
    let mut set = RangeSet2::from(start..end_excl);

    for &(start, end_excl) in rest {
        set |= RangeSet2::from(start..end_excl);
    }

    set
}

fn bench_case(c: &mut Criterion, case: &str, lhs: &[Bounds], rhs: &[Bounds]) {
    let mut group = c.benchmark_group("union_with_set");
    group.throughput(Throughput::Elements((lhs.len() + rhs.len()) as u64));

    let lhs_int = int_interval_set(lhs);
    let rhs_int = int_interval_set(rhs);
    group.bench_function(BenchmarkId::new("int_interval_set", case), |b| {
        b.iter(|| black_box(black_box(&lhs_int).union_with_set(black_box(&rhs_int))));
    });

    let lhs_blaze = range_set_blaze(lhs);
    let rhs_blaze = range_set_blaze(rhs);
    group.bench_function(BenchmarkId::new("range_set_blaze", case), |b| {
        b.iter(|| black_box(black_box(&lhs_blaze) | black_box(&rhs_blaze)));
    });

    let lhs_collections = range_collections(lhs);
    let rhs_collections = range_collections(rhs);
    group.bench_function(BenchmarkId::new("range_collections", case), |b| {
        b.iter(|| black_box(black_box(&lhs_collections) | black_box(&rhs_collections)));
    });

    group.finish();
}

fn bench_union_with_set(c: &mut Criterion) {
    let lhs = lhs_bounds();

    bench_case(c, "equal_64", &lhs, &equal_rhs());
    bench_case(c, "contained_64", &lhs, &contained_rhs());
    bench_case(c, "overlapping_64", &lhs, &overlapping_rhs());
    bench_case(
        c,
        "interleaved_disjoint_64",
        &lhs,
        &interleaved_disjoint_rhs(),
    );
    bench_case(c, "adjacent_bridge_64", &lhs, &adjacent_bridge_rhs());
}

mod support;

criterion_group! {
    name = benches;
    config = support::config();
    targets = bench_union_with_set
}

criterion_main!(benches);
