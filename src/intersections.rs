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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_intersection_table() {
        let players_map = vec![
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

        let expected = vec![
            (1, 2, 2),
            (1, 3, 2),
            (1, 4, 2),
            (2, 3, 2),
            (2, 4, 2),
            (3, 4, 2),
            (5, 6, 2),
            (5, 7, 2),
            (5, 8, 2),
            (6, 7, 2),
            (6, 8, 2),
            (7, 8, 2),
            (9, 10, 2),
            (9, 11, 2),
            (9, 12, 2),
            (10, 11, 2),
            (10, 12, 2),
            (11, 12, 2),
            (13, 14, 2),
            (13, 15, 2),
            (13, 16, 2),
            (14, 15, 2),
            (14, 16, 2),
            (15, 16, 2),
        ];

        assert_eq!(
            expected,
            make_intersections_table(&players_map, &previous_seating)
        );
    }
}
