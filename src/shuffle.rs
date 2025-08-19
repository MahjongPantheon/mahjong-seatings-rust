use crate::interfaces::PlayersMap;
use lcg_rand::rand::LCG;

/// Make sure players will sit on random winds
pub fn update_places_to_random(seating: &PlayersMap, rand_factor: u64) -> Option<PlayersMap> {
    let mut tables = Vec::new();
    for chunk in seating.chunks(4) {
        tables.push(chunk.to_vec());
    }

    let mut result_seating = Vec::new();

    let mut random: LCG = LCG::new();
    random.seed = rand_factor;

    for table in tables {
        result_seating.extend(shuffle(&table, &mut random));
    }

    Some(result_seating)
}

/// Shuffle array while maintaining its keys
/// Should rely on seeded RNG
pub fn shuffle(array: &[(u32, i32)], random: &mut LCG) -> Vec<(u32, i32)> {
    let mut result = array.to_vec();
    let mut i = result.len();

    while i > 1 {
        i -= 1;
        let j = (random.next() * (i as u64 + 1u64)) as usize;
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
