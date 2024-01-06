use super::{
    id_manager::{self, IdManager},
    state::{self, *},
};

pub fn clear_full_rows(state: &mut State, clear: &bool) {
    if !*clear {
        return;
    };

    for i in (0..FIELD_HEIGHT as usize).rev() {
        let row = &state.field[i];

        if row.iter().all(|&x| x != EMPTY) {
            state.field.remove(i);
            state.field.insert(0, vec![EMPTY; FIELD_WIDTH as usize]);
            state.cleared_rows += 1;
            clear_full_rows(state, &true)
        } else {
            clear_full_rows(state, &false)
        }
    }
}

pub fn gravity(state: &mut State, fall: &bool, id_manager: &mut IdManager) {
    if !*fall {
        return;
    };

    // bool for each piece id
    let mut floating = vec![true; 256];

    find_floating(state, &mut floating);

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

                    // likely not needed here, basic logic may be enough
                    find_floating(state, &mut floating);
                }
            }
        }
    }

    // shift floating pieces down by 1 row
    for row in (0..FIELD_HEIGHT as usize).rev() {
        for col in 0..FIELD_WIDTH as usize {
            let tile = state.field[row][col];
            if tile != EMPTY {
                let piece_id = id_manager::get_piece_id(tile);
                if floating[piece_id as usize] {
                    state.field[row][col] = EMPTY;
                    state.field[row + 1][col] = tile;

                    gravity(state, &true, id_manager)
                }
            }
        }
    }
}

fn find_floating(state: &State, floating: &mut Vec<bool>) {
    for row in 0..FIELD_HEIGHT as usize {
        for col in 0..FIELD_WIDTH as usize {
            let tile = state.field[row][col];

            if tile == EMPTY {
                continue;
            }

            let piece_id = id_manager::get_piece_id(tile);

            if row == FIELD_HEIGHT as usize - 1 {
                floating[piece_id as usize] = false;
            } else {
                let below = state.field[row + 1][col];

                if below != EMPTY && below != tile {
                    floating[piece_id as usize] = false;
                }
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

        clear_full_rows(&mut state, &true);
        println!("after clear {}", state);

        gravity(&mut state, &true, &mut id_manager);

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
