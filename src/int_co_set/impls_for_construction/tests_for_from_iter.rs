use int_interval::I8CO;

use crate::{
    I8COSet,
    int_co_set::test_support::{intervals, iv},
};

use super::*;

#[test]
fn from_iter_empty() {
    let set: I8COSet = Vec::<I8CO>::new().into_iter().collect();

    assert!(intervals(&set).is_empty());
}

#[test]
fn from_iter_sorts_and_merges_overlap_adjacency_and_duplicates() {
    let set: I8COSet = [
        iv(12, 14),
        iv(-20, -17),
        iv(-18, -14), // overlaps [-20, -17)
        iv(-12, -10),
        iv(-14, -12), // connects [-20, -14) with [-12, -10)
        iv(12, 14),   // duplicate
        iv(-10, -9),  // remains separated from [12, 14)
    ]
    .into_iter()
    .collect();

    assert_eq!(intervals(&set), &[iv(-20, -9), iv(12, 14)]);
}

#[test]
fn from_iter_merges_across_batch_boundary() {
    let mut input = vec![iv(-1, 0); BATCH_SIZE];

    // This interval is placed in the next partial batch. The final merge
    // must still join it with the canonical run from the full first batch.
    input.push(iv(0, 1));

    let set: I8COSet = input.into_iter().collect();

    assert_eq!(intervals(&set), &[iv(-1, 1)]);
}

#[test]
fn from_iter_merges_multiple_full_batches() {
    let mut input = Vec::with_capacity(BATCH_SIZE * 2 + 1);

    input.extend(std::iter::repeat_n(iv(-2, 0), BATCH_SIZE));
    input.extend(std::iter::repeat_n(iv(0, 2), BATCH_SIZE));
    input.push(iv(2, 3));

    let set: I8COSet = input.into_iter().collect();

    assert_eq!(intervals(&set), &[iv(-2, 3)]);
}

#[test]
fn from_iter_matches_whole_input_normalization() {
    let input = [
        iv(30, 35),
        iv(-40, -37),
        iv(-34, -31),
        iv(-38, -33),
        iv(50, 51),
        iv(49, 50),
        iv(-10, -8),
        iv(-9, -5),
        iv(30, 35),
    ];

    let expected = normalize(input.to_vec());
    let set: I8COSet = input.into_iter().collect();

    assert_eq!(intervals(&set), expected.as_slice());
    assert!(is_canonical(intervals(&set)));
}
