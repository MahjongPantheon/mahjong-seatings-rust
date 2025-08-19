use crate::interfaces::PlayersMap;
use lcg_rand::rand::LCG;

/// Make sure players will sit on random winds
pub fn update_places_to_random(seating: &PlayersMap, rand_factor: u64) -> PlayersMap {
    let mut tables = Vec::new();
    for chunk in seating.chunks(4) {
        tables.push(chunk.to_vec());
    }

    let mut result_seating = Vec::new();

    let mut random: LCG = LCG::from_seed(rand_factor);

    for table in tables {
        result_seating.extend(shuffle(&table, &mut random));
    }

    result_seating
}

/// Shuffle array while maintaining its keys
/// Should rely on seeded RNG
pub fn shuffle(array: &[(u32, i32)], random: &mut LCG) -> Vec<(u32, i32)> {
    let mut result = array.to_vec();
    let mut i = result.len();

    while i > 1 {
        i -= 1;
        let j = random.next() as usize % array.len();
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
        let mut random = LCG::from_seed(1278);
        let array = vec![(1, 1), (2, 2), (3, 3), (4, 4)];
        let result = shuffle(&array, &mut random);
        assert_eq!(result, vec![(3, 3), (2, 2), (1, 1), (4, 4)]);
    }
}
