use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::time::Duration;
use std::{fmt, thread};

use crate::ui;

pub const FIELD_WIDTH: u8 = 5;
pub const FIELD_HEIGHT: u8 = 15;
pub const EMPTY: u8 = 12;

#[derive(Eq, Clone)]
pub struct State {
    pub field: Vec<Vec<u8>>,
    pub uncleared_field: Vec<Vec<u8>>,
    pub parent_state: Option<Box<State>>,
    pub heuristic: i32,
    pub remaining_pieces: Vec<char>,
    pub used_ids: Vec<bool>,
    pub cleared_rows: u32,
}

impl State {
    pub fn new(
        field: &Vec<Vec<u8>>,
        uncleared_field: &Vec<Vec<u8>>,
        parent_state: Option<Box<State>>,
        remaining_pieces: &Vec<char>,
        used_ids: &Vec<bool>,
    ) -> State {
        State {
            field: field.clone(),
            uncleared_field: uncleared_field.clone(),
            parent_state: Some(parent_state.clone().unwrap()),
            heuristic: 0,
            remaining_pieces: remaining_pieces.clone(),
            used_ids: used_ids.clone(),
            cleared_rows: parent_state.unwrap().cleared_rows,
        }
    }

    pub fn initial_state(pieces: &Vec<char>) -> State {
        State {
            field: vec![vec![EMPTY; FIELD_WIDTH as usize]; FIELD_HEIGHT as usize],
            uncleared_field: vec![vec![EMPTY; FIELD_WIDTH as usize]; FIELD_HEIGHT as usize],
            parent_state: None,
            heuristic: 0,
            remaining_pieces: pieces.clone(),
            used_ids: Vec::new(),
            cleared_rows: 0,
        }
    }

    pub fn get_full_rows(&mut self) -> u32 {
        let mut cleared_rows = 0;
        let mut new_field = vec![vec![EMPTY; FIELD_WIDTH as usize]; FIELD_HEIGHT as usize];

        for row in self.field.iter().rev() {
            if row.iter().all(|&cell| cell != EMPTY) {
                cleared_rows += 1;
            } else {
                new_field.pop();
                new_field.insert(0, row.clone());
            }
        }

        self.field = new_field;

        cleared_rows
    }

    pub fn animate_clearing(&mut self) {
        let mut new_field = vec![vec![EMPTY; FIELD_WIDTH as usize]; FIELD_HEIGHT as usize];

        for row in self.field.iter().rev() {
            thread::sleep(Duration::from_millis(10));

            ui::field_ui::draw_field(&new_field);

            if row.iter().all(|&cell| cell != EMPTY) {
                new_field.pop();
                new_field.insert(0, vec![EMPTY; FIELD_WIDTH as usize]);
            } else {
                new_field.pop();
                new_field.insert(0, row.clone());
            }
        }

        self.field = new_field;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clear_full_rows() {
        let field = vec![
            vec![9, 9, EMPTY, EMPTY, EMPTY],
            vec![9, 9, EMPTY, EMPTY, EMPTY],
            vec![9, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1],
        ];

        let mut state = State::initial_state(&vec!['I', 'Z', 'T', 'L', 'O']);
        state.field = field;

        let full_rows = state.get_full_rows();

        assert_eq!(full_rows, 2);

        assert_eq!(state.field.len(), 15);
        assert_eq!(state.field[13], vec![EMPTY; FIELD_WIDTH as usize]);
        assert_eq!(state.field[14], vec![EMPTY; FIELD_WIDTH as usize]);

        assert_eq!(state.cleared_rows, 2);
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "\nHeuristic: {}", self.heuristic)?;
        writeln!(f, "Cleared: {}\n", self.cleared_rows)?;

        // writeln!(f, "Field:");
        for row in &self.field {
            for &cell in row {
                let symbol = match cell {
                    0 => 'X',
                    1 => 'I',
                    2 => 'Z',
                    3 => 'T',
                    4 => 'U',
                    5 => 'V',
                    6 => 'W',
                    7 => 'Y',
                    8 => 'L',
                    9 => 'P',
                    10 => 'N',
                    11 => 'F',
                    _ => '_',
                };
                write!(f, "{} ", symbol)?;
            }
            writeln!(f)?;
        }

        // writeln!(f, "Uncleared Field:");
        // for row in &self.uncleared_field {
        //     for &cell in row {
        //         let symbol = match cell {
        //             0 => 'X',
        //             1 => 'I',
        //             2 => 'Z',
        //             3 => 'T',
        //             4 => 'U',
        //             5 => 'V',
        //             6 => 'W',
        //             7 => 'Y',
        //             8 => 'L',
        //             9 => 'P',
        //             10 => 'N',
        //             11 => 'F',
        //             _ => '_',
        //         };
        //         write!(f, "{} ", symbol)?;
        //     }
        //     writeln!(f)?;
        // }

        Ok(())
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
        // makes astar queue a min-heap
        other.heuristic.cmp(&self.heuristic)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
