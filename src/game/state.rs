use crate::game;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

// u16 has enough bits to store pentomino id and rotation
pub type GameField = Vec<Vec<u16>>;

#[derive(Eq, Clone, Debug)]
pub struct State {
    // Rc<T> for multiple ownership of parent_state between children states
    // also avoids inifinite size
    pub parent_state: Option<Rc<State>>,
    // Box<T> for self-referential data, avoids inifinite size
    pub uncleared_state: Option<Box<State>>,
    pub field: GameField,
    pub remaining_pieces: Vec<char>,
    pub cleared_rows: u32,
}

impl State {
    pub fn new(lookahead_size: u8) -> Self {
        State {
            parent_state: None,
            uncleared_state: None,
            field: vec![vec![game::EMPTY; game::FIELD_WIDTH]; game::FIELD_HEIGHT],
            remaining_pieces: Vec::with_capacity(lookahead_size as usize),
            cleared_rows: 0,
        }
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "cleared rows: {}", self.cleared_rows)?;

        writeln!(f, "cleared:")?;
        for row in &self.field {
            for &tile in row {
                if tile == game::EMPTY {
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

impl Hash for State {
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

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.field == other.field && self.remaining_pieces == other.remaining_pieces
    }
}
