pub mod id_manager;
pub mod next_shapes;
pub mod state;

use crate::game::{state::Field, state::State};
use id_manager::IdManager;
use std::collections::{HashSet, VecDeque};

pub const FIELD_WIDTH: usize = 5;
pub const FIELD_HEIGHT: usize = 15;
pub const EMPTY: u16 = 13;
pub const STACK_SIZE: usize = 5;

// clears full rows and applies gravity, recursively
pub fn update(
    state: &mut State,
    id_manager: &mut IdManager,
    mut cleared_count: u32,
    mut clear_rows: bool,
) -> u32 {
    if !clear_rows {
        return cleared_count;
    }

    clear_rows = false;

    // for each row, we either clear it or update the composite_id of its tiles
    for row in (0..FIELD_HEIGHT).rev() {
        // all() is short-circuiting
        // if row is full
        if state.field[row].iter().all(|&x| x != EMPTY) {
            // clear row
            for col in 0..FIELD_WIDTH {
                state.field[row][col] = EMPTY;
            }

            cleared_count += 1;
            state.cleared_rows += 1;

            clear_rows = true;

            continue;
        }

        // update composite_id of separated tiles
        for col in 0..FIELD_WIDTH {
            let tile = state.field[row][col];

            if tile == EMPTY || is_connected(state, row as u8, col as u8, &get_unique_id(tile)) {
                continue;
            }

            let pent_id = get_pent_id(tile);

            state.field[row][col] =
                create_composite_id(pent_id, id_manager.next_unique_id(pent_id));
        }
    }

    gravity(state);

    update(state, id_manager, cleared_count, clear_rows);

    cleared_count
}

// animated version of update()
// returns a vector of Fields to be drawn, as animation frames
pub fn animate_update(
    state: &mut State,
    id_manager: &mut IdManager,
    mut cleared_count: u32,
    mut clear_rows: bool,
    frames: &mut VecDeque<Field>,
) -> u32 {
    if !clear_rows {
        return cleared_count;
    }

    // initial frame
    if cleared_count == 0 {
        frames.push_back(state.field.clone());
    }

    clear_rows = false;

    // for each row, we either clear it or update the composite_id of its tiles
    for row in (0..FIELD_HEIGHT).rev() {
        // all() is short-circuiting
        // if row is full
        if state.field[row].iter().all(|&x| x != EMPTY) {
            // clear row
            for col in 0..FIELD_WIDTH {
                state.field[row][col] = EMPTY;
            }

            cleared_count += 1;
            state.cleared_rows += 1;

            clear_rows = true;

            frames.push_back(state.field.clone());

            continue;
        }

        // update composite_id of separated tiles
        for col in 0..FIELD_WIDTH {
            let tile = state.field[row][col];

            if tile == EMPTY || is_connected(state, row as u8, col as u8, &get_unique_id(tile)) {
                continue;
            }

            let pent_id = get_pent_id(tile);

            state.field[row][col] =
                create_composite_id(pent_id, id_manager.next_unique_id(pent_id));
        }
    }

    frames.push_back(state.field.clone());

    gravity(state);

    frames.push_back(state.field.clone());

    animate_update(state, id_manager, cleared_count, clear_rows, frames);

    cleared_count
}

