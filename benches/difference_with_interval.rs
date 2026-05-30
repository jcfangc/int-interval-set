use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use int_interval::I32CO;
use int_interval_set::I32COSet;
use range_collections::RangeSet2;
use range_set_blaze::RangeSetBlaze;

type Bounds = (i32, i32);

const N: usize = 64;
const STRIDE: i32 = 8;
const WIDTH: i32 = 4;

const CASES: &[(&str, Bounds)] = &[
    ("disjoint_before", (-16, -8)),
    ("disjoint_gap_middle", (260, 264)),
    ("remove_first_exact", (0, 4)),
    ("trim_middle_left", (256, 258)),
    ("split_middle", (257, 259)),
    ("remove_middle_span", (16 * STRIDE, 48 * STRIDE)),
    ("clip_middle_span", (16 * STRIDE + 2, 48 * STRIDE + 2)),
];

/// Produces `[0, 4), [8, 12), ..., [504, 508)`.
fn set_bounds() -> Vec<Bounds> {
    (0..N)
        .map(|i| {
            let start = i as i32 * STRIDE;
            (start, start + WIDTH)
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

fn bench_difference_with_interval(c: &mut Criterion) {
    let bounds = set_bounds();

    let int_interval_set = build_int_interval_set(&bounds);
    let range_set_blaze = build_range_set_blaze(&bounds);
    let range_collections = build_range_collections(&bounds);

    for &(case, (start, end_excl)) in CASES {
        let query = I32CO::try_new(start, end_excl).unwrap();

        // Comparator libraries do not expose a dedicated half-open interval
        // difference entry point, so the query is prebuilt as a singleton set.
        let blaze_query = RangeSetBlaze::from(start..=(end_excl - 1));
        let collections_query = RangeSet2::from(start..end_excl);

        let mut group = c.benchmark_group("difference_with_interval");
        group.throughput(Throughput::Elements(bounds.len() as u64));

        group.bench_function(BenchmarkId::new("int_interval_set", case), |b| {
            b.iter(|| black_box(&int_interval_set).difference_with_interval(black_box(query)))
        });

        group.bench_function(BenchmarkId::new("range_set_blaze", case), |b| {
            b.iter(|| black_box(&range_set_blaze) - black_box(&blaze_query))
        });

        group.bench_function(BenchmarkId::new("range_collections", case), |b| {
            b.iter(|| black_box(&range_collections) - black_box(&collections_query))
        });

        group.finish();
    }
}

mod support;

criterion_group! {
    name = benches;
    config = support::config();
    targets = bench_difference_with_interval
}

criterion_main!(benches);
