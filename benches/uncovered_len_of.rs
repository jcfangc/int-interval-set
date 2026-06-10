use divan::{Bencher, black_box};
use int_interval::I32CO;
use int_interval_set::I32COSet;

fn main() {
    divan::main();
}

type Bounds = (i32, i32);

const N: usize = 64;
const STRIDE: i32 = 8;
const WIDTH: i32 = 4;

#[divan::bench(name = "uncovered_len_of/disjoint_before/int_interval_set")]
fn uncovered_len_of_disjoint_before_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (-16, -8));
}

#[divan::bench(name = "uncovered_len_of/contained_in_hit/int_interval_set")]
fn uncovered_len_of_contained_in_hit_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (1, 3));
}

#[divan::bench(name = "uncovered_len_of/single_gap/int_interval_set")]
fn uncovered_len_of_single_gap_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (4, 8));
}

#[divan::bench(name = "uncovered_len_of/partial_single/int_interval_set")]
fn uncovered_len_of_partial_single_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (2, 6));
}

#[divan::bench(name = "uncovered_len_of/span_two_hits/int_interval_set")]
fn uncovered_len_of_span_two_hits_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (0, 12));
}

#[divan::bench(name = "uncovered_len_of/span_middle_32/int_interval_set")]
fn uncovered_len_of_span_middle_32_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (16 * STRIDE, 47 * STRIDE + WIDTH));
}

#[divan::bench(name = "uncovered_len_of/full_span/int_interval_set")]
fn uncovered_len_of_full_span_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (0, 63 * STRIDE + WIDTH));
}

#[divan::bench(name = "uncovered_len_of/outer_padded_span/int_interval_set")]
fn uncovered_len_of_outer_padded_span_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (-8, 63 * STRIDE + WIDTH + 8));
}

fn bench_int_interval_set(bencher: Bencher, bounds: Bounds) {
    let set = build_set(&set_bounds());
    let query = I32CO::try_new(bounds.0, bounds.1).unwrap();

    bencher.bench(|| black_box(&set).uncovered_len_of(black_box(query)));
}

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
