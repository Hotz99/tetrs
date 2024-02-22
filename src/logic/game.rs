use std::{
    collections::{HashMap, HashSet},
    thread,
    time::Duration,
};

use macroquad::window::next_frame;

use crate::ui;

use super::{
    id_manager,
    state::{self, *},
};

pub fn animate_clear_rows(
    state: &mut State,
    mut cleared_rows: u32,
    mut clear: bool,
    delay_ms: u64,
) -> u32 {
    if !clear {
        return cleared_rows;
    }

    // initial draw
    ui::draw_field(&state.field);
    next_frame();
    thread::sleep(Duration::from_millis(delay_ms));

    clear = false;

    // for each row, we either clear it or update the composite_id of its tiles
    for row in (0..FIELD_HEIGHT).rev() {
        // all() is short-circuiting
        // if row is full
        if (&state.field[row as usize]).iter().all(|&x| x != EMPTY) {
            for col in 0..FIELD_WIDTH {
                state.field[row as usize][col as usize] = EMPTY;
            }

            ui::draw_field(&state.field);
            next_frame();
            thread::sleep(Duration::from_millis(delay_ms * 2));

            cleared_rows += 1;
            clear = true;
        }

        state.cleared_rows += cleared_rows;

        // update composite_id of separated tiles
        for col in 0..FIELD_WIDTH {
            let tile = state.field[row as usize][col as usize];

            if tile == EMPTY
                || is_connected(
                    state,
                    row as u8,
                    col as u8,
                    &id_manager::get_unique_id(tile),
                )
            {
                continue;
            }

            let pent_id = id_manager::get_pent_id(tile);

            state.field[row as usize][col as usize] = id_manager::create_composite_id(
                pent_id,
                id_manager::next_unique_id(&mut state.used_ids, pent_id),
            );
        }
    }

    animate_gravity(state, delay_ms);

    animate_clear_rows(state, cleared_rows, clear, delay_ms);

    cleared_rows
}

fn animate_gravity(state: &mut State, delay_ms: u64) {
    loop {
        // if one tile is settled, so will the rest of the tiles that make up the piece (are copies of the same composite_id)
        let mut settled_ids: HashSet<u16> = HashSet::new();
        let mut possible_shifts: Vec<(usize, usize)> = Vec::new();

        settled_ids.clear();

        // traverse bottom-up
        for row in (0..(FIELD_HEIGHT)).rev() {
            for col in 0..FIELD_WIDTH {
                let tile = state.field[row][col];

                if tile == EMPTY || settled_ids.contains(&tile) {
                    continue;
                }

                if row == (FIELD_HEIGHT - 1) {
                    settled_ids.insert(tile);
                    continue;
                }

                let below = state.field[row + 1][col];

                if below != EMPTY && below != tile {
                    settled_ids.insert(tile);
                    continue;
                }

                possible_shifts.push((row, col));
            }
        }

        let mut shifted = false;

        for (row, col) in possible_shifts {
            let tile = state.field[row][col];

            if settled_ids.contains(&tile) {
                continue;
            }

            state.field[row][col] = EMPTY;
            state.field[row + 1][col] = tile;

            shifted = true;
        }

        if !shifted {
            return;
        }
    }
}

// todo: clear all full rows at once, instead of 'clear one then gravity, clear one then gravity'
// recursively clear rows
pub fn simulate_clear_rows(state: &mut State, mut cleared_rows: u32, mut clear: bool) -> u32 {
    if !clear {
        return cleared_rows;
    }

    clear = false;

    // for each row, we either clear it or update the composite_id of its tiles
    for row in (0..FIELD_HEIGHT).rev() {
        // all() is short-circuiting
        // if row is full
        if (&state.field[row as usize]).iter().all(|&x| x != EMPTY) {
            for col in 0..FIELD_WIDTH {
                state.field[row as usize][col as usize] = EMPTY;
            }

            cleared_rows += 1;
            clear = true;
        }

        state.cleared_rows += cleared_rows;

        // update composite_id of separated tiles
        for col in 0..FIELD_WIDTH {
            let tile = state.field[row as usize][col as usize];

            if tile == EMPTY
                || is_connected(
                    state,
                    row as u8,
                    col as u8,
                    &id_manager::get_unique_id(tile),
                )
            {
                continue;
            }

            let pent_id = id_manager::get_pent_id(tile);

            state.field[row as usize][col as usize] = id_manager::create_composite_id(
                pent_id,
                id_manager::next_unique_id(&mut state.used_ids, pent_id),
            );
        }
    }

    gravity(state);

    simulate_clear_rows(state, cleared_rows, clear);

    cleared_rows
}

