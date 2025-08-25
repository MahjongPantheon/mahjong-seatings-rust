use crate::interfaces::{PlayersMap, TableWithRating};
use crate::shuffle::update_places_to_random;

/// Make interval seating
/// Players from the top are seating with interval of $step, but if table count is
/// not divisible by $step, rest of players are seated with step 1.
pub fn make_interval_seating(
    current_rating_list: &PlayersMap,
    step: usize,
    rand_factor: u64,
) -> PlayersMap {
    let mut tables = Vec::new();
    let mut current_table = Vec::new();

    // These guys from bottom could not be placed with desired interval, so they play with interval 1
    let players_to_seat_with_no_interval = 4 * ((current_rating_list.len() / 4) % step);
    // These guys from top should be placed as required
    let players_possible_to_seat_with_interval =
        current_rating_list.len() - players_to_seat_with_no_interval;

    // Fill tables with interval of $step
    for offset in 0..step {
        let mut i = offset;
        while i < players_possible_to_seat_with_interval {
            current_table.push(current_rating_list[i]);
            if current_table.len() == 4 {
                let max_rating = current_table
                    .iter()
                    .map(|(_, rating)| *rating)
                    .max()
                    .unwrap_or(-1000000);
                tables.push(TableWithRating {
                    players: current_table.clone(),
                    max_rating,
                });
                current_table.clear();
            }
            i += step;
        }
    }

    // Fill rest of tables with interval 1
    for i in players_possible_to_seat_with_interval..current_rating_list.len() {
        current_table.push(current_rating_list[i]);
        if current_table.len() == 4 {
            let max_rating = current_table
                .iter()
                .map(|(_, rating)| *rating)
                .max()
                .unwrap_or(-1000000);
            tables.push(TableWithRating {
                players: current_table.clone(),
                max_rating,
            });
            current_table.clear();
        }
    }

    // Sort tables by top player score
    tables.sort_by(|a, b| b.max_rating.cmp(&a.max_rating));

    let mut flattened_groups = Vec::new();
    for table in tables {
        flattened_groups.extend(table.players);
    }

    update_places_to_random(&flattened_groups, rand_factor)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_interval_seating_step1() {
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

        let seating = make_interval_seating(&players, 1, 12345);

        assert_eq!(
            seating,
            vec![
                (4, 1505),
                (1, 1508),
                (2, 1507),
                (3, 1506),
                (6, 1503),
                (8, 1501),
                (5, 1504),
                (7, 1502),
                (12, 1498),
                (9, 1500),
                (10, 1499),
                (11, 1498),
                (14, 1496),
                (15, 1495),
                (16, 1494),
                (13, 1497)
            ]
        );
    }

    #[test]
    fn test_make_interval_seating_step2() {
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

        let seating = make_interval_seating(&players, 2, 12345);

        assert_eq!(
            seating,
            vec![
                (7, 1502),
                (1, 1508),
                (3, 1506),
                (5, 1504),
                (4, 1505),
                (8, 1501),
                (2, 1507),
                (6, 1503),
                (15, 1495),
                (9, 1500),
                (11, 1498),
                (13, 1497),
                (12, 1498),
                (14, 1496),
                (16, 1494),
                (10, 1499)
            ]
        );
    }

    #[test]
    fn test_make_interval_seating_step3() {
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

        let seating = make_interval_seating(&players, 3, 12345);

        assert_eq!(
            seating,
            vec![
                (10, 1499),
                (1, 1508),
                (4, 1505),
                (7, 1502),
                (5, 1504),
                (11, 1498),
                (2, 1507),
                (8, 1501),
                (12, 1498),
                (3, 1506),
                (6, 1503),
                (9, 1500),
                (14, 1496),
                (15, 1495),
                (16, 1494),
                (13, 1497)
            ]
        );
    }

    #[test]
    fn test_make_interval_seating_step4() {
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

        let seating = make_interval_seating(&players, 4, 12345);

        assert_eq!(
            seating,
            vec![
                (13, 1497),
                (1, 1508),
                (5, 1504),
                (9, 1500),
                (6, 1503),
                (14, 1496),
                (2, 1507),
                (10, 1499),
                (15, 1495),
                (3, 1506),
                (7, 1502),
                (11, 1498),
                (8, 1501),
                (12, 1498),
                (16, 1494),
                (4, 1505)
            ]
        );
    }
}
