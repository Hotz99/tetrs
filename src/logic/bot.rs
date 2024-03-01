use std::{
    cell::RefCell,
    collections::{BinaryHeap, HashMap, HashSet},
    fs::OpenOptions,
    rc::Rc,
};

use priority_queue::PriorityQueue;

use crate::data::pentomino_db::PentominoDB;

use super::{
    game,
    id_manager::{self, IdManager},
    next_shapes::{self, STACK_SIZE},
    state::{self, Field, State},
};

pub fn search(
    initial_state: State,
    pent_db: &PentominoDB,
    id_manager: &mut IdManager,
) -> Option<State> {
    let mut queue = PriorityQueue::new();
    let mut visited = HashSet::new();

    let initial_state_rc = Rc::new(initial_state);

    visited.insert(Rc::clone(&initial_state_rc));
    queue.push(initial_state_rc, 0);

    loop {
        let (current_state, _) = queue.pop()?;

        if current_state.remaining_pieces.is_empty() {
            // return N-1 parent states, where N is next_shapes::STACK_SIZE
            let final_state = current_state
                .as_ref()
                .clone()
                .parent_state?
                .as_ref()
                .clone()
                .parent_state?
                .as_ref()
                .clone()
                .parent_state?
                .as_ref()
                .clone()
                .parent_state?
                .as_ref()
                .clone();

            return Some(*final_state.uncleared_state.unwrap());
        }

        let piece_to_place = current_state.as_ref().remaining_pieces[0];

        // generate_states() will only clone() into uncleared_state if is_first_generation
        let is_first_generation =
            current_state.as_ref().remaining_pieces.len() == next_shapes::STACK_SIZE;

        let child_states = generate_states(
            current_state,
            piece_to_place,
            pent_db,
            id_manager,
            is_first_generation,
        );

        for mut child in child_states {
            let heuristic = heuristic(&mut child, id_manager);
            let child_rc = Rc::new(child);

            if visited.insert(Rc::clone(&child_rc)) {
                queue.push(child_rc, heuristic);
            }
        }
    }
}

fn generate_states(
    parent_state_rc: Rc<State>,
    piece: char,
    pent_db: &PentominoDB,
    id_manager: &mut id_manager::IdManager,
    is_first_generation: bool,
) -> Vec<State> {
    let mut states = Vec::new();

    let pent_id = game::char_to_id(piece);
    let composite_id = game::create_composite_id(pent_id, id_manager.next_unique_id(pent_id));

    for mutation in &pent_db.data[pent_id as usize] {
        for row in 0..=(state::FIELD_HEIGHT - mutation.len()) {
            for col in 0..=(state::FIELD_WIDTH - mutation[0].len()) {
                // [row][col] is top-left of 2d vec 'mutation'
                if can_place(parent_state_rc.field.as_ref(), mutation, row, col) {
                    let mut child_state = State {
                        parent_state: Some(Rc::clone(&parent_state_rc)),
                        uncleared_state: None,
                        field: place_piece(
                            parent_state_rc.field.clone(),
                            mutation,
                            composite_id,
                            row,
                            col,
                        ),
                        cleared_rows: parent_state_rc.cleared_rows,
                        remaining_pieces: parent_state_rc.remaining_pieces.clone(),
                    };

                    if is_first_generation {
                        child_state.uncleared_state = Some(Box::new(child_state.clone()));
                    }

                    child_state.remaining_pieces.remove(0);
                    states.push(child_state);
                }
            }
        }
    }

    states
}

fn can_place(field: &Field, mutation: &Vec<Vec<u8>>, row: usize, col: usize) -> bool {
    let mut floating_tiles = 0;

    for delta_row in 0..mutation.len() {
        for delta_col in 0..mutation[0].len() {
            // mutation is a 2d vec of 0s and 1s
            if mutation[delta_row][delta_col] == 0 {
                continue;
            }

            // start from top-left of mutation
            let tile_row = row + delta_row;
            let tile_col = col + delta_col;

            // if tile is out of bounds
            if tile_col >= field[0].len() || tile_row >= field.len() {
                return false;
            }

            // if overlapping with other tiles
            if field[tile_row][tile_col] != state::EMPTY {
                return false;
            }

            // if not at the bottom of the field
            if tile_row < field.len() - 1 {
                // if bottom-most mutation tile
                if delta_row == mutation.len() - 1 {
                    if field[tile_row + 1][tile_col] == state::EMPTY {
                        // tile is floating
                        floating_tiles += 1;
                    }
                } else {
                    if mutation[delta_row + 1][delta_col] == 0
                        && field[tile_row + 1][tile_col] == state::EMPTY
                    {
                        floating_tiles += 1;
                    }
                }
            }

            // piece is floating
            if floating_tiles >= mutation[0].len() {
                return false;
            }

            // actually place the tile
            // field[tile_row][tile_col] = composite_id;
        }
    }

    true
}

fn place_piece(
    mut field: Field,
    mutation: &Vec<Vec<u8>>,
    composite_id: u16,
    row: usize,
    col: usize,
) -> Field {
    for delta_row in 0..mutation.len() {
        for delta_col in 0..mutation[0].len() {
            if mutation[delta_row][delta_col] == 0 {
                continue;
            }

            let tile_row = row + delta_row;
            let tile_col = col + delta_col;

            field[tile_row][tile_col] = composite_id;
        }
    }

    field
}

pub fn heuristic(state: &mut State, id_manager: &mut IdManager) -> i32 {
    let mut score = 0;
    let mut penalize_top: i32;

    let cleared_rows = game::update(state, id_manager, 0, true, false, 0) as i32;

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

#[cfg(test)]
mod tests {

    use super::*;
    use state::EMPTY;

    #[test]
    fn test_try_place() {
        let mut state = State::initial_state();

        state.remaining_pieces = vec!['P', 'N', 'F'];

        let p_composite_id = game::create_composite_id(9, 0);

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

        let x_composite_id = game::create_composite_id(0, 0);
        let x_piece = vec![vec![0, 1, 0], vec![1, 1, 1], vec![0, 1, 0]];

        let l_composite_id = game::create_composite_id(8, 0);
        let l_piece = vec![vec![1, 0], vec![1, 0], vec![1, 0], vec![1, 1]];

        println!("result: {}", can_place(&state.field, &l_piece, 8, 1));

        println!("{}", state);
    }

    #[test]
    fn test_heuristic() {
        let mut state_a = State::initial_state();

        state_a.remaining_pieces = vec!['X', 'I', 'Z', 'T', 'U'];

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

        let mut state_b = State::initial_state();

        state_b.remaining_pieces = vec!['X', 'I', 'Z', 'T', 'U'];

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

        let mut id_manager = IdManager::new();

        let heuristic_a = heuristic(&mut state_a, &mut id_manager);
        let heuristic_b = heuristic(&mut state_b, &mut id_manager);

        println!("HEURISTIC A: {}", heuristic_a);

        println!("HEURISTIC B: {}", heuristic_b);

        // heuristic_a should be greater than heuristic_b
        assert!(state_a.field.len() == 15);
        assert!(state_b.field.len() == 15);
        assert!(heuristic_a < heuristic_b);
    }
}
