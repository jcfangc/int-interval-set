use divan::{Bencher, black_box};
use int_interval::I32CO;
use int_interval_set::I32COSet;

fn main() {
    divan::main();
}

type Bounds = (i32, i32);

const N: usize = 64;

#[divan::bench(name = "covered_len_of/disjoint_before/int_interval_set")]
fn covered_len_of_disjoint_before_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (-32, -16));
}

#[divan::bench(name = "covered_len_of/adjacent_before_first/int_interval_set")]
fn covered_len_of_adjacent_before_first_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (-8, 0));
}

#[divan::bench(name = "covered_len_of/contained_single/int_interval_set")]
fn covered_len_of_contained_single_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (258, 262));
}

#[divan::bench(name = "covered_len_of/span_single_and_gap/int_interval_set")]
fn covered_len_of_span_single_and_gap_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (258, 274));
}

#[divan::bench(name = "covered_len_of/span_many_middle/int_interval_set")]
fn covered_len_of_span_many_middle_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (250, 582));
}

#[divan::bench(name = "covered_len_of/cover_all/int_interval_set")]
fn covered_len_of_cover_all_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, (-16, 1032));
}

fn bounds() -> Vec<Bounds> {
    (0..N)
        .map(|i| {
            let start = i as i32 * 16;
            (start, start + 8)
        })
        .collect()
}

fn bench_int_interval_set(bencher: Bencher, query: Bounds) {
    let set = int_interval_set(&bounds());
    let query = I32CO::try_new(query.0, query.1).unwrap();

    bencher.bench(|| black_box(black_box(&set).covered_len_of(black_box(query))));
}

#[inline]
fn int_interval_set(bounds: &[Bounds]) -> I32COSet {
    bounds
        .iter()
        .map(|&(start, end_excl)| I32CO::try_new(start, end_excl).unwrap())
        .collect()
}
