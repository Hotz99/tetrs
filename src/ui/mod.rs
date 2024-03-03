use crate::game::{self, state::Field};

use egui::{Color32, Pos2, Stroke, Vec2};

pub const SCALE: f32 = 40.0;

pub fn draw_game_field(ui: &mut egui::Ui, field: &Field) {
    let (response, painter) = ui.allocate_painter(
        Vec2::new(
            game::FIELD_WIDTH as f32 * SCALE,
            game::FIELD_HEIGHT as f32 * SCALE,
        ),
        egui::Sense::hover(),
    );

    let draw_area = response.rect;

    painter.rect_stroke(draw_area, 0.0, Stroke::new(4.0, Color32::WHITE));

    // draw pentominoes
    for row in 0..game::FIELD_HEIGHT {
        for col in 0..game::FIELD_WIDTH {
            let tile = field[row][col];

            if tile == game::EMPTY {
                continue;
            }

            let color = get_pent_color(game::get_pent_id(tile));
            let spacing = 3.0;

            let x = draw_area.left() + col as f32 * SCALE;
            let y = draw_area.top() + row as f32 * SCALE;

            painter.rect_filled(
                egui::Rect::from_min_max(
                    Pos2::new(x + (spacing / 2.0), y + (spacing / 2.0)),
                    Pos2::new(x + SCALE - (spacing / 2.0), y + SCALE - (spacing / 2.0)),
                ),
                6.0,
                color,
            );
        }
    }
}

fn get_pent_color(i: u8) -> egui::Color32 {
    match i {
        0 => egui::Color32::from_rgb(0, 0, 255),      // bright blue
        1 => egui::Color32::from_rgb(255, 165, 0),    // bright orange
        2 => egui::Color32::from_rgb(0, 255, 255),    // bright cyan
        3 => egui::Color32::from_rgb(0, 255, 0),      // bright green
        4 => egui::Color32::from_rgb(255, 0, 255),    // bright magenta
        5 => egui::Color32::from_rgb(255, 105, 180),  // bright pink
        6 => egui::Color32::from_rgb(255, 0, 180),    // purple
        7 => egui::Color32::from_rgb(255, 255, 0),    // bright yellow
        8 => egui::Color32::from_rgb(127, 0, 255),    // bright purple
        9 => egui::Color32::from_rgb(0, 128, 255),    // bright dark blue
        10 => egui::Color32::from_rgb(255, 0, 0),     // bright red
        11 => egui::Color32::from_rgb(128, 255, 128), // light green
        _ => egui::Color32::from_rgb(211, 211, 211),  // light gray
    }
}
