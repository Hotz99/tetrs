use std::{
    collections::{HashSet, VecDeque},
    time::{Duration, Instant},
};

use crate::{bot, game, pentominoes, ui};

const DEFAULT_DELAY_MS: u16 = 350;
const EMA_ALPHA: f64 = 0.5;

pub struct App {
    pub game_state: game::State,
    lookahead_size: u8,
    pentomino_permutations: pentominoes::Permutations,
    id_manager: game::IdManager,
    next_up: game::NextShapes,
    last_frame_instance: Option<Instant>,
    pub delay_ms: u16,
    current_frame: Option<game::GameField>,
    frame_buffer: VecDeque<game::GameField>,
    ema_solution_time: Option<Duration>,
    pub is_bot_paused: bool,
}

impl App {
    pub fn new(lookahead_size: u8) -> Self {
        Self {
            game_state: game::State::new(lookahead_size),
            lookahead_size,
            pentomino_permutations: pentominoes::load_permutations(),
            id_manager: game::IdManager::default(),
            next_up: game::NextShapes::new(lookahead_size),
            last_frame_instance: None,
            delay_ms: DEFAULT_DELAY_MS,
            current_frame: None,
            frame_buffer: VecDeque::default(),
            ema_solution_time: None,
            is_bot_paused: false,
        }
    }

    fn bot_search(&mut self) -> Option<game::GameField> {
        self.game_state.remaining_pieces = self.next_up.get_next_stack();

        match bot::search(
            self.game_state.clone(),
            &self.pentomino_permutations,
            &mut self.id_manager,
            &self.lookahead_size,
        ) {
            Some(solution) => {
                let solution_field = solution.field.clone();
                self.game_state = solution;
                return Some(solution_field);
            }
            None => {
                println!("NO SOLUTION");
                return None;
            }
        };
    }

    pub fn perf_test_bot(&mut self, n_runs: u32, n_searches: u32, lookahead_size: u8) {
        let mut total_run_time = Duration::new(0, 0);
        let mut total_solution_time = Duration::new(0, 0);
        let mut failed_counter = 0;

        for i in 0..n_runs {
            let mut state = game::State::new(lookahead_size);

            let mut run_time = Duration::new(0, 0);
            let run_start = Instant::now();

            let mut failed = false;
            for _ in 0..n_searches {
                state.remaining_pieces = self.next_up.get_next_stack();

                let solution_start = Instant::now();

                match bot::search(
                    state,
                    &self.pentomino_permutations,
                    &mut self.id_manager,
                    &lookahead_size,
                ) {
                    Some(solution) => {
                        let solution_end = Instant::now();

                        total_solution_time += solution_end - solution_start;

                        state = solution;
                    }
                    None => {
                        failed_counter += 1;
                        failed = true;
                        break;
                    }
                };

                game::update(&mut state, &mut self.id_manager, 0, true);
            }

            let run_end = Instant::now();

            run_time += run_end - run_start;
            total_run_time += run_time;

            if failed {
                println!("run {}: failed", i + 1);
                continue;
            }

            println!("run {}: {:?}", (i + 1), run_time);
        }

        let total_solutions = (n_runs * n_searches) - failed_counter;

        println!("\ntotal solutions count: {:?}", total_solutions);
        println!(
            "avg solution time: {:?}",
            total_solution_time / total_solutions
        );
        println!(
            "solutions per second: {:.2}",
            total_solutions as f64 / total_run_time.as_secs_f64()
        );

        println!("\navg run time: {:?}", total_run_time / n_runs);
        println!("failed runs count: {:?}", failed_counter);
    }
}

impl eframe::App for App {
    // absolute mess, but i could not manage to make it cleaner
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // `frame_buffer` is empty when game starts or all frames have been rendered
        if self.frame_buffer.is_empty() {
            let start_time = Instant::now();
            self.bot_search();
            let new_solution_time = Some(Instant::now().duration_since(start_time));

            // felt like some smoothing of solution time was needed, hence employing
            // exponential moving average
            self.ema_solution_time = Some(Duration::from_secs_f64(
                new_solution_time.unwrap_or_default().as_secs_f64() * EMA_ALPHA
                    + self.ema_solution_time.unwrap_or_default().as_secs_f64() * (1.0 - EMA_ALPHA),
            ));

            game::animate_update(
                &mut self.game_state.field,
                &mut self.id_manager,
                // flag to control recursion
                true,
                // start with zero cleared rows for this specific update
                0,
                // buffer to update with cleared rows count
                &mut self.game_state.cleared_rows,
                // buffer to add frames to
                &mut self.frame_buffer,
            );

            // remove duplicate frames, seems expensive
            // but we dont care about performance in the animated version
            let mut seen = HashSet::new();
            self.frame_buffer.retain(|e| seen.insert(e.clone()));

            self.last_frame_instance = Some(Instant::now());
            self.current_frame = self.frame_buffer.pop_front();
        };

        if self
            .last_frame_instance
            .expect("last_frame_instance is None")
            .elapsed()
            >= Duration::from_millis(self.delay_ms as u64)
        {
            self.last_frame_instance = Some(Instant::now());

            if !self.is_bot_paused {
                self.current_frame = self.frame_buffer.pop_front();
            }
        }

        if let Some(frame_to_draw) = &self.current_frame {
            let ema_solution_time_ms =
                self.ema_solution_time.unwrap_or_default().as_secs_f64() * 1000.0;

            egui::CentralPanel::default().show(ctx, |ui| {
                // TODO reduce coupling
                ui::draw_ui(
                    ui,
                    &frame_to_draw,
                    &mut self.delay_ms,
                    self.game_state.cleared_rows,
                    ema_solution_time_ms,
                    &mut self.is_bot_paused,
                );
            });
        }

        // always repaint to avoid flickering/empty screen
        ctx.request_repaint();
    }
}
