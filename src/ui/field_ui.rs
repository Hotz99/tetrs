use macroquad::prelude::*;

const SCALE: f32 = 40.0;

pub fn draw() {
    clear_background(BLACK);

    for x in 0..5 {
        for y in 0..15 {
            draw_rectangle(
                x as f32 * SCALE,
                y as f32 * SCALE,
                SCALE - 1.0,
                SCALE - 1.0,
                WHITE,
            );
        }
    }
}
