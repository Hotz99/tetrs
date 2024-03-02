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
        state::{Field, State},
    },
    ui,
};

pub struct App {
    state: State,
    id_manager: IdManager,
    lookahead: NextShapes,
    pent_db: PentominoDB,
    delay_ms: u16,
    frames: VecDeque<Field>,
    current_frame: Option<Field>,
    last_frame_time: Instant,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.frames.is_empty() {
            let start_time = Instant::now();

            self.bot_search();

            println!("search time: {:?}", Instant::now() - start_time);

            game::animate_update(
                &mut self.state,
                &mut self.id_manager,
                0,
                true,
                &mut self.frames,
            );

            // TODO: remove duplicates for VecDeque
            // self.frames.dedup_by(|x, y| x == y);

            println!("frame count: {}", self.frames.len());

            self.current_frame = self.frames.pop_front();
            self.last_frame_time = Instant::now();
        }

        let now = Instant::now();

        // if delay_ms time has passed, update current_frame
        if now.duration_since(self.last_frame_time).as_millis() >= self.delay_ms as u128 {
            self.last_frame_time = now;
            self.current_frame = self.frames.pop_front();

            println!("tickj")
        }

        // let mut foo_state = State::initial_state();
        // foo_state.field = self.current_frame.clone().unwrap();

        // println!("{}", foo_state);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui::draw_game_field(ui, &self.current_frame.as_ref().unwrap());
        });

        ctx.request_repaint();
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            state: State::initial_state(),
            id_manager: IdManager::new(),
            lookahead: NextShapes::new(),
            pent_db: PentominoDB::new(),
            delay_ms: 500,
            frames: VecDeque::new(),
            current_frame: Some(Field::new()),
            last_frame_time: Instant::now(),
        }
    }

    fn bot_search(&mut self) {
        self.state.remaining_pieces = self.lookahead.get_next_stack();

        match bot::search(self.state.clone(), &self.pent_db, &mut self.id_manager) {
            Some(solution) => {
                self.state = solution;
                println!("SOLUTION FOUND");
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
            let mut state = State::initial_state();

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

                game::update(&mut state, &mut self.id_manager, 0, true, false, 0);
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
