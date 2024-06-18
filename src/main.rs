mod app;
mod bot;
mod game;
mod pentominoes;
mod ui;

use app::App;

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(egui::Vec2::new(445.0, 615.0)),
        ..Default::default()
    };

    eframe::run_native("Tetrs", options, Box::new(|cc| Box::new(App::new())));

    // let mut app = App::new();

    // app.test_bot();
}