fn gravity(state: &mut State) {
    // if one tile is settled, so will the rest of the tiles that make up the piece
    // where a tile is an entry in a 2d vec (game field),
    // tiles of the same piece have the same composite_id

    let mut settled_ids: HashSet<u16> = HashSet::new();
    let mut possible_shifts: Vec<(usize, usize)> = Vec::new();

    loop {
        settled_ids.clear();
        possible_shifts.clear();

        for row in (0..FIELD_HEIGHT).rev() {
            for col in 0..FIELD_WIDTH {
                let tile = state.field[row][col];

                if tile == EMPTY {
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

        for &(row, col) in &possible_shifts {
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

// checks if tile is connected to other tiles of the same piece
fn is_connected(state: &State, row: u8, col: u8, unique_id: &u16) -> bool {
    // neighbor offsets
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

        let neighbor = state.field[tile_row as usize][tile_col as usize];

        if neighbor != EMPTY && get_unique_id(neighbor) == *unique_id {
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

// composite_id (16 bits) = pent_id (4 bits) + unique_id (12 bits)
pub fn create_composite_id(pent_id: u8, unique_id: u16) -> u16 {
    ((pent_id as u16) << 12) | (unique_id & 0x0FFF) // extract 12 bits
}

pub fn get_pent_id(composite_id: u16) -> u8 {
    (composite_id >> 12) as u8
}

pub fn get_unique_id(composite_id: u16) -> u16 {
    composite_id & 0x0FFF
}

#[cfg(test)]
mod tests {
    use crate::game::{self, state::State};

    use tests::id_manager::IdManager;

    use super::*;

    #[test]
    fn test_gravity() {
        let mut state = State::initialize();
        state.remaining_pieces = vec!['X'];

        let mut id_manager = IdManager::new();

        let l_composite_id = create_composite_id(8, id_manager.next_unique_id(8));

        let p_composite_id = create_composite_id(9, id_manager.next_unique_id(9));

        let x_composite_id = create_composite_id(0, id_manager.next_unique_id(0));

        let field1 = vec![
            vec![
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
            ],
            vec![
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
            ],
            vec![
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
            ],
            vec![
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
            ],
            vec![
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
            ],
            vec![
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
            ],
            vec![
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
            ],
            vec![
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
            ],
            vec![
                p_composite_id,
                p_composite_id,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
            ],
            vec![
                p_composite_id,
                p_composite_id,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
            ],
            vec![
                p_composite_id,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
            ],
            vec![
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
            ],
            vec![
                game::EMPTY,
                l_composite_id,
                l_composite_id,
                l_composite_id,
                l_composite_id,
            ],
            vec![
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
            ],
            vec![
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
            ],
        ];

        let field2 = vec![
            vec![
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
            ],
            vec![
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
            ],
            vec![
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
            ],
            vec![
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
            ],
            vec![
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
            ],
            vec![
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
            ],
            vec![
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
            ],
            vec![
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
            ],
            vec![
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
            ],
            vec![
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
            ],
            vec![
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
            ],
            vec![
                x_composite_id,
                x_composite_id,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
            ],
            vec![
                game::EMPTY,
                x_composite_id,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
            ],
            vec![
                l_composite_id,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
            ],
            vec![
                l_composite_id,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
            ],
        ];

        let field3 = vec![
            vec![
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
            ],
            vec![
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
            ],
            vec![
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
            ],
            vec![
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
            ],
            vec![
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
            ],
            vec![
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
            ],
            vec![
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
            ],
            vec![
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
            ],
            vec![
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
            ],
            vec![
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
            ],
            vec![
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                l_composite_id,
            ],
            vec![
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                l_composite_id,
            ],
            vec![
                x_composite_id,
                x_composite_id,
                x_composite_id,
                x_composite_id,
                l_composite_id,
            ],
            vec![
                game::EMPTY,
                x_composite_id,
                game::EMPTY,
                game::EMPTY,
                l_composite_id,
            ],
            vec![
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                l_composite_id,
            ],
        ];

        state.field = field3;

        println!("b4 clear+gravity\n {}", state);

        update(&mut state, &mut id_manager, 0, true);

        println!("after clear+gravity\n {}", state);

        assert_eq!(state.field[2][0], EMPTY);
    }

    #[test]
    fn test_update() {
        let mut state = State::initialize();
        state.remaining_pieces = vec!['X'];

        let comp_id1 = create_composite_id(9, 0);
        let comp_id2 = create_composite_id(11, 1);

        let field = vec![
            vec![
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
            ],
            vec![
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
            ],
            vec![
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
            ],
            vec![
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
            ],
            vec![
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
            ],
            vec![
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
            ],
            vec![
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
            ],
            vec![
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
            ],
            vec![
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
            ],
            vec![
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
            ],
            vec![
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
            ],
            vec![
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
                game::EMPTY,
            ],
            vec![comp_id1, game::EMPTY, game::EMPTY, game::EMPTY, game::EMPTY],
            vec![comp_id1, comp_id2, comp_id2, comp_id2, comp_id2],
            vec![comp_id1, game::EMPTY, game::EMPTY, game::EMPTY, game::EMPTY],
        ];

        state.field = field;

        println!("b4 clear\n {}", state);
        println!("P unique_id: {}", get_unique_id(comp_id1));

        update(&mut state, &mut IdManager::new(), 0, true);
        println!("after clear\n {}", state);

        // assert_eq!(state.field[13], vec![EMPTY; FIELD_WIDTH as usize]);
        // assert_eq!(state.field[12][0], EMPTY);
        assert_eq!(is_connected(&state, 12, 0, &get_unique_id(comp_id1)), false);
    }

    #[test]
    fn test_create_composite_id() {
        for x in 0..12 {
            for y in 0..4096 {
                let composite_id = create_composite_id(x, y);
                assert_eq!(get_pent_id(composite_id), x);
                assert_eq!(get_unique_id(composite_id), y);
            }
        }
    }

    #[test]
    fn test_get_unique_id() {
        let composite_id = create_composite_id(13, 2);

        assert_eq!(get_pent_id(composite_id), 13);
        assert_eq!(get_unique_id(composite_id), 2);
    }
}
