use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use int_interval::I32CO;
use int_interval_set::I32COSet;
use rangemap::RangeSet;

type Bounds = (i32, i32);

const INTERVAL_COUNT: i32 = 64;

const CASES: &[(&str, Bounds)] = &[
    ("disjoint_left", (-32, -16)),
    ("contained_middle", (514, 522)),
    ("crosses_gap_middle", (520, 530)),
    ("covers_middle_16", (384, 636)),
    ("covers_all", (-16, 1032)),
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
fn rangemap_set(bounds: &[Bounds]) -> RangeSet<i32> {
    let mut set = RangeSet::new();

    for &(start, end_excl) in bounds {
        set.insert(start..end_excl);
    }

    set
}

fn bench_intervals_intersecting(c: &mut Criterion) {
    let bounds = source_bounds();

    let int_set = int_interval_set(&bounds);
    let rangemap_set = rangemap_set(&bounds);

    for &(case, (start, end_excl)) in CASES {
        let mut group = c.benchmark_group("intervals_intersecting");

        let int_query = I32CO::try_new(start, end_excl).unwrap();
        group.bench_function(BenchmarkId::new("int_interval_set", case), |b| {
            b.iter(|| {
                let count = black_box(&int_set)
                    .intervals_intersecting(black_box(int_query))
                    .fold(0usize, |count, interval| {
                        black_box(interval);
                        count + 1
                    });

                black_box(count)
            });
        });

        let rangemap_query = start..end_excl;
        group.bench_function(BenchmarkId::new("rangemap", case), |b| {
            b.iter(|| {
                let count = black_box(&rangemap_set)
                    .overlapping(black_box(&rangemap_query))
                    .fold(0usize, |count, interval| {
                        black_box(interval);
                        count + 1
                    });

                black_box(count)
            });
        });

        group.finish();
    }
}

mod support;

criterion_group! {
    name = benches;
    config = support::config();
    targets = bench_intervals_intersecting
}

criterion_main!(benches);
