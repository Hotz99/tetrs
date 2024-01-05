use macroquad::prelude::*;

use crate::logic::{
    id_manager::{self},
    state,
};

const SCALE: f32 = 40.0;

pub fn draw_field(field: &Vec<Vec<u16>>) {
    clear_background(BLACK);

    for row in 0..field.len() {
        for col in 0..field[0].len() {
            if field[row][col] != state::EMPTY {
                draw_rectangle(
                    col as f32 * SCALE,
                    row as f32 * SCALE,
                    SCALE - 1.0,
                    SCALE - 1.0,
                    get_color(id_manager::get_pent_id(field[row][col])),
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
