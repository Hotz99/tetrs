use super::state::*;

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
    for y in (1..FIELD_HEIGHT as usize).rev() {
        for x in 0..FIELD_WIDTH as usize {
            if state.field[y][x] == EMPTY && state.field[y - 1][x] != EMPTY {
                state.field[y][x] = state.field[y - 1][x];
                state.field[y - 1][x] = EMPTY;
            } else if state.field[y][x] != EMPTY && state.field[y - 1][x] != EMPTY {
                return;
            }
        }
    }
}
