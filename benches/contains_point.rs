use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use int_interval::I32CO;
use int_interval_set::I32COSet;
use range_collections::RangeSet2;
use range_set_blaze::RangeSetBlaze;

type Bounds = (i32, i32);

const N: usize = 64;

const CASES: &[(&str, i32)] = &[
    ("hit_first", 1),
    ("gap_first", 2),
    ("hit_middle", 129),
    ("gap_middle", 130),
    ("hit_last", 253),
    ("gap_last", 254),
    ("before_all", -1),
    ("after_all", 256),
];

/// Produces 64 non-adjacent intervals: `[0, 2), [4, 6), ..., [252, 254)`.
fn bounds() -> Vec<Bounds> {
    (0..N)
        .map(|i| {
            let start = i as i32 * 4;
            (start, start + 2)
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

fn bench_contains_point(c: &mut Criterion) {
    let bounds = bounds();

    let int_interval_set = build_int_interval_set(&bounds);
    let range_set_blaze = build_range_set_blaze(&bounds);
    let range_collections = build_range_collections(&bounds);

    for &(case, point) in CASES {
        let mut group = c.benchmark_group("contains_point");

        group.bench_function(BenchmarkId::new("int_interval_set", case), |b| {
            b.iter(|| black_box(&int_interval_set).contains_point(black_box(point)))
        });

        group.bench_function(BenchmarkId::new("range_set_blaze", case), |b| {
            b.iter(|| black_box(&range_set_blaze).contains(black_box(point)))
        });

        group.bench_function(BenchmarkId::new("range_collections", case), |b| {
            b.iter(|| black_box(&range_collections).contains(black_box(&point)))
        });

        group.finish();
    }
}

mod support;

criterion_group! {
    name = benches;
    config = support::config();
    targets = bench_contains_point
}

criterion_main!(benches);
