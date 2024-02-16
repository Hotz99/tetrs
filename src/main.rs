#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use macroquad::prelude::*;
use std::{
    thread,
    time::{Duration, Instant},
};

mod data;
mod logic;
mod ui;

use logic::{
    bot::Bot,
    game, id_manager, next_shapes,
    state::{self, State},
};

use ui::*;

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Tetrs"),
        window_width: 200,
        window_height: 600,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]

async fn main() {
    let delay_ms = 300;

    let mut lookahead = next_shapes::NextShapes::new();
    let next_stack = Vec::with_capacity(logic::next_shapes::STACK_SIZE);
    let db = data::pentomino_db::PentominoDB::new();

    // println!("NEXT STACK: {:?}", next_stack);
    let mut state = State::initial_state(&next_stack);

    let mut run_time = Duration::new(0, 0);

    for i in 0..1000 {
        state.remaining_pieces = lookahead.get_next_stack();

        let start_time = Instant::now();

        match Bot::search(state.clone(), &db) {
            Some(solution) => {
                state = solution;
                println!("solution: {}", state);
            }
            None => {
                println!("NO SOLUTION");
                println!("NEXT STACK: {:?}", state.remaining_pieces);
                println!("{}", state);
                break;
            }
        };

        let end_time = Instant::now();

        run_time = end_time - start_time;

        println!("run {i}: {:?}", run_time);

        game::clear_rows(&mut state);
        ui::draw_field(&state.field);
        next_frame().await;
        thread::sleep(Duration::from_millis(delay_ms));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bot() {
        let delay_ms = 0;

        let mut lookahead = logic::next_shapes::NextShapes::new();
        let next_stack = Vec::with_capacity(logic::next_shapes::STACK_SIZE);
        let db = data::pentomino_db::PentominoDB::new();

        let runs = 10;
        let mut total_run_time = Duration::new(0, 0);
        let mut total_score = 0;
        let mut failed_counter = 0;

        for i in 0..runs {
            let mut state = State::initial_state(&next_stack);

            let mut run_time = Duration::new(0, 0);

            for j in 0..10000 {
                state.remaining_pieces = lookahead.get_next_stack();

                let start_time = Instant::now();

                match Bot::search(state, &db) {
                    Some(solution) => {
                        state = solution;
                    }
                    None => {
                        failed_counter += 1;
                        break;
                    }
                };

                let end_time = Instant::now();

                run_time = end_time - start_time;

                println!("run {i}: {:?}", run_time);

                thread::sleep(Duration::from_millis(delay_ms));

                game::clear_rows(&mut state);

                total_run_time += run_time;

                total_score += 1;
            }
        }

        println!("avg response: {:?}", total_run_time / runs / 100);
        println!("avg score: {:?}", total_score / runs);
        println!("failed runs: {:?}", failed_counter);
    }
}
