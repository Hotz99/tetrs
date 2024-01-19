use std::{collections::HashMap, thread};

use super::{
    id_manager::{self, IdManager},
    state::{self, *},
};

pub fn clear_full_rows(state: &mut State, clear: bool) {
    if !clear {
        return;
    };

    for i in (0..FIELD_HEIGHT as usize).rev() {
        let row = &state.field[i];

        if row.iter().all(|&x| x != EMPTY) {
            state.field.remove(i);
            state.field.insert(0, vec![EMPTY; FIELD_WIDTH as usize]);
            state.cleared_rows += 1;
            clear_full_rows(state, true)
        } else {
            clear_full_rows(state, false)
        }
    }
}

pub fn gravity(state: &mut State, id_manager: &mut IdManager) {
    // update composite_id of separated tiles
    for row in 0..FIELD_HEIGHT as usize {
        for col in 0..FIELD_WIDTH as usize {
            let tile = state.field[row][col];
            if tile != EMPTY {
                let pent_id = id_manager::get_pent_id(tile);
                let piece_id = id_manager::get_piece_id(tile);

                if !is_connected(state, row, col, &piece_id) {
                    let new_piece_id = id_manager.next_piece_id();

                    state.field[row][col] =
                        id_manager::create_composite_id(&pent_id, &new_piece_id);
                }
            }
        }
    }

    // map of composite_id to whether it's floating
    let mut floating_ids: HashMap<u16, bool> = HashMap::new();

    loop {
        find_floating(state, &mut floating_ids);

        // TODO: this shit should be false at least once
        if floating_ids.values().all(|&x| x == false) {
            break;
        }

        // shift floating tiles down by 1 row
        // reversed bc we want to shift the bottom-most tiles first
        for row in (0..FIELD_HEIGHT as usize).rev() {
            for col in 0..FIELD_WIDTH as usize {
                let tile = state.field[row][col];

                if tile == EMPTY {
                    continue;
                }

                let piece_id = id_manager::get_piece_id(tile);

                if *floating_ids.get(&tile).unwrap_or(&true) {
                    state.field[row][col] = EMPTY;
                    state.field[row + 1][col] = tile;

                    println!("gravity: {}", state);
                    thread::sleep(std::time::Duration::from_millis(100));
                }
            }
        }
    }
}

fn find_floating(state: &State, floating_ids: &mut HashMap<u16, bool>) {
    // TODO: check if reversed is better
    for row in 0..FIELD_HEIGHT as usize {
        for col in 0..FIELD_WIDTH as usize {
            if state.field[row][col] == EMPTY {
                continue;
            }

            let composite_id = state.field[row][col];

            let piece_id = id_manager::get_piece_id(composite_id);

            // if tile is in bottom row
            if row == FIELD_HEIGHT as usize - 1 {
                floating_ids.insert(composite_id, false);
                continue;
            }

            let below = state.field[row + 1][col];

            if below != EMPTY && below != composite_id {
                floating_ids.insert(composite_id, false);
                continue;
            }
        }
    }
}

fn is_connected(state: &State, row: usize, col: usize, piece_id: &u8) -> bool {
    // check the neighbors' piece_id
    let deltas = vec![(-1, 0), (1, 0), (0, -1), (0, 1), (1, 1), (-1, -1)];

    for (delta_row, delta_col) in deltas {
        let tile_row = row as i32 + delta_row;
        let tile_col = col as i32 + delta_col;

        if tile_row < 0
            || tile_row >= state.field.len() as i32
            || tile_col < 0
            || tile_col >= state.field[0].len() as i32
        {
            continue;
        }

        let tile = state.field[tile_row as usize][tile_col as usize];

        if tile != EMPTY && id_manager::get_piece_id(tile) == *piece_id {
            return true;
        }
    }

    false
}

pub fn char_to_id(c: char) -> u8 {
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
        _ => 255,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut id_manager = IdManager::new();
        let mut state = State::initial_state(&vec!['X']);

        let comp_id1 = id_manager::create_composite_id(&9, &0);
        let comp_id2 = id_manager::create_composite_id(&11, &1);

        let field = vec![
            vec![
                state::EMPTY,
                state::EMPTY,
                state::EMPTY,
                state::EMPTY,
                state::EMPTY,
            ],
            vec![
                state::EMPTY,
                state::EMPTY,
                state::EMPTY,
                state::EMPTY,
                state::EMPTY,
            ],
            vec![
                state::EMPTY,
                state::EMPTY,
                state::EMPTY,
                state::EMPTY,
                state::EMPTY,
            ],
            vec![
                state::EMPTY,
                state::EMPTY,
                state::EMPTY,
                state::EMPTY,
                state::EMPTY,
            ],
            vec![
                state::EMPTY,
                state::EMPTY,
                state::EMPTY,
                state::EMPTY,
                state::EMPTY,
            ],
            vec![
                state::EMPTY,
                state::EMPTY,
                state::EMPTY,
                state::EMPTY,
                state::EMPTY,
            ],
            vec![
                state::EMPTY,
                state::EMPTY,
                state::EMPTY,
                state::EMPTY,
                state::EMPTY,
            ],
            vec![
                state::EMPTY,
                state::EMPTY,
                state::EMPTY,
                state::EMPTY,
                state::EMPTY,
            ],
            vec![
                state::EMPTY,
                state::EMPTY,
                state::EMPTY,
                state::EMPTY,
                state::EMPTY,
            ],
            vec![
                state::EMPTY,
                state::EMPTY,
                state::EMPTY,
                state::EMPTY,
                state::EMPTY,
            ],
            vec![
                state::EMPTY,
                state::EMPTY,
                state::EMPTY,
                state::EMPTY,
                state::EMPTY,
            ],
            vec![
                state::EMPTY,
                state::EMPTY,
                state::EMPTY,
                state::EMPTY,
                state::EMPTY,
            ],
            vec![
                comp_id1,
                state::EMPTY,
                state::EMPTY,
                state::EMPTY,
                state::EMPTY,
            ],
            vec![comp_id1, comp_id2, comp_id2, comp_id2, comp_id2],
            vec![
                comp_id1,
                state::EMPTY,
                state::EMPTY,
                state::EMPTY,
                state::EMPTY,
            ],
        ];

        state.field = field;
        println!("b4 clear {}", state);
        println!("P piece_id: {}", id_manager::get_piece_id(comp_id1));

        clear_full_rows(&mut state, true);
        println!("after clear {}", state);

        gravity(&mut state, &mut id_manager);

        println!("after gravity {}", state);

        println!("P piece_id: {}", id_manager::get_piece_id(comp_id1));

        // assert_eq!(state.field[13], vec![EMPTY; FIELD_WIDTH as usize]);
        // assert_eq!(state.field[12][0], EMPTY);
        assert_eq!(
            is_connected(&state, 12, 0, &id_manager::get_piece_id(comp_id1)),
            false
        );
    }
}
