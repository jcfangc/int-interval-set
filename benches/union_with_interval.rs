use criterion::{Criterion, black_box, criterion_group, criterion_main};
use int_interval::I32CO;
use int_interval_set::I32COSet;
use range_collections::{RangeSet, RangeSet2};
use range_set_blaze::RangeSetBlaze;

type Bounds = (i32, i32);

const N: usize = 64;

const QUERIES: &[(&str, Bounds)] = &[
    ("disjoint_before", (-16, -8)),
    ("adjacent_before_first", (-8, 0)),
    ("contained_middle", (514, 518)),
    ("bridge_middle_gap", (504, 512)),
    ("bridge_many_middle", (498, 566)),
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

fn bench_case(
    c: &mut Criterion,
    case: &str,
    query: Bounds,
    int_set: &I32COSet,
    blaze_set: &RangeSetBlaze<i32>,
    collections_set: &RangeSet2<i32>,
) {
    let mut group = c.benchmark_group(format!("union_with_interval/{case}"));
    let (start, end_excl) = query;

    let int_query = I32CO::try_new(start, end_excl).unwrap();

    let blaze_query = RangeSetBlaze::from_iter([start..=(end_excl - 1)]);

    let collections_query: RangeSet2<i32> = RangeSet::from(start..end_excl);

    group.bench_function("int_interval_set", |b| {
        b.iter(|| black_box(black_box(int_set).union_with_interval(black_box(int_query))));
    });

    group.bench_function("range_set_blaze", |b| {
        b.iter(|| black_box(black_box(blaze_set) | black_box(&blaze_query)));
    });

    group.bench_function("range_collections", |b| {
        b.iter(|| {
            black_box(black_box(collections_set).union::<[i32; 2]>(black_box(&collections_query)))
        });
    });

    group.finish();
}

fn bench_union_with_interval(c: &mut Criterion) {
    let bounds = bounds();

    let int_set = int_interval_set(&bounds);
    let blaze_set = range_set_blaze(&bounds);
    let collections_set = range_collections(&bounds);

    for &(case, query) in QUERIES {
        bench_case(c, case, query, &int_set, &blaze_set, &collections_set);
    }
}

mod support;

criterion_group! {
    name = benches;
    config = support::config();
    targets = bench_union_with_interval
}

criterion_main!(benches);
