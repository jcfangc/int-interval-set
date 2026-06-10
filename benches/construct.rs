use divan::{Bencher, black_box};
use int_interval::I32CO;
use int_interval_set::I32COSet;
use range_set_blaze::RangeSetBlaze;

fn main() {
    divan::main();
}

type Bounds = (i32, i32);

const N: usize = 64;

#[divan::bench(name = "construct/sorted_disjoint_64/int_interval_set")]
fn construct_sorted_disjoint_64_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, &sorted_disjoint());
}

#[divan::bench(name = "construct/sorted_disjoint_64/range_set_blaze")]
fn construct_sorted_disjoint_64_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, &sorted_disjoint());
}

#[divan::bench(name = "construct/reversed_disjoint_64/int_interval_set")]
fn construct_reversed_disjoint_64_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, &reversed_disjoint());
}

#[divan::bench(name = "construct/reversed_disjoint_64/range_set_blaze")]
fn construct_reversed_disjoint_64_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, &reversed_disjoint());
}

#[divan::bench(name = "construct/adjacent_chain_64/int_interval_set")]
fn construct_adjacent_chain_64_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, &adjacent_chain());
}

#[divan::bench(name = "construct/adjacent_chain_64/range_set_blaze")]
fn construct_adjacent_chain_64_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, &adjacent_chain());
}

#[divan::bench(name = "construct/mixed_unsorted_64/int_interval_set")]
fn construct_mixed_unsorted_64_int_interval_set(bencher: Bencher) {
    bench_int_interval_set(bencher, &mixed_unsorted());
}

#[divan::bench(name = "construct/mixed_unsorted_64/range_set_blaze")]
fn construct_mixed_unsorted_64_range_set_blaze(bencher: Bencher) {
    bench_range_set_blaze(bencher, &mixed_unsorted());
}

fn sorted_disjoint() -> Vec<Bounds> {
    (0..N)
        .map(|i| {
            let start = i as i32 * 4;
            (start, start + 2)
        })
        .collect()
}

fn reversed_disjoint() -> Vec<Bounds> {
    let mut ranges = sorted_disjoint();
    ranges.reverse();
    ranges
}

fn adjacent_chain() -> Vec<Bounds> {
    (0..N)
        .map(|i| {
            let start = i as i32 * 2;
            (start, start + 2)
        })
        .collect()
}

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

fn bench_int_interval_set(bencher: Bencher, bounds: &[Bounds]) {
    bencher.bench(|| {
        black_box(construct_int_interval_set(black_box(bounds)));
    });
}

fn bench_range_set_blaze(bencher: Bencher, bounds: &[Bounds]) {
    bencher.bench(|| {
        black_box(construct_range_set_blaze(black_box(bounds)));
    });
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