fn gravity(state: &mut State) {
    loop {
        // if one tile is settled, so will the rest of the tiles that make up the piece (are copies of the same composite_id)
        let mut settled_ids: HashSet<u16> = HashSet::new();
        let mut possible_shifts: Vec<(usize, usize)> = Vec::new();

        settled_ids.clear();

        // traverse bottom-up
        for row in (0..(FIELD_HEIGHT)).rev() {
            for col in 0..FIELD_WIDTH {
                let tile = state.field[row][col];

                if tile == EMPTY || settled_ids.contains(&tile) {
                    continue;
                }

                if row == (FIELD_HEIGHT - 1) {
                    settled_ids.insert(tile);
                    continue;
                }

                let below = state.field[row + 1][col];

                if below != EMPTY && below != tile {
                    settled_ids.insert(tile);
                    continue;
                }

                possible_shifts.push((row, col));
            }
        }

        let mut shifted = false;

        for (row, col) in possible_shifts {
            let tile = state.field[row][col];

            if settled_ids.contains(&tile) {
                continue;
            }

            state.field[row][col] = EMPTY;
            state.field[row + 1][col] = tile;

            shifted = true;
        }

        if !shifted {
            return;
        }
    }
}

fn is_connected(state: &State, row: u8, col: u8, unique_id: &u16) -> bool {
    // check the neighbors' unique_id
    let deltas = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];

    for (delta_row, delta_col) in deltas {
        let tile_row = row as i8 + delta_row;
        let tile_col = col as i8 + delta_col;

        if tile_row < 0
            || tile_row >= state.field.len() as i8
            || tile_col < 0
            || tile_col >= state.field[0].len() as i8
        {
            continue;
        }

        let tile = state.field[tile_row as usize][tile_col as usize];

        if tile != EMPTY && id_manager::get_unique_id(tile) == *unique_id {
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
    fn test_gravity() {
        let mut state = State::initial_state(&vec!['X']);

        let l_composite_id =
            id_manager::create_composite_id(8, id_manager::next_unique_id(&mut state.used_ids, 8));

        let p_composite_id =
            id_manager::create_composite_id(9, id_manager::next_unique_id(&mut state.used_ids, 9));

        let x_composite_id =
            id_manager::create_composite_id(0, id_manager::next_unique_id(&mut state.used_ids, 0));

        let field1 = vec![
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
                p_composite_id,
                p_composite_id,
                state::EMPTY,
                state::EMPTY,
                state::EMPTY,
            ],
            vec![
                p_composite_id,
                p_composite_id,
                state::EMPTY,
                state::EMPTY,
                state::EMPTY,
            ],
            vec![
                p_composite_id,
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
                l_composite_id,
                l_composite_id,
                l_composite_id,
                l_composite_id,
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
        ];

        let field2 = vec![
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
                x_composite_id,
                x_composite_id,
                state::EMPTY,
                state::EMPTY,
                state::EMPTY,
            ],
            vec![
                state::EMPTY,
                x_composite_id,
                state::EMPTY,
                state::EMPTY,
                state::EMPTY,
            ],
            vec![
                l_composite_id,
                state::EMPTY,
                state::EMPTY,
                state::EMPTY,
                state::EMPTY,
            ],
            vec![
                l_composite_id,
                state::EMPTY,
                state::EMPTY,
                state::EMPTY,
                state::EMPTY,
            ],
        ];

        state.field = field2;

        println!("b4 gravity\n {}", state);

        gravity(&mut state);
        println!("after gravity\n {}", state);

        assert_eq!(state.field[2][0], EMPTY);
    }

    #[test]
    fn test_simulate_clear_rows() {
        let mut state = State::initial_state(&vec!['X']);

        let comp_id1 = id_manager::create_composite_id(9, 0);
        let comp_id2 = id_manager::create_composite_id(11, 1);

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
        println!("P unique_id: {}", id_manager::get_unique_id(comp_id1));

        simulate_clear_rows(&mut state, 0, true);
        println!("after clear {}", state);

        // assert_eq!(state.field[13], vec![EMPTY; FIELD_WIDTH as usize]);
        // assert_eq!(state.field[12][0], EMPTY);
        assert_eq!(
            is_connected(&state, 12, 0, &id_manager::get_unique_id(comp_id1)),
            false
        );
    }
}
