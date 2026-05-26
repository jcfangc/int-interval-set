use criterion::{Criterion, black_box, criterion_group, criterion_main};
use int_interval::I32CO;
use int_interval_set::I32COSet;
use range_collections::range_set::{RangeSet, RangeSet2};
use range_set_blaze::RangeSetBlaze;

type Bounds = (i32, i32);

const INTERVALS: usize = 64;

const QUERIES: &[(&str, Bounds)] = &[
    ("before_all", (-16, -8)),
    ("hit_first", (1, 3)),
    ("adjacent_left_middle", (252, 256)),
    ("gap_middle", (260, 264)),
    ("hit_middle", (257, 259)),
    ("span_middle_gap", (258, 266)),
    ("adjacent_right_last", (508, 512)),
    ("after_all", (520, 528)),
];

/// Produces 64 canonical intervals:
///
/// ```text
/// [0, 4), [8, 12), ..., [504, 508)
/// ```
fn bounds() -> Vec<Bounds> {
    (0..INTERVALS)
        .map(|i| {
            let start = i as i32 * 8;
            (start, start + 4)
        })
        .collect()
}

fn int_interval_set(bounds: &[Bounds]) -> I32COSet {
    bounds
        .iter()
        .map(|&(start, end_excl)| I32CO::try_new(start, end_excl).unwrap())
        .collect()
}

fn range_set_blaze(bounds: &[Bounds]) -> RangeSetBlaze<i32> {
    bounds
        .iter()
        .map(|&(start, end_excl)| start..=(end_excl - 1))
        .collect()
}

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
    int_interval_set: &I32COSet,
    range_set_blaze: &RangeSetBlaze<i32>,
    range_collections: &RangeSet2<i32>,
) {
    let mut group = c.benchmark_group(format!("intersects_interval/{case}"));
    let (start, end_excl) = query;

    let int_interval_query = I32CO::try_new(start, end_excl).unwrap();

    let range_set_blaze_query = RangeSetBlaze::from_iter([start..=(end_excl - 1)]);

    let range_collections_query: RangeSet2<i32> = RangeSet::from(start..end_excl);

    group.bench_function("int_interval_set", |b| {
        b.iter(|| black_box(int_interval_set).intersects_interval(black_box(int_interval_query)));
    });

    group.bench_function("range_set_blaze", |b| {
        b.iter(|| !black_box(range_set_blaze).is_disjoint(black_box(&range_set_blaze_query)));
    });

    group.bench_function("range_collections", |b| {
        b.iter(|| black_box(range_collections).intersects(black_box(&range_collections_query)));
    });

    group.finish();
}

fn bench_intersects_interval(c: &mut Criterion) {
    let bounds = bounds();

    let int_interval_set = int_interval_set(&bounds);
    let range_set_blaze = range_set_blaze(&bounds);
    let range_collections = range_collections(&bounds);

    for &(case, query) in QUERIES {
        bench_case(
            c,
            case,
            query,
            &int_interval_set,
            &range_set_blaze,
            &range_collections,
        );
    }
}

mod support;

criterion_group! {
    name = benches;
    config = support::config();
    targets = bench_intersects_interval
}

criterion_main!(benches);
