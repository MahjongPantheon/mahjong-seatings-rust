use crate::interfaces::PlayersMap;
use crate::shuffle::update_places_to_random;
use std::collections::HashMap;

/// Swiss seating entry point
/// Wrapper for formats conformity
pub fn make_swiss_seating(
    players_map: &PlayersMap,
    previous_seatings: &Vec<Vec<u32>>,
    rand_factor: u64,
) -> PlayersMap {
    let played_with = make_played_with_matrix(players_map, previous_seatings);
    let player_table = swiss_seating_original(&players_map, &played_with);
    let player_to_rating: HashMap<u32, i32> =
        players_map.iter().map(|item| (item.0, item.1)).collect();

    let mut result_table: Vec<(u32, i32)> = player_table.into_iter().collect();
    result_table.sort_by(|a, b| a.1.cmp(&b.1));
    result_table = result_table
        .iter()
        .map(|item| (item.0, player_to_rating[&item.0]))
        .collect();

    update_places_to_random(&result_table, rand_factor)
}

/// Swiss seating generator
/// Algorithm was taken from mahjongsoft.ru website.
/// Returns hash map (player_id, table_index) tuples
fn swiss_seating_original(
    players_ratings: &PlayersMap,
    played_with: &HashMap<(u32, u32), u32>,
) -> HashMap<u32, i32> {
    let num_players = players_ratings.len();
    let mut player_table = HashMap::new();
    let mut is_playing = HashMap::new();
    let mut max_crossings = 0;
    let mut played_with_mut = played_with.clone();
    let players_ratings_map: HashMap<u32, i32> = players_ratings
        .iter()
        .map(|item| (item.0, item.1))
        .collect();

    players_ratings.iter().for_each(|item| {
        is_playing.insert(item.0, false);
        player_table.insert(item.0, -1);
    });

    while !swiss_seating_internal(
        &mut is_playing,
        max_crossings,
        0,
        num_players as u32,
        &mut player_table,
        &players_ratings_map,
        &mut played_with_mut,
        0,
    ) {
        max_crossings += 1;
    }

    player_table
}

/// Recursive swiss seating algorithm.
/// Taken from mahjongsoft.ru
fn swiss_seating_internal(
    is_playing: &mut HashMap<u32, bool>, // player_id -> is playing
    max_crossings: u32,
    mut max_crossings_precision_factor: u32,
    num_players: u32,
    player_table: &mut HashMap<u32, i32>,
    players_ratings: &HashMap<u32, i32>, // player_id -> rating
    played_with: &mut HashMap<(u32, u32), u32>,
    mut iteration: u32,
) -> bool {
    iteration += 1;
    if iteration > 15000 {
        max_crossings_precision_factor += 1;
        iteration = 0;
    }

    // Check if everybody has taken a seat
    if is_playing.iter().all(|(_, playing)| *playing) {
        return true;
    }

    let (mut max_table, players_on_max_table) =
        find_highest_table_and_players(players_ratings, player_table);

    // if table is already filled, take next table and place there a player with highest rating

    if players_on_max_table.len() == 0 || players_on_max_table.len() == 4 {
        if players_on_max_table.len() == 4 {
            max_table += 1;
        }

        let max_rating_player = find_player_with_highest_rating(is_playing, &players_ratings);
        if max_rating_player.is_none() {
            return true; // everybody has taken a seat
        }

        // check 'playing' flag and place the player to the table, then call the procedure recursively

        is_playing.insert(max_rating_player.unwrap(), true);
        player_table.insert(max_rating_player.unwrap(), max_table);

        if swiss_seating_internal(
            is_playing,
            max_crossings + max_crossings_precision_factor,
            max_crossings_precision_factor,
            num_players,
            player_table,
            players_ratings,
            played_with,
            iteration,
        ) {
            true
        } else {
            is_playing.insert(max_rating_player.unwrap(), false);
            player_table.insert(max_rating_player.unwrap(), -1);
            false
        }
    } else {
        // There are already players at the table; we should take next players with highest ratings
        let mut cur_crossings = 0;
        let mut next_players = Vec::new();

        loop {
            for i in players_ratings.keys() {
                if !is_playing[&i] {
                    let mut num_crossings = 0;
                    for j in 0..players_on_max_table.len() {
                        num_crossings += played_with[&(*i, players_on_max_table[j])];
                    }
                    if num_crossings <= cur_crossings {
                        next_players.push(*i);
                    }
                }
            }

            if next_players.len() > 0 {
                break;
            } else if cur_crossings == max_crossings + max_crossings_precision_factor {
                return false;
            } else {
                cur_crossings += 1;
            }
        }

        sort_by_rating(&mut next_players, players_ratings);

        // substitute candidates for seating, then make a check
        for i in 0..next_players.len() {
            // check 'playing' flag and place the player to the table, then call the procedure recursively
            set_table_for_player(
                is_playing,
                player_table,
                played_with,
                true,
                max_table,
                next_players[i],
                &players_on_max_table,
            );

            // return success if we found a seating, or falling back otherwise
            if swiss_seating_internal(
                is_playing,
                max_crossings + max_crossings_precision_factor - cur_crossings,
                max_crossings_precision_factor,
                num_players,
                player_table,
                players_ratings,
                played_with,
                iteration,
            ) {
                return true;
            } else {
                set_table_for_player(
                    is_playing,
                    player_table,
                    played_with,
                    false,
                    -1,
                    next_players[i],
                    &players_on_max_table,
                );
            }
        }

        false
    }
}

