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
