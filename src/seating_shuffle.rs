use crate::interfaces::{PlayersMap, WindShuffle};
use crate::shuffle::{shuffle, update_wind_placing};
use crate::wind_balance::calculate_intersection_factor;
use lcg_rand::rand::LCG;

/// Shuffled seating with random optimization
/// Note: placement uses previous seatings to try to minimize crossings, so this is not a fair random in general.
pub fn make_shuffled_seating(
    players_map: &PlayersMap,
    previous_seatings: &Vec<Vec<u32>>,
    groups_count: u32,
    rand_factor: u64,
    wind_shuffle: WindShuffle,
) -> PlayersMap {
    const MAX_ITERATIONS: usize = 1000;
    let mut best_seating = Vec::new();
    let mut factor = 100500; // lower is better, so init with very big number

    if players_map.is_empty() {
        return Vec::new();
    }

    // Split into groups
    let group_size = (players_map.len() as u32 + groups_count - 1) / groups_count;
    let mut groups: Vec<PlayersMap> = Vec::new();

    for chunk in players_map.chunks(group_size as usize) {
        groups.push(chunk.to_vec());
    }

    for i in 0..MAX_ITERATIONS {
        let mut random: LCG = LCG::from_seed(rand_factor + (i as u64) * 17);

        // Shuffle each group
        for group in &mut groups {
            *group = shuffle(group, &mut random);
        }

        // Flatten groups
        let flattened_groups: PlayersMap = groups.iter().flatten().copied().collect();

        let new_factor = calculate_intersection_factor(&flattened_groups, previous_seatings);
        if new_factor < factor {
            factor = new_factor;
            best_seating = flattened_groups;
        }
    }

    update_wind_placing(wind_shuffle, &best_seating, previous_seatings, rand_factor)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_shuffled_seating_initial() {
        let players = vec![
            (1, 1500),
            (2, 1500),
            (3, 1500),
            (4, 1500),
            (5, 1500),
            (6, 1500),
            (7, 1500),
            (8, 1500),
            (9, 1500),
            (10, 1500),
            (11, 1500),
            (12, 1500),
        ];

        let seating = make_shuffled_seating(&players, &vec![], 1, 3464752, WindShuffle::Balanced);
        assert_eq!(seating.len(), 12);
        assert_eq!(
            seating,
            vec![
                (8, 1500),
                (2, 1500),
                (4, 1500),
                (10, 1500),
                (6, 1500),
                (1, 1500),
                (7, 1500),
                (3, 1500),
                (12, 1500),
                (11, 1500),
                (5, 1500),
                (9, 1500)
            ]
        );
    }

    #[test]
    fn test_make_shuffled_seating_after_first_game() {
        let players = vec![
            (1, 1500),
            (2, 1500),
            (3, 1500),
            (4, 1500),
            (5, 1500),
            (6, 1500),
            (7, 1500),
            (8, 1500),
            (9, 1500),
            (10, 1500),
            (11, 1500),
            (12, 1500),
            (13, 1500),
            (14, 1500),
            (15, 1500),
            (16, 1500),
        ];

        let previous_seating: Vec<Vec<u32>> = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];

        let seating = make_shuffled_seating(
            &players,
            &previous_seating,
            1,
            123456,
            WindShuffle::Balanced,
        );
        assert_eq!(seating.len(), 16);
        assert_eq!(
            seating,
            vec![
                (4, 1500),
                (3, 1500),
                (10, 1500),
                (14, 1500),
                (12, 1500),
                (11, 1500),
                (13, 1500),
                (6, 1500),
                (7, 1500),
                (16, 1500),
                (8, 1500),
                (9, 1500),
                (15, 1500),
                (5, 1500),
                (2, 1500),
                (1, 1500)
            ]
        );
    }

    #[test]
    fn test_make_shuffled_seating_after_several_games() {
        let players = vec![
            (1, 1500),
            (2, 1500),
            (3, 1500),
            (4, 1500),
            (5, 1500),
            (6, 1500),
            (7, 1500),
            (8, 1500),
            (9, 1500),
            (10, 1500),
            (11, 1500),
            (12, 1500),
            (13, 1500),
            (14, 1500),
            (15, 1500),
            (16, 1500),
        ];

        let previous_seating: Vec<Vec<u32>> = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
            vec![1, 5, 9, 13],
            vec![2, 6, 10, 14],
            vec![3, 7, 11, 15],
            vec![4, 8, 12, 16],
        ];

        let seating = make_shuffled_seating(
            &players,
            &previous_seating,
            1,
            9486370,
            WindShuffle::Balanced,
        );
        assert_eq!(seating.len(), 16);
        assert_eq!(
            seating,
            vec![
                (15, 1500),
                (12, 1500),
                (8, 1500),
                (1, 1500),
                (14, 1500),
                (9, 1500),
                (6, 1500),
                (10, 1500),
                (16, 1500),
                (13, 1500),
                (4, 1500),
                (2, 1500),
                (7, 1500),
                (3, 1500),
                (5, 1500),
                (11, 1500)
            ]
        );
    }

    #[test]
    fn test_make_shuffled_seating_grouped() {
        let players = vec![
            (1, 1508),
            (2, 1507),
            (3, 1506),
            (4, 1505),
            (5, 1504),
            (6, 1503),
            (7, 1502),
            (8, 1501),
            (9, 1500),
            (10, 1499),
            (11, 1498),
            (12, 1498),
            (13, 1497),
            (14, 1496),
            (15, 1495),
            (16, 1494),
        ];

        let previous_seating: Vec<Vec<u32>> = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];

        let seating = make_shuffled_seating(
            &players,
            &previous_seating,
            2,
            3464752,
            WindShuffle::Balanced,
        );
        assert_eq!(seating.len(), 16);
        assert_eq!(
            seating,
            vec![
                (2, 1507),
                (3, 1506),
                (5, 1504),
                (6, 1503),
                (7, 1502),
                (8, 1501),
                (4, 1505),
                (1, 1508),
                (14, 1496),
                (13, 1497),
                (9, 1500),
                (11, 1498),
                (16, 1494),
                (12, 1498),
                (10, 1499),
                (15, 1495)
            ]
        );
    }
}