fn make_played_with_matrix(
    players_map: &PlayersMap,
    previous_seatings: &Vec<Vec<u32>>,
) -> HashMap<(u32, u32), u32> {
    let mut played_with: HashMap<(u32, u32), u32> = HashMap::new();

    // Initialize played_with matrix
    for player1 in players_map {
        for player2 in players_map {
            played_with.insert((player1.0, player2.0), 0);
        }
    }

    // Count previous games played together
    for table in previous_seatings {
        for i in 0..4 {
            for j in 0..4 {
                if i == j {
                    continue;
                }
                played_with.insert((table[i], table[j]), 1 + played_with[&(table[i], table[j])]);
            }
        }
    }

    played_with
}

/// Find table with highest index and highest players count already at that table
fn find_highest_table_and_players(
    players_ratings: &HashMap<u32, i32>,
    player_table: &HashMap<u32, i32>,
) -> (i32, Vec<u32>) {
    let mut max_table = 0;
    let mut players_on_max_table = Vec::new();
    for i in players_ratings.keys() {
        if player_table[&i] > max_table {
            max_table = player_table[&i];
            players_on_max_table.clear();
        }
        if player_table[&i] == max_table {
            players_on_max_table.push(*i);
        }
    }

    players_on_max_table.sort_by(|a, b| a.cmp(b));
    (max_table, players_on_max_table)
}

/// Sort first array by rating (mutating)
fn sort_by_rating(next_players: &mut Vec<u32>, players_ratings: &HashMap<u32, i32>) {
    for i in 0..next_players.len() {
        if i + 1 == next_players.len() {
            continue;
        }
        for j in i + 1..next_players.len() {
            if players_ratings[&next_players[i]] < players_ratings[&next_players[j]] {
                next_players.swap(i, j);
            }
        }
    }
}

fn find_player_with_highest_rating(
    is_playing: &HashMap<u32, bool>,
    players_ratings: &HashMap<u32, i32>,
) -> Option<u32> {
    let mut max_gp = i32::MIN;
    let mut max_rating_player = None;
    for (_, (id, playing)) in is_playing.iter().enumerate() {
        if !*playing && players_ratings[id] > max_gp {
            max_gp = players_ratings[id];
            max_rating_player = Some(*id);
        }
    }

    max_rating_player
}

