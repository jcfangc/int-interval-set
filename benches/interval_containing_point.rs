use divan::{Bencher, black_box};
use int_interval::I32CO;
use int_interval_set::I32COSet;

fn main() {
    divan::main();
}

type Bounds = (i32, i32);

const N: usize = 64;

#[divan::bench(name = "interval_containing_point/hit_first/int_interval_set")]
fn interval_containing_point_hit_first_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, 1);
}

#[divan::bench(name = "interval_containing_point/gap_first/int_interval_set")]
fn interval_containing_point_gap_first_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, 2);
}

#[divan::bench(name = "interval_containing_point/hit_middle/int_interval_set")]
fn interval_containing_point_hit_middle_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, 129);
}

#[divan::bench(name = "interval_containing_point/gap_middle/int_interval_set")]
fn interval_containing_point_gap_middle_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, 130);
}

#[divan::bench(name = "interval_containing_point/hit_last/int_interval_set")]
fn interval_containing_point_hit_last_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, 253);
}

#[divan::bench(name = "interval_containing_point/gap_last/int_interval_set")]
fn interval_containing_point_gap_last_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, 254);
}

#[divan::bench(name = "interval_containing_point/before_all/int_interval_set")]
fn interval_containing_point_before_all_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, -1);
}

#[divan::bench(name = "interval_containing_point/after_all/int_interval_set")]
fn interval_containing_point_after_all_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, 256);
}

fn bounds() -> Vec<Bounds> {
    (0..N)
        .map(|i| {
            let start = i as i32 * 4;
            (start, start + 2)
        })
        .collect()
}

fn bench_int_interval_set(bencher: Bencher, point: i32) {
    let set = build_set(&bounds());

    bencher.bench(|| black_box(&set).interval_containing_point(black_box(point)));
}

#[inline]
fn build_set(bounds: &[Bounds]) -> I32COSet {
    bounds
        .iter()
        .map(|&(start, end_excl)| I32CO::try_new(start, end_excl).unwrap())
        .collect()
}
