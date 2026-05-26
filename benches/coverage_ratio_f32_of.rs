use criterion::{Criterion, black_box, criterion_group, criterion_main};
use int_interval::I32CO;
use int_interval_set::I32COSet;

type Bounds = (i32, i32);

const INTERVAL_COUNT: i32 = 64;

const CASES: &[(&str, Bounds)] = &[
    ("uncovered_gap", (524, 528)),
    ("fully_covered_middle", (514, 522)),
    ("crosses_gap_middle", (520, 530)),
    ("covers_middle_16", (384, 640)),
    ("covers_all_span", (0, 1020)),
    ("mostly_outside", (-512, 1536)),
];

/// 64 个规范化区间：每段长度 12，段间 gap 长度 4。
///
/// ```text
/// [0, 12), [16, 28), ..., [1008, 1020)
/// ```
fn source_set() -> I32COSet {
    (0..INTERVAL_COUNT)
        .map(|i| {
            let start = i * 16;
            I32CO::try_new(start, start + 12).unwrap()
        })
        .collect()
}

fn bench_coverage_ratio_f32_of(c: &mut Criterion) {
    let set = source_set();

    for &(case, (start, end_excl)) in CASES {
        let mut group = c.benchmark_group(format!("coverage_ratio_f32_of/{case}"));

        let query = I32CO::try_new(start, end_excl).unwrap();

        group.bench_function("int_interval_set", |b| {
            b.iter(|| black_box(black_box(&set).coverage_ratio_f32_of(black_box(query))));
        });

        group.finish();
    }
}

mod support;

criterion_group! {
    name = benches;
    config = support::config();
    targets = bench_coverage_ratio_f32_of
}

criterion_main!(benches);
