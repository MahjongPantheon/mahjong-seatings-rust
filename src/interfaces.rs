use serde::{Deserialize, Serialize};

pub type PlayersMap = Vec<(u32, i32)>; // (id, rating)

#[derive(Serialize, Deserialize)]
pub struct SeatedPlayer {
    pub id: u32,
    pub local_id: u32,
}

#[derive(Serialize, Deserialize)]
pub struct TableWithRating {
    pub players: Vec<(u32, i32)>, // Array of (id, rating) pairs
    pub max_rating: i32,          // Max rating at table
}

pub type PlayedWithMatrix = Vec<Vec<usize>>; // [playerIdx1][playerIdx2] => games played together
