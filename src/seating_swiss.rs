use crate::interfaces::{PlayedWithMatrix, PlayersMap};
use crate::shuffle::update_places_to_random;
use std::collections::HashMap;

/// Swiss seating entry point
/// Wrapper for formats conformity
pub fn make_swiss_seating(
    players_map: &PlayersMap,
    previous_seatings: &Vec<Vec<u32>>,
    rand_factor: u64,
) -> PlayersMap {
    let mut played_with: PlayedWithMatrix = Vec::new();
    let players_idx: Vec<u32> = players_map.iter().map(|(id, _)| *id).collect();

    // Initialize played_with matrix
    for player1 in 0..players_map.len() {
        played_with.push(Vec::new());
        for _ in 0..players_map.len() {
            played_with[player1].push(0);
        }
    }

    // Count previous games played together
    for table in previous_seatings {
        for i in 0..4 {
            for j in 0..4 {
                if i == j {
                    continue;
                }
                let player1 = players_idx
                    .iter()
                    .position(|x| *x == table[i])
                    .unwrap_or(usize::MAX);
                let player2 = players_idx
                    .iter()
                    .position(|x| *x == table[j])
                    .unwrap_or(usize::MAX);
                if let Some(player1_map) = played_with.get_mut(player1) {
                    if let Some(count) = player1_map.get_mut(player2) {
                        *count += 1;
                    }
                }
            }
        }
    }

    let ratings: Vec<i32> = players_map.iter().map(|(_, rating)| *rating).collect();
    let player_table = swiss_seating_original(&ratings, &played_with);

    let mut player_table_vec: Vec<(usize, u32)> = player_table.into_iter().collect();
    player_table_vec.sort_by(|a, b| a.1.cmp(&b.1));

    let sorted_players: PlayersMap = player_table_vec
        .into_iter()
        .map(|(idx, _table)| players_map[idx])
        .collect();

    update_places_to_random(&sorted_players, rand_factor).unwrap_or_default()
}

/// Swiss seating generator
/// Algorithm was taken from mahjongsoft.ru website.
fn swiss_seating_original(
    player_total_game_points: &[i32],
    played_with: &PlayedWithMatrix,
) -> HashMap<usize, u32> {
    let num_players = player_total_game_points.len();
    let mut is_playing = vec![false; num_players];
    let mut player_table = HashMap::new();

    for i in 0..num_players {
        player_table.insert(i, u32::MAX); // -1 equivalent
    }

    let mut max_crossings = 0;
    let mut played_with_mut = played_with.clone();

    while !swiss_seating_internal(
        &mut is_playing,
        max_crossings,
        0,
        num_players,
        &mut player_table,
        player_total_game_points,
        &mut played_with_mut,
        0,
    ) {
        max_crossings += 1;
    }

    // Convert u32::MAX back to proper table numbers
    for (_, table) in player_table.iter_mut() {
        if *table == u32::MAX {
            *table = 0;
        }
    }

    player_table
}

/// Recursive swiss seating algorithm.
/// Taken from mahjongsoft.ru
fn swiss_seating_internal(
    is_playing: &mut [bool],
    max_crossings: u32,
    max_crossings_precision_factor: u32,
    num_players: usize,
    player_table: &mut HashMap<usize, u32>,
    player_total_game_points: &[i32],
    played_with: &mut PlayedWithMatrix,
    mut iteration: u32,
) -> bool {
    iteration += 1;
    if iteration > 15000 {
        return swiss_seating_internal(
            is_playing,
            max_crossings,
            max_crossings_precision_factor + 1,
            num_players,
            player_table,
            player_total_game_points,
            played_with,
            0,
        );
    }

    // Check if everybody has taken a seat
    if is_playing.iter().all(|&playing| playing) {
        return true;
    }

    // Find table with highest index and highest players count already at that table
    let mut max_table = 0;
    let mut players_on_max_table = Vec::new();

    for (i, &table) in player_table.iter() {
        if table != u32::MAX && table > max_table {
            max_table = table;
            players_on_max_table.clear();
            players_on_max_table.push(*i);
        } else if table == max_table && table != u32::MAX {
            players_on_max_table.push(*i);
        }
    }

    let num_players_on_max_table = players_on_max_table.len();

    // If table is already filled, take next table and place there a player with highest rating
    if num_players_on_max_table == 0 || num_players_on_max_table == 4 {
        if num_players_on_max_table == 4 {
            max_table += 1;
        }

        // Find a player with highest rating
        let mut max_gp = i32::MIN;
        let mut max_rating_player = None;
        for (i, &playing) in is_playing.iter().enumerate() {
            if !playing && player_total_game_points[i] > max_gp {
                max_gp = player_total_game_points[i];
                max_rating_player = Some(i);
            }
        }

        if let Some(player) = max_rating_player {
            is_playing[player] = true;
            player_table.insert(player, max_table);

            return if swiss_seating_internal(
                is_playing,
                max_crossings + max_crossings_precision_factor,
                max_crossings_precision_factor,
                num_players,
                player_table,
                player_total_game_points,
                played_with,
                iteration,
            ) {
                true
            } else {
                is_playing[player] = false;
                player_table.insert(player, u32::MAX);
                false
            };
        }
    } else {
        // There are already players at the table
        let mut cur_crossings = 0;

        loop {
            let mut next_players = Vec::new();

            for (i, &playing) in is_playing.iter().enumerate() {
                if !playing {
                    let mut num_crossings = 0;
                    for &table_player in &players_on_max_table {
                        if let Some(player1_map) = played_with.get_mut(i) {
                            if let Some(count) = player1_map.get_mut(table_player) {
                                num_crossings += *count;
                            }
                        }
                    }

                    if num_crossings <= cur_crossings {
                        next_players.push(i);
                    }
                }
            }

            if !next_players.is_empty() {
                // Sort players by rating (descending)
                next_players.sort_by(|&a, &b| {
                    player_total_game_points[b].cmp(&player_total_game_points[a])
                });

                // Try each candidate
                for &player in &next_players {
                    is_playing[player] = true;
                    player_table.insert(player, max_table);

                    // Update played_with matrix
                    for &table_player in &players_on_max_table {
                        if let Some(player_map) = played_with.get_mut(player) {
                            if let Some(count) = player_map.get_mut(table_player) {
                                *count += 1;
                            }
                        }
                        if let Some(table_player_map) = played_with.get_mut(table_player) {
                            if let Some(count) = table_player_map.get_mut(player) {
                                *count += 1;
                            }
                        }
                    }

                    if swiss_seating_internal(
                        is_playing,
                        max_crossings + max_crossings_precision_factor - cur_crossings as u32,
                        max_crossings_precision_factor,
                        num_players,
                        player_table,
                        player_total_game_points,
                        played_with,
                        iteration,
                    ) {
                        return true;
                    } else {
                        // Backtrack
                        is_playing[player] = false;
                        player_table.insert(player, u32::MAX);

                        // Revert played_with matrix
                        for &table_player in &players_on_max_table {
                            if let Some(player_map) = played_with.get_mut(player) {
                                if let Some(count) = player_map.get_mut(table_player) {
                                    *count -= 1;
                                }
                            }
                            if let Some(table_player_map) = played_with.get_mut(table_player) {
                                if let Some(count) = table_player_map.get_mut(player) {
                                    *count -= 1;
                                }
                            }
                        }
                    }
                }
                break;
            } else if cur_crossings == (max_crossings + max_crossings_precision_factor) as usize {
                return false;
            } else {
                cur_crossings += 1;
            }
        }
    }

    false
}
