use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::{fmt, thread};

use super::bot::Bot;
use super::id_manager;

pub const FIELD_WIDTH: u8 = 5;
pub const FIELD_HEIGHT: u8 = 15;
pub const EMPTY: u16 = 12;

#[derive(Eq, Clone)]
pub struct State {
    pub field: Vec<Vec<u16>>,
    // box to avoid recursive type
    pub parent_state: Option<Box<State>>,
    pub remaining_pieces: Vec<char>,
    pub cleared_rows: u32,
}

impl State {
    pub fn new(
        field: &Vec<Vec<u16>>,
        parent_state: Option<Box<State>>,
        remaining_pieces: &Vec<char>,
    ) -> State {
        State {
            field: field.clone(),
            parent_state: Some(parent_state.clone().unwrap()),
            remaining_pieces: remaining_pieces.clone(),
            cleared_rows: parent_state.unwrap().cleared_rows,
        }
    }

    pub fn initial_state(pieces: &Vec<char>) -> State {
        State {
            field: vec![vec![EMPTY; FIELD_WIDTH as usize]; FIELD_HEIGHT as usize],
            parent_state: None,
            remaining_pieces: pieces.clone(),
            cleared_rows: 0,
        }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Cleared: {}\n", self.cleared_rows)?;

        // writeln!(f, "Field:");
        for row in &self.field {
            for &tile in row {
                if tile == EMPTY {
                    write!(f, "_ ")?;
                    continue;
                }

                let symbol = match id_manager::get_pent_id(tile) {
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
