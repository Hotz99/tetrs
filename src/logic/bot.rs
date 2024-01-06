use std::collections::{BinaryHeap, HashSet};

use crate::data::pentomino_db::PentominoDB;

use super::{
    id_manager::{self, IdManager},
    state::{self, State},
};

pub struct Bot {}

impl Bot {
    pub fn heuristic_search(
        state: &State,
        pent_db: &PentominoDB,
        id_manager: &mut IdManager,
    ) -> Option<State> {
        let mut visited = HashSet::new();
        let mut queue = BinaryHeap::new();

        visited.insert(state.clone());

        queue.push(state.clone());

        while let Some(current_state) = queue.pop() {
            if current_state.remaining_pieces.is_empty() {
                let mut final_state = current_state;

                // better approach ?
                loop {
                    match final_state.parent_state {
                        Some(parent_state) => final_state = *parent_state,
                        None => break,
                    }
                }

                return Some(final_state);
            }

            let children_states = Self::generate_states(
                &current_state,
                current_state.remaining_pieces[0],
                pent_db,
                id_manager,
            );

            for mut child in children_states {
                if visited.insert(child.clone()) {
                    child.heuristic = Self::heuristic(&mut child);
                    queue.push(child);
                }
            }
        }
        None
    }

    fn generate_states(
        parent_state: &State,
        piece: char,
        pent_db: &PentominoDB,
        id_manager: &mut IdManager,
    ) -> Vec<State> {
        let mut states = Vec::new();
        let pent_id = id_manager::char_to_id(piece);
        let composite_id = id_manager::create_composite_id(&pent_id, &id_manager.next_piece_id());

        for mutation in &pent_db.data[pent_id as usize] {
            // reverse order to start with bottom rows
            for row in (0..=(state::FIELD_HEIGHT as usize - mutation.len())).rev() {
                for col in 0..=(state::FIELD_WIDTH as usize - mutation[0].len()) {
                    let mut parent_clone = parent_state.clone();

                    // [row][col] is the top-left of the 2d vec 'mutation'
                    if Self::try_place(&mut parent_clone, mutation, composite_id, row, col) {
                        let child_state = State::new(
                            &parent_clone.field.clone(),
                            &parent_clone.uncleared_field.clone(),
                            Some(Box::new(parent_clone)),
                            // better approach ?
                            &parent_state
                                .remaining_pieces
                                .iter()
                                .filter(|&x| *x != piece)
                                .cloned()
                                .collect(),
                        );

                        states.push(child_state);
                    }
                }
            }
        }
        states
    }

    fn try_place(
        state: &mut State,
        mutation: &Vec<Vec<u8>>,
        composite_id: u16,
        row: usize,
        col: usize,
    ) -> bool {
        for delta_row in (0..mutation.len()).rev() {
            for delta_col in 0..mutation[0].len() {
                // mutation is a 2d vec of 0s and 1s
                if mutation[delta_row][delta_col] == 0 {
                    continue;
                }

                let tile_row = row + delta_row;

                let tile_col = col + delta_col;

                // if cell is out of bounds
                if tile_col >= state.field[0].len() || tile_row >= state.field.len() {
                    return false;
                }

                // if already occupied / overlap with other pieces
                if state.field[tile_row][tile_col] != state::EMPTY {
                    return false;
                }

                // if floating
                if tile_row != state.field.len() - 1 {
                    // if bottom-most tile
                    if delta_row == mutation.len() - 1 {
                        // if below is empty
                        if state.field[tile_row + 1][tile_col] == state::EMPTY {
                            return false;
                        }
                    }
                }

                state.uncleared_field = state.field.clone();

                // actually place the tile
                state.field[tile_row][tile_col] = composite_id;
            }
        }

        true
    }

    pub fn heuristic(state: &mut State) -> i32 {
        let mut penalty = 0;

        for row in 0..state::FIELD_HEIGHT {
            // penalty bias towards bottom rows
            let penalize_top = (12 * state::FIELD_HEIGHT as i32 / (row as i32 + 1)) << 13;

            for col in 0..state::FIELD_WIDTH {
                if state.field[row as usize][col as usize] != state::EMPTY {
                    penalty += penalize_top;
                } else {
                    penalty -= penalize_top;
                }
            }
        }

        let cleared_rows = state.get_full_rows();

        penalty -= (cleared_rows ^ 4 * 9000) as i32;

        penalty
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use state::EMPTY;

    #[test]
    fn test_heuristic() {
        let mut state_a = State::initial_state(&vec![
            'X', 'I', 'Z', 'T', 'U', 'V', 'W', 'Y', 'L', 'P', 'N', 'F',
        ]);

        state_a.field = vec![
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
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
        ];

        let mut state_b = State::initial_state(&vec![
            'X', 'I', 'Z', 'T', 'U', 'V', 'W', 'Y', 'L', 'P', 'N', 'F',
        ]);

        state_b.field = vec![
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
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![9, 9, EMPTY, EMPTY, EMPTY],
            vec![9, 9, EMPTY, EMPTY, EMPTY],
            vec![9, EMPTY, EMPTY, EMPTY, EMPTY],
        ];

        let heuristic_a = Bot::heuristic(&mut state_a);
        println!("HEURISTIC A: {}", heuristic_a);

        let heuristic_b = Bot::heuristic(&mut state_b);
        println!("HEURISTIC B: {}", heuristic_b);

        // heuristic_a should be greater than heuristic_b
        assert!(state_a.field.len() == 15);
        assert!(state_b.field.len() == 15);
        assert!(heuristic_a > heuristic_b);
    }

    #[test]
    fn test_try_place() {
        let mut state = State::initial_state(&vec![
            'X', 'I', 'Z', 'T', 'U', 'V', 'W', 'Y', 'L', 'P', 'N', 'F',
        ]);

        state.field = vec![
            vec![1, 1, EMPTY, EMPTY, EMPTY],
            vec![1, 1, EMPTY, EMPTY, EMPTY],
            vec![1, EMPTY, EMPTY, EMPTY, EMPTY],
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
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
        ];

        let piece_id = 10;
        let piece = vec![vec![1, 1], vec![1, 1], vec![1, 0]];

        let x = 0;
        let y = 0;

        let result = Bot::try_place(&mut state, &piece, piece_id, x, y);

        assert_eq!(result, false);
    }
}
