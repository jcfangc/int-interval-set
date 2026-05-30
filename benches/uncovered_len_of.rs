use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use int_interval::I32CO;
use int_interval_set::I32COSet;

type Bounds = (i32, i32);

const N: usize = 64;
const STRIDE: i32 = 8;
const WIDTH: i32 = 4;

const CASES: &[(&str, Bounds)] = &[
    ("disjoint_before", (-16, -8)),
    ("contained_in_hit", (1, 3)),
    ("single_gap", (4, 8)),
    ("partial_single", (2, 6)),
    ("span_two_hits", (0, 12)),
    ("span_middle_32", (16 * STRIDE, 47 * STRIDE + WIDTH)),
    ("full_span", (0, 63 * STRIDE + WIDTH)),
    ("outer_padded_span", (-8, 63 * STRIDE + WIDTH + 8)),
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
fn build_set(bounds: &[Bounds]) -> I32COSet {
    bounds
        .iter()
        .map(|&(start, end_excl)| I32CO::try_new(start, end_excl).unwrap())
        .collect()
}

fn bench_uncovered_len_of(c: &mut Criterion) {
    let set = build_set(&set_bounds());

    for &(case, (start, end_excl)) in CASES {
        let query = I32CO::try_new(start, end_excl).unwrap();
        let mut group = c.benchmark_group("uncovered_len_of");

        group.bench_function(BenchmarkId::new("int_interval_set", case), |b| {
            b.iter(|| black_box(&set).uncovered_len_of(black_box(query)))
        });

        group.finish();
    }
}

mod support;

criterion_group! {
    name = benches;
    config = support::config();
    targets = bench_uncovered_len_of
}

criterion_main!(benches);
