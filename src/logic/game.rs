use std::f32::consts::E;

use super::{
    id_manager::{self, IdManager},
    state::*,
};

pub fn update(state: &mut State) {
    // line clearing
    let mut new_field = vec![vec![EMPTY; FIELD_WIDTH as usize]; FIELD_HEIGHT as usize];

    for row in state.field.iter().rev() {
        if row.iter().all(|&cell| cell != EMPTY) {
            state.cleared_rows += 1;
        } else {
            new_field.pop();
            new_field.insert(0, row.clone());
        }
    }

    state.field = new_field;

    // gravity

    // bool for each piece id
    let mut floating = [true; 256];

    // find floating pieces
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

    // shift floating pieces down by 1 row
    for row in (0..FIELD_HEIGHT).rev() {
        for col in (0..FIELD_WIDTH).rev() {
            let tile = state.field[row as usize][col as usize];
            if tile != EMPTY {
                let piece_id = id_manager::get_piece_id(tile);
                if floating[piece_id as usize] && row < FIELD_HEIGHT - 1 {
                    state.field[row as usize][col as usize] = EMPTY;
                    state.field[(row + 1) as usize][col as usize] = tile;
                }
            }
        }
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
