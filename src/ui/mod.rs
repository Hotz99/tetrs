use egui::{Color32, Pos2, Stroke, Vec2};

use crate::logic::{
    game, id_manager,
    state::{self},
};

pub const SCALE: f32 = 40.0;

pub fn draw_game_field(ui: &mut egui::Ui, field: &state::Field) {
    let (response, painter) = ui.allocate_painter(
        Vec2::new(
            state::FIELD_WIDTH as f32 * SCALE,
            state::FIELD_HEIGHT as f32 * SCALE,
        ),
        egui::Sense::hover(),
    );

    // draw area
    let rect = response.rect;

    // draw horizontal lines
    for col in 0..=state::FIELD_WIDTH {
        let x = rect.left() + col as f32 * SCALE;

        painter.line_segment(
            [Pos2::new(x, rect.top()), Pos2::new(x, rect.bottom())],
            Stroke::new(1.0, Color32::LIGHT_GRAY),
        );
    }

    // draw vertical lines
    for row in 0..=state::FIELD_HEIGHT {
        let y = rect.top() + row as f32 * SCALE;

        painter.line_segment(
            [Pos2::new(rect.left(), y), Pos2::new(rect.right(), y)],
            Stroke::new(1.0, Color32::LIGHT_GRAY),
        );
    }

    // draw pentominoes
    for row in 0..state::FIELD_HEIGHT {
        for col in 0..state::FIELD_WIDTH {
            let color = get_pent_id_color(game::get_pent_id(field[row][col]));
            let x = rect.left() + col as f32 * SCALE;
            let y = rect.top() + row as f32 * SCALE;

            painter.rect_filled(
                egui::Rect::from_min_max(
                    Pos2::new(x, y),
                    Pos2::new(x + SCALE - 1.0, y + SCALE - 1.0),
                ),
                0.0,
                color,
            );
        }
    }
}

fn get_pent_id_color(pent_id: u8) -> Color32 {
    match pent_id {
        0 => Color32::BLUE,
        1 => Color32::from_rgb(255, 165, 0),   // orange
        2 => Color32::from_rgb(135, 206, 235), // sky blue
        3 => Color32::GREEN,
        4 => Color32::from_rgb(255, 0, 255),   // magenta
        5 => Color32::from_rgb(255, 192, 203), // pink
        6 => Color32::RED,
        7 => Color32::YELLOW,
        8 => Color32::from_rgb(128, 0, 128), // purple
        9 => Color32::from_rgb(0, 0, 100),   // dark blue
        10 => Color32::from_rgb(100, 0, 0),  // dark red
        11 => Color32::from_rgb(0, 100, 0),  // dark green
        _ => Color32::LIGHT_GRAY,
    }
}
