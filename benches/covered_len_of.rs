use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use int_interval::I32CO;
use int_interval_set::I32COSet;

type Bounds = (i32, i32);

const N: usize = 64;

const QUERIES: &[(&str, Bounds)] = &[
    ("disjoint_before", (-32, -16)),
    ("adjacent_before_first", (-8, 0)),
    ("contained_single", (258, 262)),
    ("span_single_and_gap", (258, 274)),
    ("span_many_middle", (250, 582)),
    ("cover_all", (-16, 1032)),
];

/// Canonical source set:
///
/// ```text
/// [0, 8), [16, 24), ..., [1008, 1016)
/// ```
fn bounds() -> Vec<Bounds> {
    (0..N)
        .map(|i| {
            let start = i as i32 * 16;
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

fn bench_case(c: &mut Criterion, case: &str, query: Bounds, set: &I32COSet) {
    let mut group = c.benchmark_group("covered_len_of");
    let query = I32CO::try_new(query.0, query.1).unwrap();

    group.bench_function(BenchmarkId::new("int_interval_set", case), |b| {
        b.iter(|| black_box(black_box(set).covered_len_of(black_box(query))));
    });

    group.finish();
}

fn bench_covered_len_of(c: &mut Criterion) {
    let set = int_interval_set(&bounds());

    for &(case, query) in QUERIES {
        bench_case(c, case, query, &set);
    }
}

mod support;

criterion_group! {
    name = benches;
    config = support::config();
    targets = bench_covered_len_of
}

criterion_main!(benches);
