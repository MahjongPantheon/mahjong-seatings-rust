use crate::{
    make_interval_seating, make_interval_seating_with_table_size, make_swiss_seating,
    make_swiss_seating_with_table_size, PlayersMap, WindShuffle,
};

fn players(count: u32) -> PlayersMap {
    (1..=count).map(|id| (id, 1000 - id as i32)).collect()
}

fn tables(seating: &PlayersMap, size: usize) -> Vec<Vec<u32>> {
    seating
        .chunks(size)
        .map(|table| table.iter().map(|&(id, _)| id).collect())
        .collect()
}

#[test]
fn sanma_interval_keeps_rating_order_step_and_remainder_tables() {
    let input: PlayersMap = players(12).into_iter().rev().collect();
    for (step, expected) in [
        (
            1,
            vec![
                vec![1, 2, 3],
                vec![4, 5, 6],
                vec![7, 8, 9],
                vec![10, 11, 12],
            ],
        ),
        (
            2,
            vec![
                vec![1, 3, 5],
                vec![2, 4, 6],
                vec![7, 9, 11],
                vec![8, 10, 12],
            ],
        ),
        (
            3,
            vec![
                vec![1, 4, 7],
                vec![2, 5, 8],
                vec![3, 6, 9],
                vec![10, 11, 12],
            ],
        ),
        (
            5,
            vec![
                vec![1, 2, 3],
                vec![4, 5, 6],
                vec![7, 8, 9],
                vec![10, 11, 12],
            ],
        ),
    ] {
        let result = make_interval_seating_with_table_size(
            &input,
            &vec![],
            step,
            123,
            WindShuffle::Prescripted,
            3,
        );
        assert_eq!(tables(&result, 3), expected);
        for &(id, rating) in &result {
            assert_eq!(rating, 1000 - id as i32);
        }
    }
}

#[test]
fn sanma_swiss_prioritizes_ratings_without_history() {
    let result =
        make_swiss_seating_with_table_size(&players(9), &vec![], 123, WindShuffle::Prescripted, 3);
    assert_eq!(
        tables(&result, 3),
        vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]
    );
}

#[test]
fn sanma_swiss_avoids_previous_opponents_when_possible() {
    let previous = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
        vec![1, 4, 7],
        vec![2, 5, 8],
        vec![3, 6, 9],
    ];
    let input = players(9);
    let result =
        make_swiss_seating_with_table_size(&input, &previous, 123, WindShuffle::Balanced, 3);
    let mut sorted = result.clone();
    sorted.sort();
    assert_eq!(sorted, input);
    for table in tables(&result, 3) {
        for old in &previous {
            assert!(table.iter().filter(|id| old.contains(id)).count() <= 1);
        }
    }
    assert_eq!(
        result,
        make_swiss_seating_with_table_size(&input, &previous, 123, WindShuffle::Balanced, 3)
    );
}

#[test]
fn sanma_swiss_relaxes_crossings_when_repeats_are_unavoidable() {
    let input = players(6);
    let previous = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let mut result =
        make_swiss_seating_with_table_size(&input, &previous, 123, WindShuffle::Random, 3);
    result.sort();
    assert_eq!(result, input);
}

#[test]
fn explicit_four_player_modes_match_original_entry_points() {
    let input = players(12);
    let previous = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
    for mode in [
        WindShuffle::Random,
        WindShuffle::Balanced,
        WindShuffle::Prescripted,
    ] {
        // The enum does not implement Copy, so use a fresh value for each call.
        let fresh_mode = || match mode {
            WindShuffle::Random => WindShuffle::Random,
            WindShuffle::Balanced => WindShuffle::Balanced,
            WindShuffle::Prescripted => WindShuffle::Prescripted,
        };
        assert_eq!(
            make_interval_seating(&input, &previous, 2, 123, fresh_mode()),
            make_interval_seating_with_table_size(&input, &previous, 2, 123, fresh_mode(), 4)
        );
        assert_eq!(
            make_swiss_seating(&input, &previous, 123, fresh_mode()),
            make_swiss_seating_with_table_size(&input, &previous, 123, fresh_mode(), 4)
        );
    }
}

#[test]
fn empty_sanma_modes_return_empty_seating() {
    assert!(make_interval_seating_with_table_size(
        &vec![],
        &vec![],
        1,
        123,
        WindShuffle::Random,
        3
    )
    .is_empty());
    assert!(
        make_swiss_seating_with_table_size(&vec![], &vec![], 123, WindShuffle::Random, 3)
            .is_empty()
    );
}

#[test]
#[should_panic(expected = "Interval step must be positive")]
fn interval_rejects_zero_step() {
    make_interval_seating_with_table_size(&players(3), &vec![], 0, 123, WindShuffle::Random, 3);
}

#[test]
#[should_panic(expected = "Incomplete table")]
fn swiss_rejects_incomplete_tables() {
    make_swiss_seating_with_table_size(&players(4), &vec![], 123, WindShuffle::Random, 3);
}

#[test]
#[should_panic(expected = "Invalid previous table size")]
fn swiss_rejects_four_player_history_in_sanma() {
    make_swiss_seating_with_table_size(
        &players(3),
        &vec![vec![1, 2, 3, 4]],
        123,
        WindShuffle::Random,
        3,
    );
}