fn set_table_for_player(
    is_playing: &mut HashMap<u32, bool>,
    player_table: &mut HashMap<u32, i32>,
    played_with: &mut HashMap<(u32, u32), u32>,
    set_is_playing: bool,
    set_table_number: i32,
    player: u32,
    players_on_max_table: &Vec<u32>,
) {
    is_playing.insert(player, set_is_playing);
    player_table.insert(player, set_table_number);
    for j in 0..players_on_max_table.len() {
        let play_count = if set_is_playing {
            played_with[&(player, players_on_max_table[j])] + 1
        } else {
            if played_with[&(player, players_on_max_table[j])] > 0 {
                played_with[&(player, players_on_max_table[j])] - 1
            } else {
                0
            }
        };
        played_with.insert((player, players_on_max_table[j]), play_count);
        played_with.insert((players_on_max_table[j], player), play_count);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intersections::make_intersections_table;
    // use crate::intersections::make_intersections_table;

    #[test]
    fn test_make_played_with_matrix() {
        let players_map = vec![
            (1, -1200),
            (2, 9200),
            (3, -13700),
            (4, 4400),
            (5, -27400),
            (6, 10500),
            (7, -29500),
            (8, -8000),
        ];
        let previous_seatings = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![1, 3, 5, 7],
            vec![2, 4, 6, 8],
        ];

        let played_with = make_played_with_matrix(&players_map, &previous_seatings);
        assert_eq!(played_with[&(1, 2)], 1);
        assert_eq!(played_with[&(1, 3)], 2);
        assert_eq!(played_with[&(2, 3)], 1);
        assert_eq!(played_with[&(2, 4)], 2);
        assert_eq!(played_with[&(2, 1)], 1);
        assert_eq!(played_with[&(3, 1)], 2);
        assert_eq!(played_with[&(3, 2)], 1);
        assert_eq!(played_with[&(4, 2)], 2);
    }

    #[test]
    fn test_find_highest_table_and_players() {
        let players_ratings = vec![
            (1, -1200),
            (2, 9200),
            (3, -13700),
            (4, 4400),
            (5, -27400),
            (6, 10500),
            (7, -29500),
            (8, -8000),
        ]
        .into_iter()
        .collect::<HashMap<u32, i32>>();
        let player_table = vec![
            (1, 1),
            (2, 1),
            (3, 1),
            (4, 1),
            (5, 2),
            (6, 2),
            (7, 2),
            (8, 2),
        ]
        .into_iter()
        .collect::<HashMap<u32, i32>>();
        let (max_table, players_on_max_table) =
            find_highest_table_and_players(&players_ratings, &player_table);
        assert_eq!(max_table, 2);
        assert_eq!(players_on_max_table, vec![5, 6, 7, 8]);
    }

    #[test]
    fn test_sort_by_rating() {
        let mut players = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let players_ratings = vec![
            (1, -1200),
            (2, 9200),
            (3, -13700),
            (4, 4400),
            (5, -27400),
            (6, 10500),
            (7, -29500),
            (8, -39500),
        ]
        .into_iter()
        .collect::<HashMap<u32, i32>>();
        sort_by_rating(&mut players, &players_ratings);
        assert_eq!(players, vec![6, 2, 4, 1, 3, 5, 7, 8]);
    }

    #[test]
    fn test_find_player_with_highest_rating() {
        let is_playing = vec![(1, false), (2, true), (3, false), (4, false)]
            .into_iter()
            .collect::<HashMap<u32, bool>>();
        let players_ratings = vec![(1, -1200), (2, 9200), (3, -13700), (4, 4400)]
            .into_iter()
            .collect::<HashMap<u32, i32>>();
        let player = find_player_with_highest_rating(&is_playing, &players_ratings);
        assert_eq!(player, Some(4)); // top player who is not playing
    }

    #[test]
    fn test_set_table_for_player() {
        let mut is_playing = vec![
            (1, false),
            (2, false),
            (3, false),
            (4, false),
            (5, false),
            (6, false),
            (7, false),
            (8, false),
        ]
        .into_iter()
        .collect::<HashMap<u32, bool>>();
        let players_map = vec![
            (1, -1200),
            (2, 9200),
            (3, -13700),
            (4, 4400),
            (5, -27400),
            (6, 10500),
            (7, -29500),
            (8, -8000),
        ];
        let previous_seatings = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![1, 3, 5, 7],
            vec![2, 4, 6, 8],
        ];

        let mut played_with = make_played_with_matrix(&players_map, &previous_seatings);

        let mut player_table = vec![
            (1, 1),
            (2, 1),
            (3, 1),
            (4, 1),
            (5, 2),
            (6, 2),
            (7, 2),
            (8, 2),
        ]
        .into_iter()
        .collect::<HashMap<u32, i32>>();

        set_table_for_player(
            &mut is_playing,
            &mut player_table,
            &mut played_with,
            true,
            4,
            5,
            &vec![5, 6, 7, 8],
        );

        assert_eq!(is_playing[&5], true);
        assert_eq!(player_table[&5], 4);
        assert_eq!(played_with[&(5, 6)], 2);

        set_table_for_player(
            &mut is_playing,
            &mut player_table,
            &mut played_with,
            false,
            -1,
            5,
            &vec![5, 6, 7, 8],
        );

        assert_eq!(is_playing[&5], false);
        assert_eq!(player_table[&5], -1);
        assert_eq!(played_with[&(5, 6)], 1);
    }

    #[test]
    fn test_swiss_seating() {
        let players = vec![
            (1, -1200),
            (2, 9200),
            (3, -13700),
            (4, 4400),
            (5, -27400),
            (6, 10500),
            (7, -29500),
            (8, -8000),
            (9, -23700),
            (10, -9000),
            (11, 1900),
            (12, -38200),
            (13, -1000),
            (14, 13400),
            (15, -34900),
            (16, -19200),
            (17, 8500),
            (18, 11700),
            (19, -32100),
            (20, -4700),
            (21, -15100),
            (22, -2000),
            (23, -25700),
            (24, 21400),
            (25, 40000),
            (26, 64200),
            (27, -14700),
            (28, 49500),
            (29, 35400),
            (30, 1900),
            (31, 59400),
            (32, -31300),
        ];

        let previous_seatings = vec![
            // session 1
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
            vec![17, 18, 19, 20],
            vec![21, 22, 23, 24],
            vec![25, 26, 27, 28],
            vec![29, 30, 31, 32],
            // session 2
            vec![1, 5, 9, 13],
            vec![2, 6, 10, 14],
            vec![3, 7, 11, 15],
            vec![4, 8, 12, 16],
            vec![17, 21, 25, 29],
            vec![18, 22, 26, 30],
            vec![19, 23, 27, 31],
            vec![20, 24, 28, 32],
            // session 3
            vec![26, 14, 31, 24],
            vec![29, 28, 18, 6],
            vec![25, 11, 30, 2],
            vec![4, 22, 13, 17],
            vec![20, 1, 8, 10],
            vec![27, 16, 21, 3],
            vec![7, 9, 23, 32],
            vec![5, 12, 19, 15],
            // // session 4
            // vec![13, 26, 29, 2],
            // vec![11, 28, 17, 31],
            // vec![18, 24, 4, 25],
            // vec![1, 27, 30, 14],
            // vec![9, 6, 15, 22],
            // vec![21, 12, 20, 7],
            // vec![3, 32, 8, 19],
            // vec![16, 5, 10, 23],
            // // session 5
            // vec![26, 17, 6, 1],
            // vec![25, 13, 31, 20],
            // vec![4, 14, 28, 21],
            // vec![29, 11, 5, 24],
            // vec![2, 18, 9, 8],
            // vec![23, 12, 3, 30],
            // vec![16, 19, 7, 22],
            // vec![32, 15, 27, 10],
            // session 6
            /*[26, 20, 11, 4],
            [31, 21, 1, 18],
            [28, 30, 16, 9],
            [12, 25, 32, 6],
            [29, 8, 23, 15],
            [24, 19, 13, 10],
            [3, 5, 22, 14],
            [2, 17, 7, 27],*/

            // session 7
            /*[11, 26, 8, 21],
            [30, 4, 31, 6],
            [12, 2, 22, 28],
            [25, 9, 19, 14],
            [29, 24, 16, 1],
            [10, 3, 13, 18],
            [5, 23, 17, 20],
            [32, 15, 27, 7],*/

            // session 8
            /*[26, 7, 10, 31],
            [23, 1, 25, 28],
            [20, 22, 27, 29],
            [30, 8, 17, 24],
            [32, 18, 14, 11],
            [13, 21, 19, 6],
            [16, 2, 4, 5],
            [12, 3, 9, 15]*/
        ];

        let seating = make_swiss_seating(&players, &previous_seatings, 12345);
        let intersections = make_intersections_table(&seating, &previous_seatings);

        // Swiss seating should produce seating of 32 players in 8 games with no more than 2 intersections of each pair
        intersections.iter().for_each(|item| assert!(item.2.le(&2)));
    }
}
