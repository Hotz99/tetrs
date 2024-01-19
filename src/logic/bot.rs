use std::collections::{BinaryHeap, HashSet};

use priority_queue::PriorityQueue;

use crate::data::pentomino_db::PentominoDB;

use super::{
    game,
    id_manager::{self, IdManager},
    next_shapes,
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
        let mut queue = PriorityQueue::new();

        visited.insert(state.clone());
        queue.push(state.clone(), 0);

        while let Some((current_state, _)) = queue.pop() {
            if current_state.remaining_pieces.is_empty() {
                let mut final_state = current_state;

                for _ in 0..next_shapes::STACK_SIZE - 1 {
                    match final_state.parent_state {
                        Some(parent_state) => final_state = *parent_state,
                        None => break,
                    }
                }

                println!("queue size: {}", queue.len());

                return Some(final_state);
            }

            let children_states = Self::generate_states(
                &current_state,
                current_state.remaining_pieces[0],
                pent_db,
                id_manager,
            );

            if children_states.len() == 0 {
                println!("NO_CHILD");
            }

            for mut child in children_states {
                if visited.insert(child.clone()) {
                    let score = Self::heuristic(&mut child);
                    queue.push(child, score);
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
                    let mut child_state = parent_state.clone();

                    // [row][col] is top-left of 2d vec 'mutation'
                    if Self::try_place(&mut child_state, mutation, composite_id, row, col) {
                        child_state.parent_state = Some(Box::new(parent_state.clone()));
                        child_state.remaining_pieces.remove(0);

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

                // if tile is out of bounds
                if tile_col >= state.field[0].len() || tile_row >= state.field.len() {
                    return false;
                }

                // if overlapping with other tiles
                if state.field[tile_row][tile_col] != state::EMPTY {
                    return false;
                }

                // if not at the bottom of the field
                if tile_row != state.field.len() - 1 {
                    // if bottom-most tile
                    if delta_row == mutation.len() - 1 {
                        // if below is empty
                        if state.field[tile_row + 1][tile_col] == state::EMPTY {
                            // tile is floating
                            return false;
                        }
                    }
                }

                // actually place the tile
                state.field[tile_row][tile_col] = composite_id;
            }
        }

        true
    }

    pub fn heuristic(state: &mut State) -> i32 {
        let full_rows = Self::count_full_rows(&state.field) as i32;

        let mut score = 0;

        score += full_rows ^ 4 * 9000;

        // println!("b4 clear {}", state);

        game::clear_full_rows(state, true);

        // println!("after clear {}", state);

        for row in 0..state::FIELD_HEIGHT {
            // score bias towards bottom rows
            let penalize_top = (12 * state::FIELD_HEIGHT as i32 / (row as i32 + 1)) << 13;

            for col in 0..state::FIELD_WIDTH {
                if state.field[row as usize][col as usize] != state::EMPTY {
                    score -= penalize_top;
                } else {
                    score += penalize_top;
                }
            }
        }

        score
    }

    fn count_full_rows(field: &Vec<Vec<u16>>) -> u8 {
        let mut full_rows = 0;

        for row in field.iter().rev() {
            if row.iter().all(|&cell| cell != state::EMPTY) {
                full_rows += 1;
            }
        }

        full_rows
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use state::EMPTY;

    #[test]
    fn test_try_place() {
        let mut state = State::initial_state(&vec!['X', 'I', 'Z', 'T', 'U']);

        state.field = vec![
            vec![1, 1, EMPTY, EMPTY, EMPTY],
            vec![1, 1, EMPTY, EMPTY, EMPTY],
            vec![1, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
        ];

        let piece_id = 10;
        let piece = vec![vec![1, 1], vec![1, 1], vec![1, 0]];

        let result = Bot::try_place(&mut state, &piece, piece_id, 0, 2);

        assert_eq!(result, false);
    }

    #[test]
    fn test_heuristic() {
        let mut state_a = State::initial_state(&vec!['X', 'I', 'Z', 'T', 'U']);

        state_a.field = vec![
            vec![1, 2, 3, 4, EMPTY],
            vec![1, 2, 3, 4, EMPTY],
            vec![1, 2, 3, 4, EMPTY],
            vec![1, 2, 3, 4, EMPTY],
            vec![1, 2, 3, 4, EMPTY],
            vec![1, 2, 3, 4, EMPTY],
            vec![1, 2, 3, 4, EMPTY],
            vec![1, 2, 3, 4, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
        ];

        let mut state_b = State::initial_state(&vec!['X', 'I', 'Z', 'T', 'U']);

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
            vec![1, 2, 3, 4, EMPTY],
            vec![1, 2, 3, 4, EMPTY],
            vec![1, 2, 3, 4, EMPTY],
            vec![1, 2, 3, 4, EMPTY],
        ];

        println!("HEURISTIC A: {}", Bot::heuristic(&mut state_a));

        println!("HEURISTIC B: {}", Bot::heuristic(&mut state_b));

        // heuristic_a should be greater than heuristic_b
        assert!(state_a.field.len() == 15);
        assert!(state_b.field.len() == 15);
        assert!(Bot::heuristic(&mut state_a) < Bot::heuristic(&mut state_b));
    }
}
