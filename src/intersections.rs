use crate::interfaces::PlayersMap;

/// Format seating data for better view
pub fn make_intersections_table(
    seating: &PlayersMap,
    previous_seatings: &Vec<Vec<u32>>,
) -> Vec<(u32, u32, u32)> {
    let possible_intersections = vec![[0, 1], [0, 2], [0, 3], [1, 2], [1, 3], [2, 3]];

    let mut new_seating_chunks = Vec::new();
    for chunk in seating.chunks(4) {
        let table: Vec<u32> = chunk.iter().map(|(id, _)| *id).collect();
        new_seating_chunks.push(table);
    }

    let mut all_seatings = previous_seatings.clone();
    all_seatings.extend(new_seating_chunks);

    let mut intersection_data: Vec<(u32, u32, u32)> = Vec::new();
    for game in &all_seatings {
        for intersection in &possible_intersections {
            let item = intersection_data
                .iter()
                .position(|x| (*x).0 == game[intersection[0]] && (*x).1 == game[intersection[1]]);
            match item {
                Some(i) => intersection_data[i].2 += 1,
                None => {
                    intersection_data.push((game[intersection[0]], game[intersection[1]], 1));
                }
            }
        }
    }

    intersection_data.iter().map(|x| (*x).clone()).collect()
}
