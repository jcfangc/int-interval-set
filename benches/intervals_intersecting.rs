use divan::{Bencher, black_box};
use int_interval::I32CO;
use int_interval_set::I32COSet;
use rangemap::RangeSet;

fn main() {
    divan::main();
}

type Bounds = (i32, i32);

const INTERVAL_COUNT: i32 = 64;

#[divan::bench(name = "intervals_intersecting/disjoint_left/int_interval_set")]
fn intervals_intersecting_disjoint_left_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (-32, -16));
}

#[divan::bench(name = "intervals_intersecting/disjoint_left/rangemap")]
fn intervals_intersecting_disjoint_left_rangemap(bencher: Bencher) {
    bench_rangemap(bencher, (-32, -16));
}

#[divan::bench(name = "intervals_intersecting/contained_middle/int_interval_set")]
fn intervals_intersecting_contained_middle_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (514, 522));
}

#[divan::bench(name = "intervals_intersecting/contained_middle/rangemap")]
fn intervals_intersecting_contained_middle_rangemap(bencher: Bencher) {
    bench_rangemap(bencher, (514, 522));
}

#[divan::bench(name = "intervals_intersecting/crosses_gap_middle/int_interval_set")]
fn intervals_intersecting_crosses_gap_middle_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (520, 530));
}

#[divan::bench(name = "intervals_intersecting/crosses_gap_middle/rangemap")]
fn intervals_intersecting_crosses_gap_middle_rangemap(bencher: Bencher) {
    bench_rangemap(bencher, (520, 530));
}

#[divan::bench(name = "intervals_intersecting/covers_middle_16/int_interval_set")]
fn intervals_intersecting_covers_middle_16_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (384, 636));
}

#[divan::bench(name = "intervals_intersecting/covers_middle_16/rangemap")]
fn intervals_intersecting_covers_middle_16_rangemap(bencher: Bencher) {
    bench_rangemap(bencher, (384, 636));
}

#[divan::bench(name = "intervals_intersecting/covers_all/int_interval_set")]
fn intervals_intersecting_covers_all_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (-16, 1032));
}

#[divan::bench(name = "intervals_intersecting/covers_all/rangemap")]
fn intervals_intersecting_covers_all_rangemap(bencher: Bencher) {
    bench_rangemap(bencher, (-16, 1032));
}

fn bench_int_interval_set(bencher: Bencher, query: Bounds) {
    let bounds = source_bounds();
    let set = int_interval_set(&bounds);
    let query = I32CO::try_new(query.0, query.1).unwrap();

    bencher.bench(|| {
        let count = black_box(&set)
            .intervals_intersecting(black_box(query))
            .fold(0usize, |count, interval| {
                black_box(interval);
                count + 1
            });

        black_box(count)
    });
}

fn bench_rangemap(bencher: Bencher, query: Bounds) {
    let bounds = source_bounds();
    let set = rangemap_set(&bounds);
    let query = query.0..query.1;

    bencher.bench(|| {
        let count =
            black_box(&set)
                .overlapping(black_box(&query))
                .fold(0usize, |count, interval| {
                    black_box(interval);
                    count + 1
                });

        black_box(count)
    });
}

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
