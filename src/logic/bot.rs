use std::{
    borrow::BorrowMut,
    cell::RefCell,
    collections::{BinaryHeap, HashMap, HashSet},
    fs::OpenOptions,
    rc::Rc,
};

use priority_queue::PriorityQueue;

use crate::data::pentomino_db::PentominoDB;

use super::{
    game, id_manager, next_shapes,
    state::{self, Field, State},
};

pub struct Bot {}

impl Bot {
    pub fn search(initial_state: State, pent_db: &PentominoDB) -> Option<State> {
        let mut parents: HashMap<Rc<State>, Rc<State>> = HashMap::new();
        let mut uncleared_fields: HashMap<Rc<State>, state::Field> = HashMap::new();
        let mut queue = PriorityQueue::new();

        let initial_rc = Rc::new(initial_state);

        queue.push(initial_rc.clone(), 0);

        let mut total_children = 0;

        let mut final_state = loop {
            let queue_len = queue.len();

            let (current_state, _) = queue.pop()?;

            if current_state.remaining_pieces.is_empty() {
                break current_state;
            }

            let children_states =
                Self::generate_states(&current_state, current_state.remaining_pieces[0], pent_db);

            total_children += children_states.len();

            for child in children_states {
                let child_rc = Rc::new(child);
                if parents
                    .insert(child_rc.clone(), current_state.clone())
                    .is_none()
                {
                    // if first generation, insert copy of state.field before clearing rows into hash map
                    // if queue_len == 1 {
                    //     uncleared_fields.insert(child_rc.clone(), child_rc.field.clone());
                    // }

                    let heuristic = Self::heuristic(&mut (*child_rc).clone());
                    queue.push(child_rc.clone(), heuristic);
                }
            }
        };

        loop {
            let parent = parents.get(&final_state)?;

            if *parent == initial_rc {
                break;
            } else {
                final_state = parent.clone();
            }
        }

        Some((*final_state).clone())
    }

    fn generate_states(parent_state: &State, piece: char, pent_db: &PentominoDB) -> Vec<State> {
        let mut states = Vec::new();
        let pent_id = id_manager::char_to_id(piece);
        let mut used_ids = parent_state.used_ids.clone();
        let composite_id = id_manager::create_composite_id(
            &pent_id,
            &id_manager::next_unique_id(&mut used_ids, &pent_id),
        );

        for mutation in &pent_db.data[pent_id as usize] {
            // reverse order to start with bottom rows
            for row in (0..=(state::FIELD_HEIGHT as usize - mutation.len())).rev() {
                for col in 0..=(state::FIELD_WIDTH as usize - mutation[0].len()) {
                    let mut child_state = parent_state.clone();
                    child_state.used_ids = used_ids.clone();

                    // [row][col] is top-left of 2d vec 'mutation'
                    if Self::try_place(&mut child_state, mutation, composite_id, row, col) {
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
        let mut floating_tiles = 0;

        for delta_row in (0..mutation.len()).rev() {
            for delta_col in 0..mutation[0].len() {
                // mutation is a 2d vec of 0s and 1s
                if mutation[delta_row][delta_col] == 0 {
                    continue;
                }

                // start from top-left of mutation
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
                if tile_row < state.field.len() - 1 {
                    // if bottom-most mutation tile
                    if delta_row == mutation.len() - 1 {
                        if state.field[tile_row + 1][tile_col] == state::EMPTY {
                            // tile is floating
                            floating_tiles += 1;
                        }
                    } else {
                        if mutation[delta_row + 1][delta_col] == 0
                            && state.field[tile_row + 1][tile_col] == state::EMPTY
                        {
                            floating_tiles += 1;
                        }
                    }
                }

                // all tiles are floating
                if floating_tiles >= mutation[0].len() {
                    return false;
                }

                // actually place the tile
                state.field[tile_row][tile_col] = composite_id;
            }
        }

        true
    }

    pub fn heuristic(state: &mut State) -> i32 {
        let mut score = 0;
        let mut penalize_top: i32;

        let cleared_rows = game::clear_rows(state) as i32;

        score += cleared_rows ^ 4 * 9000;

        for row in 0..state::FIELD_HEIGHT {
            // score bias towards bottom rows
            penalize_top = 12 * state::FIELD_HEIGHT as i32 / (row as i32 + 1) << 13;

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
}

#[cfg(test)]
mod tests {
    use crate::logic;

    use super::*;
    use state::EMPTY;

    // test search() with a 15x5 field with the field[14][0] filled and the next stack is an L piece
    #[test]
    fn test_search() {
        let mut state = State::initial_state(&vec!['L']);
    }

    #[test]
    fn test_try_place() {
        let mut state = State::initial_state(&vec!['P', 'N', 'F']);

        let p_composite_id = logic::id_manager::create_composite_id(&9, &0);

        state.field = vec![
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
            vec![EMPTY, EMPTY, p_composite_id, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
        ];

        let x_composite_id = logic::id_manager::create_composite_id(&0, &0);
        let x_piece = vec![vec![0, 1, 0], vec![1, 1, 1], vec![0, 1, 0]];

        let l_composite_id = logic::id_manager::create_composite_id(&8, &0);
        let l_piece = vec![vec![1, 0], vec![1, 0], vec![1, 0], vec![1, 1]];

        println!(
            "result: {}",
            Bot::try_place(&mut state, &l_piece, l_composite_id, 8, 1)
        );

        println!("{}", state);
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

        let heuristic_a = Bot::heuristic(&mut state_a);
        let heuristic_b = Bot::heuristic(&mut state_b);

        println!("HEURISTIC A: {}", heuristic_a);

        println!("HEURISTIC B: {}", heuristic_b);

        // heuristic_a should be greater than heuristic_b
        assert!(state_a.field.len() == 15);
        assert!(state_b.field.len() == 15);
        assert!(heuristic_a < heuristic_b);
    }
}
