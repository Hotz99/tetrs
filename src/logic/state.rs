use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

pub const FIELD_WIDTH: u8 = 5;
pub const FIELD_HEIGHT: u8 = 15;

#[derive(Eq, Clone)]
pub struct State {
    pub field: Vec<Vec<u8>>,
    pub parent_state: Option<Box<State>>,
    pub heuristic: i32,
    pub remaining_pieces: Vec<char>,
    pub used_ids: Vec<bool>,
    pub cleared_rows: u32,
}

impl State {
    pub fn new(
        field: &Vec<Vec<u8>>,
        parent_state: Option<Box<State>>,
        remaining_pieces: &Vec<char>,
        used_ids: &Vec<bool>,
    ) -> State {
        State {
            field: field.clone(),
            parent_state: Some(parent_state.clone().unwrap()),
            heuristic: 0,
            remaining_pieces: remaining_pieces.clone(),
            used_ids: used_ids.clone(),
            cleared_rows: 0,
        }
    }

    pub fn initial_state() -> State {
        State {
            field: vec![vec![255; FIELD_WIDTH as usize]; FIELD_HEIGHT as usize],
            parent_state: None,
            heuristic: 0,
            remaining_pieces: Vec::new(),
            used_ids: Vec::new(),
            cleared_rows: 0,
        }
    }

    fn heuristic(&mut self) -> i32 {
        // let cleared_rows_for_round = self.clear_full_rows(search, false, 0, true);
        let cleared_rows_for_round = 0;
        self.cleared_rows += cleared_rows_for_round;

        let mut penalty = 0;
        let empty = 0;

        for row in 0..FIELD_HEIGHT {
            // penalty bias towards bottom rows
            let penalize_top = (12 * FIELD_HEIGHT as i32 / (row as i32 + 1)) << 13;

            for col in 0..FIELD_WIDTH {
                if self.field[row as usize][col as usize] != empty {
                    penalty += penalize_top;
                } else {
                    penalty -= penalize_top;
                }
            }
        }

        penalty -= cleared_rows_for_round.pow(4) as i32 * 9000;

        self.heuristic = penalty;
        penalty
    }

    // Stub for clear_full_rows. Implement the logic as per your requirement.
    fn clear_full_rows(&mut self) -> u32 {
        // Implementation goes here
        0
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for &piece in &self.remaining_pieces {
            piece.hash(state);
        }

        for row in &self.field {
            for &value in row {
                value.hash(state);
            }
        }
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.field == other.field && self.remaining_pieces == other.remaining_pieces
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.heuristic.cmp(&other.heuristic)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
