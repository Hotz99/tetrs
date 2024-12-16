use crate::{game, pentominoes};
use std::{collections::HashSet, rc::Rc};

use priority_queue::PriorityQueue;

mod heuristic;

pub fn search(
    initial_state: game::State,
    permutations: &pentominoes::Permutations,
    id_manager: &mut game::IdManager,
    lookahead_size: &u8,
) -> Option<game::State> {
    let mut queue = PriorityQueue::new();
    let mut visited = HashSet::new();

    let rc_initial_state = Rc::new(initial_state);

    visited.insert(Rc::clone(&rc_initial_state));
    queue.push(rc_initial_state, 0);

    loop {
        let (current_state, _) = queue.pop()?;

        if current_state.remaining_pieces.is_empty() {
            let mut rc_current_state = Rc::clone(&current_state);
            // return N-1 parent states, where N is App::lookahead_size
            for _ in 1..*lookahead_size {
                rc_current_state = match &rc_current_state.parent_state {
                    Some(parent) => Rc::clone(parent),
                    // only initial_state has no parent_state
                    None => break,
                };
            }

            // access and clone() `uncleared_state` field
            // then unwrap(), then dereference `Box` containing `State`
            return Some(*rc_current_state.uncleared_state.clone().unwrap());
        }

        let piece_to_place = current_state.remaining_pieces[0];
        // generate_states() will only clone() into uncleared_state if is_first_generation
        let is_first_generation = current_state.remaining_pieces.len() == *lookahead_size as usize;

        let child_states = generate_states(
            &current_state,
            piece_to_place,
            permutations,
            id_manager,
            is_first_generation,
        );

        for mut child in child_states {
            let heuristic = heuristic::apply(&mut child, id_manager);
            let rc_child = Rc::new(child);

            if visited.insert(Rc::clone(&rc_child)) {
                queue.push(rc_child, heuristic);
            }
        }
    }
}

fn generate_states(
    rc_parent_state: &Rc<game::State>,
    piece: char,
    permutations: &pentominoes::Permutations,
    id_manager: &mut game::IdManager,
    is_first_generation: bool,
) -> Vec<game::State> {
    let mut states = Vec::new();

    let pent_id = pentominoes::char_to_id(piece);
    let composite_id = game::create_composite_id(pent_id, id_manager.next_unique_id(pent_id));

    for mutation in &permutations[pent_id as usize] {
        for row in 0..=(game::FIELD_HEIGHT - mutation.len()) {
            for col in 0..=(game::FIELD_WIDTH - mutation[0].len()) {
                // [row][col] is top-left of 2d vec 'mutation'
                if !can_place(rc_parent_state.field.as_ref(), mutation, row, col) {
                    continue;
                }

                let mut child_state = game::State {
                    // https://rust-lang.github.io/rust-clippy/master/index.html#needless_borrow
                    // before: parent_state: Some(&rc_parent_state)
                    // after:
                    parent_state: Some(Rc::clone(rc_parent_state)),
                    uncleared_state: None,
                    field: place_piece(
                        rc_parent_state.field.clone(),
                        mutation,
                        composite_id,
                        row,
                        col,
                    ),
                    cleared_rows: rc_parent_state.cleared_rows,
                    remaining_pieces: rc_parent_state.remaining_pieces.clone(),
                };

                if is_first_generation {
                    child_state.uncleared_state = Some(Box::new(child_state.clone()));
                }

                child_state.remaining_pieces.remove(0);
                states.push(child_state);
            }
        }
    }

    states
}

// https://rust-lang.github.io/rust-clippy/master/index.html#ptr_arg
// before: mutation: &Vec<Vec<u8>>
// after: mutation: &[Vec<u8>]
fn can_place(field: &game::GameField, mutation: &[Vec<u8>], row: usize, col: usize) -> bool {
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
            if field[tile_row][tile_col] != game::EMPTY {
                return false;
            }

            // if not at the bottom of the field
            if tile_row < field.len() - 1 {
                // if bottom-most mutation tile
                if delta_row == mutation.len() - 1 {
                    if field[tile_row + 1][tile_col] == game::EMPTY {
                        // tile is floating
                        floating_tiles += 1;
                    }
                } else if mutation[delta_row + 1][delta_col] == 0
                    && field[tile_row + 1][tile_col] == game::EMPTY
                {
                    floating_tiles += 1;
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
    mut field: game::GameField,
    // https://rust-lang.github.io/rust-clippy/master/index.html#ptr_arg
    // before: mutation: &Vec<Vec<u8>>,
    // after:
    mutation: &[Vec<u8>],
    composite_id: u16,
    row: usize,
    col: usize,
) -> game::GameField {
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

#[cfg(test)]
mod tests {

    use super::*;
    use game::EMPTY;

    #[test]
    fn test_try_place() {
        let mut state = game::State::new(crate::DEFAULT_LOOKAHEAD_SIZE);

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
        let mut state_a = game::State::new(crate::DEFAULT_LOOKAHEAD_SIZE);

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

        let mut state_b = game::State::new(crate::DEFAULT_LOOKAHEAD_SIZE);

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

        let mut id_manager = game::IdManager::default();

        let heuristic_a = heuristic::apply(&mut state_a, &mut id_manager);
        let heuristic_b = heuristic::apply(&mut state_b, &mut id_manager);

        println!("HEURISTIC A: {}", heuristic_a);

        println!("HEURISTIC B: {}", heuristic_b);

        // heuristic_a should be greater than heuristic_b
        assert!(state_a.field.len() == 15);
        assert!(state_b.field.len() == 15);
        assert!(heuristic_a < heuristic_b);
    }
}
