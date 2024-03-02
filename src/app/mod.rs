use std::{
    collections::VecDeque,
    thread,
    time::{Duration, Instant},
};

use crate::{
    data::pentomino_db::PentominoDB,
    logic::{
        bot, game,
        id_manager::IdManager,
        next_shapes::NextShapes,
        state::{Field, GameState},
    },
    ui,
};

pub struct App {
    game_state: GameState,
    id_manager: IdManager,
    lookahead: NextShapes,
    pent_db: PentominoDB,
    delay_ms: u16,
    frames: VecDeque<Field>,
    current_frame: Option<Field>,
    last_frame_time: Instant,
    solution_times: VecDeque<Duration>,
    avg_solution_time: Duration,
    is_bot_paused: bool,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.frames.is_empty() {
            let start_time = Instant::now();

            self.bot_search();

            game::animate_update(
                &mut self.game_state,
                &mut self.id_manager,
                0,
                true,
                &mut self.frames,
            );

            // TODO: remove duplicates for VecDeque
            // self.frames.dedup_by(|x, y| x == y);

            self.current_frame = self.frames.pop_front();
            self.last_frame_time = Instant::now();
            self.solution_times.push_back(Instant::now() - start_time);

            if self.solution_times.len() > 10 {
                self.solution_times.pop_front();
            }
        }

        let now = Instant::now();

        // if delay_ms time has passed, update current_frame
        if now.duration_since(self.last_frame_time).as_millis() as u16 >= self.delay_ms {
            self.last_frame_time = now;

            if !self.is_bot_paused {
                self.current_frame = self.frames.pop_front();
                self.avg_solution_time =
                    self.solution_times.iter().sum::<Duration>() / self.solution_times.len() as u32;
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                // left side
                ui::draw_game_field(ui, self.current_frame.as_ref().unwrap());

                // right side
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Delay (ms): ");
                        ui.add(egui::Slider::new(&mut self.delay_ms, 0..=1000).logarithmic(true));
                    });

                    ui.add_space(20.0);

                    ui.label(format!("Cleared rows:  {}", self.game_state.cleared_rows));

                    ui.add_space(20.0);

                    ui.label(format!(
                        "Avg solution time:  {:.3} ms",
                        self.avg_solution_time.as_secs_f64() * 1000.0
                    ));

                    ui.add_space(20.0);

                    if ui.button("Pause | Continue").clicked() {
                        self.is_bot_paused = !self.is_bot_paused;
                    }
                });
            });
        });

        ctx.request_repaint();
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            game_state: GameState::initial_game_state(),
            id_manager: IdManager::new(),
            lookahead: NextShapes::new(),
            pent_db: PentominoDB::new(),
            delay_ms: 300,
            frames: VecDeque::new(),
            current_frame: Some(Field::new()),
            last_frame_time: Instant::now(),
            solution_times: VecDeque::new(),
            avg_solution_time: Duration::new(0, 0),
            is_bot_paused: false,
        }
    }

    fn bot_search(&mut self) {
        self.game_state.remaining_pieces = self.lookahead.get_next_stack();

        match bot::search(self.game_state.clone(), &self.pent_db, &mut self.id_manager) {
            Some(solution) => {
                self.game_state = solution;
            }
            None => {
                println!("NO SOLUTION");
            }
        };
    }

    pub fn test_bot(&mut self) {
        let runs = 10;
        let searches = 1000;
        let mut total_run_time = Duration::new(0, 0);
        let mut total_solution_time = Duration::new(0, 0);
        let mut total_score = 0;
        let mut failed_counter = 0;

        for i in 0..runs {
            let mut state = GameState::initial_game_state();

            let mut run_time = Duration::new(0, 0);
            let run_start = Instant::now();

            for j in 0..searches {
                state.remaining_pieces = self.lookahead.get_next_stack();

                let solution_start = Instant::now();

                match bot::search(state, &self.pent_db, &mut self.id_manager) {
                    Some(solution) => {
                        let solution_end = Instant::now();

                        total_solution_time += solution_end - solution_start;

                        state = solution;
                        total_score += 1;
                    }
                    None => {
                        failed_counter += 1;
                        break;
                    }
                };

                game::update(&mut state, &mut self.id_manager, 0, true);
            }

            let run_end = Instant::now();

            run_time += run_end - run_start;
            total_run_time += run_time;

            println!("run {}: {:?}", (i + 1), run_time);
        }
        println!("\nfailed runs: {:?}", failed_counter);
        println!("\navg run time: {:?}", total_run_time / runs);
        println!(
            "avg solution time: {:?}",
            total_solution_time / (runs * searches)
        );
        println!("avg score: {:?}", total_score / runs);
    }
}
