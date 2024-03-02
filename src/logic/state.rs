use std::cell::{Cell, RefCell};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::{fmt, thread};

use super::{game, id_manager, next_shapes};

pub type Field = Vec<Vec<u16>>;

pub const FIELD_WIDTH: usize = 5;
pub const FIELD_HEIGHT: usize = 15;
pub const EMPTY: u16 = 13;

#[derive(Eq, Clone)]
pub struct GameState {
    pub parent_state: Option<Rc<GameState>>,
    pub uncleared_state: Option<Box<GameState>>,
    pub field: Field,
    pub remaining_pieces: Vec<char>,
    pub cleared_rows: u32,
}

impl GameState {
    pub fn initial_game_state() -> GameState {
        GameState {
            parent_state: None,
            uncleared_state: None,
            field: vec![vec![EMPTY; FIELD_WIDTH]; FIELD_HEIGHT],
            remaining_pieces: Vec::with_capacity(next_shapes::STACK_SIZE),
            cleared_rows: 0,
        }
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "cleared rows: {}", self.cleared_rows)?;

        writeln!(f, "cleared:");
        for row in &self.field {
            for &tile in row {
                if tile == EMPTY {
                    write!(f, "_ ")?;
                    continue;
                }

                let symbol = match game::get_pent_id(tile) {
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

impl Hash for GameState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for piece in &self.remaining_pieces {
            piece.hash(state);
        }

        for row in &self.field {
            for &value in row {
                value.hash(state);
            }
        }
    }
}

impl PartialEq for GameState {
    fn eq(&self, other: &Self) -> bool {
        self.field == other.field && self.remaining_pieces == other.remaining_pieces
    }
}
