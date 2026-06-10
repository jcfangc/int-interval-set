use divan::{Bencher, black_box};
use int_interval::I32CO;
use int_interval_set::I32COSet;

fn main() {
    divan::main();
}

type Bounds = (i32, i32);

const INTERVAL_COUNT: i32 = 64;

#[divan::bench(name = "coverage_ratio_f32_of/uncovered_gap/int_interval_set")]
fn coverage_ratio_f32_of_uncovered_gap_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (524, 528));
}

#[divan::bench(name = "coverage_ratio_f32_of/fully_covered_middle/int_interval_set")]
fn coverage_ratio_f32_of_fully_covered_middle_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (514, 522));
}

#[divan::bench(name = "coverage_ratio_f32_of/crosses_gap_middle/int_interval_set")]
fn coverage_ratio_f32_of_crosses_gap_middle_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (520, 530));
}

#[divan::bench(name = "coverage_ratio_f32_of/covers_middle_16/int_interval_set")]
fn coverage_ratio_f32_of_covers_middle_16_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (384, 640));
}

#[divan::bench(name = "coverage_ratio_f32_of/covers_all_span/int_interval_set")]
fn coverage_ratio_f32_of_covers_all_span_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (0, 1020));
}

#[divan::bench(name = "coverage_ratio_f32_of/mostly_outside/int_interval_set")]
fn coverage_ratio_f32_of_mostly_outside_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (-512, 1536));
}

fn bench_int_interval_set(bencher: Bencher, bounds: Bounds) {
    let set = source_set();
    let query = I32CO::try_new(bounds.0, bounds.1).unwrap();

    bencher.bench(|| black_box(&set).coverage_ratio_f32_of(black_box(query)));
}

fn source_set() -> I32COSet {
    (0..INTERVAL_COUNT)
        .map(|i| {
            let start = i * 16;
            I32CO::try_new(start, start + 12).unwrap()
        })
        .collect()
}
