#![forbid(unsafe_code)]
#![allow(dead_code)]
mod interfaces;
mod intersections;
mod matrix;
mod minimap;
mod primes;
mod seating_interval;
mod seating_shuffle;
mod seating_swiss;
mod shuffle;
mod wind_balance;

pub use crate::interfaces::PlayersMap;
pub use crate::interfaces::WindShuffle;
pub use crate::seating_interval::{make_interval_seating, make_interval_seating_with_table_size};
pub use crate::seating_shuffle::make_shuffled_seating;
pub use crate::seating_swiss::{make_swiss_seating, make_swiss_seating_with_table_size};
pub use crate::shuffle::update_wind_placing;

pub use crate::seating_shuffle::make_shuffled_seating_with_table_size;
pub use crate::shuffle::update_wind_placing_with_table_size;

#[cfg(test)]
mod sanma_tests {
    use super::*;

    #[test]
    fn shuffled_tables_include_every_real_player_once_and_avoid_previous_opponents() {
        let players: PlayersMap = (1..=9).map(|id| (id, 1500)).collect();
        let previous = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let result = make_shuffled_seating_with_table_size(
            &players,
            &previous,
            1,
            123456,
            WindShuffle::Balanced,
            3,
        );
        let mut ids: Vec<u32> = result.iter().map(|&(id, _)| id).collect();
        ids.sort();
        assert_eq!(ids, (1..=9).collect::<Vec<_>>());
        for table in result.chunks(3) {
            for old_table in &previous {
                assert!(
                    table
                        .iter()
                        .filter(|&&(id, _)| old_table.contains(&id))
                        .count()
                        <= 1
                );
            }
        }
        assert_eq!(
            result,
            make_shuffled_seating_with_table_size(
                &players,
                &previous,
                1,
                123456,
                WindShuffle::Balanced,
                3
            )
        );
    }

    #[test]
    fn wind_placement_preserves_three_player_tables() {
        let players: PlayersMap = (1..=6).map(|id| (id, 1500)).collect();
        let previous = vec![vec![1, 2, 3], vec![4, 5, 6]];
        for mode in [
            WindShuffle::Random,
            WindShuffle::Balanced,
            WindShuffle::Prescripted,
        ] {
            let result = update_wind_placing_with_table_size(mode, &players, &previous, 99, 3);
            for (table, old) in result.chunks(3).zip(players.chunks(3)) {
                let mut sorted = table.to_vec();
                sorted.sort();
                assert_eq!(sorted, old);
            }
        }
        let balanced =
            update_wind_placing_with_table_size(WindShuffle::Balanced, &players, &previous, 99, 3);
        assert!(balanced.iter().zip(&players).all(|(new, old)| new != old));
        assert_eq!(
            update_wind_placing_with_table_size(
                WindShuffle::Prescripted,
                &players,
                &previous,
                99,
                3
            ),
            players
        );
    }

    #[test]
    #[should_panic(expected = "Incomplete table")]
    fn rejects_incomplete_tables() {
        make_shuffled_seating_with_table_size(
            &vec![(1, 0), (2, 0)],
            &vec![],
            1,
            1,
            WindShuffle::Random,
            3,
        );
    }

    #[test]
    #[should_panic(expected = "Table size must be 3 or 4")]
    fn rejects_unsupported_table_sizes() {
        update_wind_placing_with_table_size(WindShuffle::Random, &vec![], &vec![], 1, 0);
    }

    #[test]
    #[should_panic(expected = "Groups count must be positive")]
    fn rejects_zero_groups() {
        make_shuffled_seating_with_table_size(
            &vec![(1, 0), (2, 0), (3, 0)],
            &vec![],
            0,
            1,
            WindShuffle::Random,
            3,
        );
    }

    #[test]
    fn accepts_empty_player_lists() {
        assert!(make_shuffled_seating_with_table_size(
            &vec![],
            &vec![],
            1,
            1,
            WindShuffle::Random,
            3
        )
        .is_empty());
    }
}

#[cfg(test)]
mod seating_modes_tests;
