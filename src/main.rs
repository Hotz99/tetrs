mod app;
mod bot;
mod game;
mod pentominoes;
mod ui;

const DEFAULT_N_RUNS: u32 = 100;
const DEFAULT_N_SEARCHES: u32 = 100;
const DEFAULT_LOOKAHEAD_SIZE: u8 = 5;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 && args[1] == "--perf" {
        let n_runs = match args.get(2) {
            Some(n) => n.parse().unwrap(),
            None => DEFAULT_N_RUNS,
        };

        let n_searches = match args.get(3) {
            Some(n) => n.parse().unwrap(),
            None => DEFAULT_N_SEARCHES,
        };

        let lookahead_size = match args.get(4) {
            Some(n) => n.parse().unwrap(),
            None => DEFAULT_LOOKAHEAD_SIZE,
        };

        let mut app = app::App::new(lookahead_size);

        app.perf_test_bot(n_runs, n_searches, lookahead_size);

        println!("\nn_runs: {}", n_runs);
        println!("n_searches: {}", n_searches);
        println!("lookahead_size: {}", lookahead_size);
    } else {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size(egui::Vec2::new(445.0, 615.0)),
            ..Default::default()
        };

        println!("running with lookahead size: {}", DEFAULT_LOOKAHEAD_SIZE);

        let _ = eframe::run_native(
            "Tetrs",
            options,
            Box::new(|_creation_ctx| Box::new(app::App::new(DEFAULT_LOOKAHEAD_SIZE))),
        );
    }
}
