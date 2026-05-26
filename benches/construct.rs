use criterion::{Criterion, black_box, criterion_group, criterion_main};
use int_interval::I32CO;
use int_interval_set::I32COSet;
use range_set_blaze::RangeSetBlaze;

type Bounds = (i32, i32);

const N: usize = 64;

/// 已规范化输入：排序、互不重叠、互不相邻。
fn sorted_disjoint() -> Vec<Bounds> {
    (0..N)
        .map(|i| {
            let start = i as i32 * 4;
            (start, start + 2)
        })
        .collect()
}

/// 与 `sorted_disjoint` 结果相同，但输入顺序完全反转。
fn reversed_disjoint() -> Vec<Bounds> {
    let mut ranges = sorted_disjoint();
    ranges.reverse();
    ranges
}

/// 所有区间首尾相接，最终规范化为单一区间。
fn adjacent_chain() -> Vec<Bounds> {
    (0..N)
        .map(|i| {
            let start = i as i32 * 2;
            (start, start + 2)
        })
        .collect()
}

/// 同时包含乱序、相邻与重叠；每四个输入区间合并为一个结果区间。
fn mixed_unsorted() -> Vec<Bounds> {
    let mut ranges = Vec::with_capacity(N);

    for i in 0..(N / 4) {
        let base = i as i32 * 40;

        ranges.extend([
            (base + 8, base + 18),
            (base, base + 10),
            (base + 24, base + 30),
            (base + 18, base + 24),
        ]);
    }

    ranges.reverse();
    ranges
}

#[inline]
fn construct_int_interval_set(bounds: &[Bounds]) -> I32COSet {
    bounds
        .iter()
        .map(|&(start, end_excl)| I32CO::try_new(start, end_excl).unwrap())
        .collect()
}

#[inline]
fn construct_range_set_blaze(bounds: &[Bounds]) -> RangeSetBlaze<i32> {
    bounds
        .iter()
        .map(|&(start, end_excl)| start..=(end_excl - 1))
        .collect()
}

fn bench_case(c: &mut Criterion, case: &str, bounds: &[Bounds]) {
    let mut group = c.benchmark_group(format!("construct/{case}"));

    group.bench_function("int_interval_set", |b| {
        b.iter(|| {
            black_box(construct_int_interval_set(black_box(bounds)));
        });
    });

    group.bench_function("range_set_blaze", |b| {
        b.iter(|| {
            black_box(construct_range_set_blaze(black_box(bounds)));
        });
    });

    group.finish();
}

fn bench_construct(c: &mut Criterion) {
    let sorted_disjoint = sorted_disjoint();
    let reversed_disjoint = reversed_disjoint();
    let adjacent_chain = adjacent_chain();
    let mixed_unsorted = mixed_unsorted();

    bench_case(c, "sorted_disjoint_64", &sorted_disjoint);
    bench_case(c, "reversed_disjoint_64", &reversed_disjoint);
    bench_case(c, "adjacent_chain_64", &adjacent_chain);
    bench_case(c, "mixed_unsorted_64", &mixed_unsorted);
}

mod support;

criterion_group! {
    name = benches;
    config = support::config();
    targets = bench_construct
}

criterion_main!(benches);
