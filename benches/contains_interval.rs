use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use int_interval::I32CO;
use int_interval_set::I32COSet;
use range_collections::RangeSet2;
use range_set_blaze::RangeSetBlaze;

type Bounds = (i32, i32);

const INTERVAL_COUNT: i32 = 64;

const CASES: &[(&str, Bounds)] = &[
    ("contained_first", (2, 10)),
    ("contained_middle", (514, 522)),
    ("contained_last", (1010, 1018)),
    ("crosses_gap", (520, 530)),
    ("inside_gap", (524, 528)),
    ("outside_right", (1020, 1028)),
];

/// ```text
/// [0, 12), [16, 28), ..., [1008, 1020)
/// ```
/// Produces 64 canonical intervals of length 12 separated by gaps of length 4.
///
/// Layout: `[0, 12), [16, 28), ..., [1008, 1020)`.
fn source_bounds() -> Vec<Bounds> {
    (0..INTERVAL_COUNT)
        .map(|i| {
            let start = i * 16;
            (start, start + 12)
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

fn bench_contains_interval(c: &mut Criterion) {
    let bounds = source_bounds();

    let int_set = int_interval_set(&bounds);
    let blaze_set = range_set_blaze(&bounds);
    let collections_set = range_collections(&bounds);

    for &(case, (start, end_excl)) in CASES {
        let mut group = c.benchmark_group("contains_interval");

        let int_query = I32CO::try_new(start, end_excl).unwrap();
        group.bench_function(BenchmarkId::new("int_interval_set", case), |b| {
            b.iter(|| black_box(&int_set).contains_interval(black_box(int_query)));
        });

        let blaze_query = RangeSetBlaze::from(start..=(end_excl - 1));
        group.bench_function(BenchmarkId::new("range_set_blaze", case), |b| {
            b.iter(|| black_box(&blaze_set).is_superset(black_box(&blaze_query)));
        });

        let collections_query = RangeSet2::from(start..end_excl);
        group.bench_function(BenchmarkId::new("range_collections", case), |b| {
            b.iter(|| {
                black_box(&collections_set).is_superset(black_box(collections_query.as_ref()))
            });
        });

        group.finish();
    }
}

mod support;

criterion_group! {
    name = benches;
    config = support::config();
    targets = bench_contains_interval
}

criterion_main!(benches);
