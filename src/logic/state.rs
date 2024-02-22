use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::{fmt, thread};

use super::{game, id_manager};

pub type Field = Vec<Vec<u16>>;

pub const FIELD_WIDTH: usize = 5;
pub const FIELD_HEIGHT: usize = 15;
pub const EMPTY: u16 = 13;

#[derive(Eq, Clone)]
pub struct State {
    pub field: Field,
    pub remaining_pieces: Vec<char>,
    pub cleared_rows: u32,
    pub used_ids: Vec<bool>,
}

impl State {
    pub fn initial_state(pieces: &Vec<char>) -> State {
        State {
            field: vec![vec![EMPTY; FIELD_WIDTH]; FIELD_HEIGHT],
            remaining_pieces: pieces.clone(),
            cleared_rows: 0,
            used_ids: vec![false; u16::MAX as usize],
        }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "cleared rows: {}", self.cleared_rows)?;

        writeln!(f, "cleared:");
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
