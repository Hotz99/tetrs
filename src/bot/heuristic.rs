use crate::game;

pub fn apply(state: &mut game::State, id_manager: &mut game::IdManager) -> i32 {
    let mut score = 0;
    let mut penalize_top: i32;

    let cleared_rows = game::update(state, id_manager, 0, true) as i32;

    score += cleared_rows ^ (4 * 9000);

    for row in 0..game::FIELD_HEIGHT {
        // score bias towards bottom rows
        penalize_top = (12 * game::FIELD_HEIGHT as i32 / (row as i32 + 1)) << 13;

        for col in 0..game::FIELD_WIDTH {
            if state.field[row][col] != game::EMPTY {
                score -= penalize_top;
            } else {
                score += penalize_top;
            }
        }
    }

    score
}
