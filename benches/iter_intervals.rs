use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use int_interval::I32CO;
use int_interval_set::I32COSet;
use range_collections::{RangeSet, RangeSet2};
use range_set_blaze::RangeSetBlaze;

type Bounds = (i32, i32);

const N_SMALL: usize = 64;
const N_LARGE: usize = 1024;

/// Produces canonical disjoint intervals:
///
/// ```text
/// [0, 4), [8, 12), [16, 20), ...
/// ```
fn sparse_bounds(n: usize) -> Vec<Bounds> {
    (0..n)
        .map(|i| {
            let start = i as i32 * 8;
            (start, start + 4)
        })
        .collect()
}

/// Produces adjacent inputs that canonicalize into one interval.
fn adjacent_bounds(n: usize) -> Vec<Bounds> {
    (0..n)
        .map(|i| {
            let start = i as i32 * 4;
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

fn bench_case(c: &mut Criterion, case: &str, bounds: &[Bounds]) {
    let int_set = int_interval_set(bounds);
    let blaze_set = range_set_blaze(bounds);
    let collections_set = range_collections(bounds);

    let output_intervals = int_set.interval_count();

    let mut group = c.benchmark_group("iter_intervals");

    group.throughput(Throughput::Elements(output_intervals as u64));

    group.bench_function(BenchmarkId::new("int_interval_set", case), |b| {
        b.iter(|| {
            for interval in black_box(&int_set).iter_intervals() {
                black_box(interval);
            }
        });
    });

    group.bench_function(BenchmarkId::new("range_set_blaze", case), |b| {
        b.iter(|| {
            for interval in black_box(&blaze_set).ranges() {
                black_box(interval);
            }
        });
    });

    group.bench_function(BenchmarkId::new("range_collections", case), |b| {
        b.iter(|| {
            for interval in black_box(&collections_set).iter() {
                black_box(interval);
            }
        });
    });

    group.finish();
}

fn bench_iter_intervals(c: &mut Criterion) {
    let merged_64 = adjacent_bounds(N_SMALL);
    let sparse_64 = sparse_bounds(N_SMALL);
    let sparse_1024 = sparse_bounds(N_LARGE);

    bench_case(c, "merged_64_to_1", &merged_64);
    bench_case(c, "sparse_64", &sparse_64);
    bench_case(c, "sparse_1024", &sparse_1024);
}

mod support;

criterion_group! {
    name = benches;
    config = support::config();
    targets = bench_iter_intervals
}

criterion_main!(benches);
