use int_interval::I8CO;

use crate::{
    I8COSet,
    int_co_set::test_support::{intervals, iv},
};

use super::*;

#[test]
fn from_par_iter_empty() {
    let set: I8COSet = Vec::<I8CO>::new().into_par_iter().collect();

    assert!(intervals(&set).is_empty());
}

#[test]
fn from_par_iter_sorts_and_merges_overlap_adjacency_and_duplicates() {
    let set: I8COSet = [
        iv(12, 14),
        iv(-20, -17),
        iv(-18, -14), // overlaps [-20, -17)
        iv(-12, -10),
        iv(-14, -12), // connects [-20, -14) with [-12, -10)
        iv(12, 14),   // duplicate
        iv(-10, -9),  // remains separated from [12, 14)
    ]
    .into_par_iter()
    .collect();

    assert_eq!(intervals(&set), &[iv(-20, -9), iv(12, 14)]);
}

#[test]
fn from_par_iter_merges_many_reduced_runs() {
    let mut input = Vec::with_capacity(BATCH_SIZE * 8 + 4);

    input.extend(std::iter::repeat_n(iv(-8, -6), BATCH_SIZE * 2));
    input.extend(std::iter::repeat_n(iv(-2, 0), BATCH_SIZE * 2));
    input.extend(std::iter::repeat_n(iv(-6, -4), BATCH_SIZE * 2));
    input.extend(std::iter::repeat_n(iv(-4, -2), BATCH_SIZE * 2));
    input.extend([iv(20, 22), iv(21, 24), iv(30, 31), iv(30, 31)]);

    let set: I8COSet = input.into_par_iter().collect();

    assert_eq!(intervals(&set), &[iv(-8, 0), iv(20, 24), iv(30, 31)]);
    assert!(is_canonical(intervals(&set)));
}

#[test]
fn from_par_iter_matches_whole_input_normalization() {
    let input = [
        iv(30, 35),
        iv(-20, -17),
        iv(-14, -10),
        iv(-18, -13),
        iv(50, 51),
        iv(49, 50),
        iv(20, 22),
        iv(21, 25),
        iv(30, 35),
    ];

    let expected = normalize(input.to_vec());
    let set: I8COSet = input.into_par_iter().collect();

    assert_eq!(intervals(&set), expected.as_slice());
    assert!(is_canonical(intervals(&set)));
}

#[test]
fn from_par_iter_matches_from_iter_for_large_input() {
    let input: Vec<I8CO> = (0..BATCH_SIZE * 16)
        .map(|i| match i % 8 {
            0 => iv(40, 45),
            1 => iv(-60, -58),
            2 => iv(-50, -48),
            3 => iv(-58, -54),
            4 => iv(-49, -45),
            5 => iv(30, 31),
            6 => iv(31, 34),
            _ => iv(60, 61),
        })
        .collect();

    let sequential: I8COSet = input.iter().copied().collect();
    let parallel: I8COSet = input.into_par_iter().collect();

    assert_eq!(parallel, sequential);
    assert!(is_canonical(intervals(&parallel)));
}
