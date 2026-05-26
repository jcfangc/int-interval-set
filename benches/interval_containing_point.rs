use criterion::{Criterion, black_box, criterion_group, criterion_main};
use int_interval::I32CO;
use int_interval_set::I32COSet;

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

/// 64 个互不相邻区间：[0, 2), [4, 6), ..., [252, 254)。
fn bounds() -> Vec<Bounds> {
    (0..N)
        .map(|i| {
            let start = i as i32 * 4;
            (start, start + 2)
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

fn bench_interval_containing_point(c: &mut Criterion) {
    let set = build_set(&bounds());

    for &(case, point) in CASES {
        let mut group = c.benchmark_group(format!("interval_containing_point/{case}"));

        group.bench_function("int_interval_set", |b| {
            b.iter(|| black_box(&set).interval_containing_point(black_box(point)))
        });

        group.finish();
    }
}

mod support;

criterion_group! {
    name = benches;
    config = support::config();
    targets = bench_interval_containing_point
}

criterion_main!(benches);
