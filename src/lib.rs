#![forbid(unsafe_code)]

mod interfaces;
mod intersections;
mod matrix;
mod minimap;
mod primes;
mod seating_interval;
mod seating_shuffle;
mod seating_swiss;
mod shuffle;

pub use crate::seating_interval::make_interval_seating;
pub use crate::seating_shuffle::make_shuffled_seating;
pub use crate::seating_swiss::make_swiss_seating;
