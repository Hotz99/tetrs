#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

use std::{
    cell::RefCell,
    rc::Rc,
    thread,
    time::{Duration, Instant},
};

mod app;
mod data;
mod logic;
mod ui;

use app::App;
use logic::{
    bot, game, id_manager, next_shapes,
    state::{self, State},
};

use ui::*;

use eframe::egui;

use egui::{Color32, Pos2, Stroke, Vec2};

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(Vec2::new(
            state::FIELD_WIDTH as f32 * ui::SCALE,
            state::FIELD_HEIGHT as f32 * ui::SCALE,
        )),
        ..Default::default()
    };

    eframe::run_native("Tetrs", options, Box::new(|cc| Box::new(App::new())));

    // let mut app = App::new();

    // app.test_bot();
}
