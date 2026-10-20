use crate::{interfaces::PlayersMap, wind_balance::update_places_at_each_table, WindShuffle};
use lcg_rand::rand::LCG;

pub fn update_wind_placing(
    wind_shuffle: WindShuffle,
    seating: &PlayersMap,
    previous_seatings: &Vec<Vec<u32>>,
    rand_factor: u64,
) -> PlayersMap {
    update_wind_placing_with_table_size(wind_shuffle, seating, previous_seatings, rand_factor, 4)
}

/// Update winds for complete three- or four-player tables.
pub fn update_wind_placing_with_table_size(
    wind_shuffle: WindShuffle,
    seating: &PlayersMap,
    previous_seatings: &Vec<Vec<u32>>,
    rand_factor: u64,
    table_size: usize,
) -> PlayersMap {
    assert!(
        table_size == 3 || table_size == 4,
        "Table size must be 3 or 4"
    );
    assert!(seating.len() % table_size == 0, "Incomplete table");
    assert!(
        previous_seatings
            .iter()
            .all(|table| table.len() == table_size),
        "Invalid previous table size"
    );
    if wind_shuffle == WindShuffle::Random {
        let mut random = LCG::from_seed(rand_factor);
        seating
            .chunks(table_size)
            .flat_map(|table| shuffle(table, &mut random))
            .collect()
    } else if wind_shuffle == WindShuffle::Balanced {
        if table_size == 4 {
            update_places_at_each_table(seating, previous_seatings, rand_factor)
        } else {
            crate::wind_balance::update_places_at_three_player_tables(
                seating,
                previous_seatings,
                rand_factor,
            )
        }
    } else {
        seating.to_vec()
    }
}

/// Make sure players will sit on random winds.
pub fn update_places_to_random(seating: &PlayersMap, rand_factor: u64) -> PlayersMap {
    update_wind_placing(WindShuffle::Random, seating, &vec![], rand_factor)
}

/// Shuffle array while maintaining its keys
/// Should rely on seeded RNG
pub fn shuffle<T: Clone>(array: &[T], random: &mut LCG) -> Vec<T> {
    let mut result = array.to_vec();
    let mut i = result.len();

    while i > 1 {
        i -= 1;
        let j = random.next() as usize % i;
        if i != j {
            result.swap(i, j);
        }
    }

    result
}

pub fn get_random_seed() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_places_to_random() {
        let seating = vec![(1, 1), (2, 2), (3, 3), (4, 4)];
        let result = update_places_to_random(&seating, 12345);
        assert_eq!(result, vec![(4, 4), (1, 1), (2, 2), (3, 3)]);
    }

    #[test]
    fn test_shuffle() {
        let mut random = LCG::from_seed(1260);
        let array = vec![(1, 1), (2, 2), (3, 3), (4, 4)];
        let result = shuffle(&array, &mut random);
        assert_eq!(result, vec![(2, 2), (4, 4), (1, 1), (3, 3)]);
    }
}
