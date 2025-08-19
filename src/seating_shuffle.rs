use crate::interfaces::PlayersMap;
use crate::shuffle::shuffle;
use lcg_rand::rand::LCG;

const POSSIBLE_PLACEMENTS: [(u8, u8, u8, u8); 24] = [
    (0u8, 1u8, 2u8, 3u8),
    (1u8, 0u8, 2u8, 3u8),
    (2u8, 0u8, 1u8, 3u8),
    (3u8, 0u8, 1u8, 2u8),
    (0u8, 1u8, 3u8, 2u8),
    (1u8, 0u8, 3u8, 2u8),
    (2u8, 0u8, 3u8, 1u8),
    (3u8, 0u8, 2u8, 1u8),
    (0u8, 2u8, 1u8, 3u8),
    (1u8, 2u8, 0u8, 3u8),
    (2u8, 1u8, 0u8, 3u8),
    (3u8, 1u8, 0u8, 2u8),
    (0u8, 2u8, 3u8, 1u8),
    (1u8, 2u8, 3u8, 0u8),
    (2u8, 1u8, 3u8, 0u8),
    (3u8, 1u8, 2u8, 0u8),
    (0u8, 3u8, 1u8, 2u8),
    (1u8, 3u8, 0u8, 2u8),
    (2u8, 3u8, 0u8, 1u8),
    (3u8, 2u8, 0u8, 1u8),
    (0u8, 3u8, 2u8, 1u8),
    (1u8, 3u8, 2u8, 0u8),
    (2u8, 3u8, 1u8, 0u8),
    (3u8, 2u8, 1u8, 0u8),
];

/// Shuffled seating with random optimization
/// Note: placement uses previous seatings to try to minimize crossings, so this is not a fair random in general.
pub fn make_shuffled_seating(
    players_map: &PlayersMap,
    previous_seatings: &Vec<Vec<u32>>,
    groups_count: u32,
    rand_factor: u64,
) -> Option<PlayersMap> {
    const MAX_ITERATIONS: usize = 1000;
    let mut best_seating = Vec::new();
    let mut factor = 100500; // lower is better, so init with very big number

    if players_map.is_empty() {
        return Some(Vec::new());
    }

    // Split into groups
    let group_size = (players_map.len() as u32 + groups_count - 1) / groups_count;
    let mut groups: Vec<PlayersMap> = Vec::new();

    for chunk in players_map.chunks(group_size as usize) {
        groups.push(chunk.to_vec());
    }

    for i in 0..MAX_ITERATIONS {
        let mut random: LCG = LCG::new();
        random.seed = rand_factor + (i as u64) * 17;

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

    update_places_at_each_table(&best_seating, previous_seatings)
}

/// Make sure players will initially sit to winds that they did not seat before
/// (or sat less times)
fn update_places_at_each_table(
    seating: &PlayersMap,
    previous_seatings: &Vec<Vec<u32>>,
) -> Option<PlayersMap> {
    let mut tables = Vec::new();
    for chunk in seating.chunks(4) {
        tables.push(chunk.to_vec());
    }

    let mut result_seating = Vec::new();
    for table in tables {
        let mut best_result = 10005000;
        let mut best_placement = Vec::new();

        for placement in &POSSIBLE_PLACEMENTS {
            let new_result = calc_sub_sums(
                table[placement.0 as usize].0,
                table[placement.1 as usize].0,
                table[placement.2 as usize].0,
                table[placement.3 as usize].0,
                previous_seatings,
            );

            if new_result < best_result {
                best_result = new_result;
                best_placement = vec![
                    table[placement.0 as usize],
                    table[placement.1 as usize],
                    table[placement.2 as usize],
                    table[placement.3 as usize],
                ];
            }
        }

        result_seating.extend(best_placement);
    }

    Some(result_seating)
}

/// Calculate generalized value of seating applicability.
/// Sequential games of same players add +10 to factor, while simple crossings add only +1.
/// Less factor value is better!
fn calculate_intersection_factor(seating: &PlayersMap, previous_seatings: &Vec<Vec<u32>>) -> i32 {
    let mut factor = 0;
    let mut crossings: Vec<Vec<Vec<u32>>> = Vec::new();

    let tables_count = seating.len() / 4;
    let mut games = Vec::new();

    // Chunk previous seatings into games
    for chunk in previous_seatings.chunks(tables_count.max(1)) {
        games.push(chunk.to_vec());
    }

    // Add new seating
    let new_seating: Vec<u32> = seating.iter().map(|(id, _)| *id).collect();
    let mut new_seating_chunks: Vec<Vec<u32>> = Vec::new();
    for chunk in new_seating.chunks(4) {
        new_seating_chunks.push(chunk.to_vec());
    }
    games.push(new_seating_chunks);

    for (game_idx, tables) in games.iter().enumerate() {
        for game in tables {
            for player1 in 0..game.len() {
                crossings.push(Vec::new());
                for player2 in 0..game.len() {
                    crossings[player1].push(Vec::new());
                    if player1 == player2 {
                        continue;
                    }
                    crossings[player1][player2].push(game_idx as u32);
                }
            }
        }
    }

    for opponents_list in crossings {
        for crossing_list in opponents_list {
            if crossing_list.len() <= 1 {
                continue;
            }

            factor += 1;
            let mut sorted_crossings = crossing_list.clone();
            sorted_crossings.sort();

            for i in 0..sorted_crossings.len() - 1 {
                if sorted_crossings[i + 1] - sorted_crossings[i] == 1 {
                    // players will play two sequential games
                    factor += 10;
                }
            }
        }
    }

    factor / 2 // div by 2 because of symmetrical matrix counting
}

/// Calculate index of distribution equality for seating at particular
/// winds. Ideally, we want that seating, which produces smallest index.
fn calc_sub_sums(
    player1: u32,
    player2: u32,
    player3: u32,
    player4: u32,
    prev_data: &Vec<Vec<u32>>,
) -> u32 {
    let mut total_sum = 0;

    for (idx, &player) in [player1, player2, player3, player4].iter().enumerate() {
        let mut buckets = [0u32, 0u32, 0u32, 0u32];
        buckets[idx] += 1;

        for table in prev_data {
            if let Some(idx_at_table) = table.iter().position(|&p| p == player) {
                buckets[idx_at_table] += 1;
            }
        }

        total_sum += buckets[0].abs_diff(buckets[1])
            + buckets[0].abs_diff(buckets[2])
            + buckets[0].abs_diff(buckets[3])
            + buckets[1].abs_diff(buckets[2])
            + buckets[1].abs_diff(buckets[3])
            + buckets[2].abs_diff(buckets[3]);
    }

    total_sum
}
