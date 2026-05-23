use crate::u8::test_support::{intervals, iv};

use super::*;

#[test]
fn from_iter_empty() {
    let set: U8COSet = Vec::<U8CO>::new().into_iter().collect();

    assert!(intervals(&set).is_empty());
}

#[test]
fn from_iter_sorts_and_merges_overlap_adjacency_and_duplicates() {
    let set: U8COSet = [
        iv(12, 14),
        iv(0, 3),
        iv(2, 6), // overlaps [0, 3)
        iv(8, 10),
        iv(6, 8),   // connects [0, 6) with [8, 10)
        iv(12, 14), // duplicate
        iv(10, 11), // remains separated from [12, 14)
    ]
    .into_iter()
    .collect();

    assert_eq!(intervals(&set), &[iv(0, 11), iv(12, 14)]);
}

#[test]
fn from_iter_merges_across_batch_boundary() {
    let mut input = vec![iv(0, 1); BATCH_SIZE];

    // This interval is placed in the next partial batch. The final merge
    // must still join it with the canonical run from the full first batch.
    input.push(iv(1, 2));

    let set: U8COSet = input.into_iter().collect();

    assert_eq!(intervals(&set), &[iv(0, 2)]);
}

#[test]
fn from_iter_merges_multiple_full_batches() {
    let mut input = Vec::with_capacity(BATCH_SIZE * 2 + 1);

    input.extend(std::iter::repeat_n(iv(0, 2), BATCH_SIZE));
    input.extend(std::iter::repeat_n(iv(2, 4), BATCH_SIZE));
    input.push(iv(4, 5));

    let set: U8COSet = input.into_iter().collect();

    assert_eq!(intervals(&set), &[iv(0, 5)]);
}

#[test]
fn from_iter_matches_whole_input_normalization() {
    let input = [
        iv(30, 35),
        iv(1, 4),
        iv(7, 10),
        iv(3, 8),
        iv(50, 51),
        iv(49, 50),
        iv(20, 22),
        iv(21, 25),
        iv(30, 35),
    ];

    let expected = normalize(input.to_vec());
    let set: U8COSet = input.into_iter().collect();

    assert_eq!(intervals(&set), expected.as_slice());
    assert!(is_canonical(intervals(&set)));
}
