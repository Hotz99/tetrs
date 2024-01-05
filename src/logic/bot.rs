use std::collections::{BinaryHeap, HashSet};

use crate::data::pentomino_db::PentominoDB;

use super::state::{self, State};

pub struct Bot {}

impl Bot {
    pub fn astar_search(state: &State, pieces: &Vec<char>, pent_db: &PentominoDB) -> Option<State> {
        let initial_field = state.field.clone();

        let used_ids = if state.used_ids.is_empty() {
            vec![false; u16::MAX as usize]
        } else {
            state.used_ids.clone()
        };

        let initial_state = State::new(&initial_field, None, &pieces, &used_ids);

        let mut visited = HashSet::new();
        let mut queue = BinaryHeap::new();

        visited.insert(initial_state.clone());
        queue.push(initial_state);

        while let Some(current_state) = queue.pop() {
            if current_state.remaining_pieces.is_empty() {
                // implement visualization
                return Some(
                    *current_state
                        .parent_state
                        .unwrap()
                        .parent_state
                        .unwrap()
                        .parent_state
                        .unwrap()
                        .parent_state
                        .unwrap()
                        .parent_state
                        .unwrap(),
                );
            }

            let next_states =
                Self::generate_states(&current_state, current_state.remaining_pieces[0], pent_db);

            for next_state in next_states {
                if visited.insert(next_state.clone()) {
                    queue.push(next_state);
                }
            }
        }

        println!("No solution found");
        None
    }

    fn generate_states(parent_state: &State, piece: char, pent_db: &PentominoDB) -> Vec<State> {
        let field = &parent_state.field;
        let remaining_pieces = &parent_state.remaining_pieces;
        let ids = &parent_state.used_ids;

        let mut states = Vec::new();
        let pent_id = Self::char_to_id(piece);

        for mutation in &pent_db.data[pent_id as usize] {
            let new_piece = mutation;
            for x in 0..=state::FIELD_WIDTH - new_piece.len() as u8 {
                for y in 0..=state::FIELD_HEIGHT - new_piece[0].len() as u8 {
                    let mut field_clone = field.clone();

                    let mut new_state = State::new(
                        &field_clone,
                        Some(Box::new(parent_state.clone())),
                        &remaining_pieces,
                        &ids,
                    );

                    if Self::can_place(&mut field_clone, new_piece, x, y) {
                        new_state.remaining_pieces.retain(|&p| p != piece);
                        states.push(new_state);
                    }
                }
            }
        }
        states
    }

    fn char_to_id(c: char) -> u8 {
        match c {
            'X' => 0,
            'I' => 1,
            'Z' => 2,
            'T' => 3,
            'U' => 4,
            'V' => 5,
            'W' => 6,
            'Y' => 7,
            'L' => 8,
            'P' => 9,
            'N' => 10,
            'F' => 11,
            _ => 0,
        }
    }

    fn can_place(field: &Vec<Vec<u8>>, piece: &Vec<Vec<u8>>, x: u8, y: u8) -> bool {
        for (i, row) in piece.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                let field_x = x as usize + j;
                let field_y = y as usize + i;

                // if cell is out of bounds
                if field_x >= field[0].len() || field_y >= field.len() {
                    return false;
                }

                // if already occupied / overlap with other pieces
                if field[field_y][field_x] != 0 {
                    return false;
                }
            }
        }
        true
    }
}
