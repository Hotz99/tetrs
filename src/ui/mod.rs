use macroquad::{
    color::*, math::vec2, shapes::draw_rectangle, text::draw_text, ui::widgets,
    window::clear_background,
};

use crate::logic::{id_manager, state};

const SCALE: f32 = 40.0;

pub fn draw_field(field: &state::Field) {
    clear_background(BLACK);

    for row in 0..field.len() {
        for col in 0..field[0].len() {
            if field[row][col] != state::EMPTY {
                let x = col as f32 * SCALE;
                let y = row as f32 * SCALE;

                draw_rectangle(
                    x,
                    y,
                    SCALE - 1.0,
                    SCALE - 1.0,
                    get_color(id_manager::get_pent_id(field[row][col])),
                );

                draw_text(
                    &field[row][col].to_string(),
                    x + SCALE / 14.0,
                    y + SCALE / 2.0,
                    SCALE / 2.0,
                    WHITE,
                );
            }
        }
    }
}

fn get_color(i: u8) -> Color {
    match i {
        0 => BLUE,
        1 => ORANGE,
        2 => SKYBLUE,
        3 => GREEN,
        4 => MAGENTA,
        5 => PINK,
        6 => RED,
        7 => YELLOW,
        8 => PURPLE,
        9 => Color::new(0.0, 0.0, 0.39, 1.0),  // Dark Blue
        10 => Color::new(0.39, 0.0, 0.0, 1.0), // Dark Red
        11 => Color::new(0.0, 0.39, 0.0, 1.0), // Dark Green
        _ => LIGHTGRAY,
    }
}
