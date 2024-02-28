#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

use macroquad::prelude::*;
use std::{
    cell::RefCell,
    rc::Rc,
    thread,
    time::{Duration, Instant},
};

mod data;
mod logic;
mod ui;

use logic::{
    bot, game, id_manager, next_shapes,
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
    // test_bot();

    let delay_ms = 500;

    let mut lookahead = next_shapes::NextShapes::new();
    let db = data::pentomino_db::PentominoDB::new();
    let mut id_manager = id_manager::IdManager::new();

    let mut state = State::initial_state();

    loop {
        state.remaining_pieces = lookahead.get_next_stack();

        match bot::search(state, &db, &mut id_manager) {
            Some(solution) => {
                state = solution;
            }
            None => {
                println!("NO SOLUTION");
                break;
            }
        };

        game::clear_rows(&mut state, &mut id_manager, 0, true, true, delay_ms);
    }
}

fn test_bot() {
    let mut lookahead = logic::next_shapes::NextShapes::new();
    let pent_db = data::pentomino_db::PentominoDB::new();
    let mut id_manager = id_manager::IdManager::new();

    let runs = 10;
    let searches = 10000;
    let mut total_run_time = Duration::new(0, 0);
    let mut total_solution_time = Duration::new(0, 0);
    let mut total_score = 0;
    let mut failed_counter = 0;

    for i in 0..runs {
        let mut state = State::initial_state();

        let mut run_time = Duration::new(0, 0);
        let run_start = Instant::now();

        for j in 0..searches {
            state.remaining_pieces = lookahead.get_next_stack();

            let solution_start = Instant::now();

            match bot::search(state, &pent_db, &mut id_manager) {
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

            game::clear_rows(&mut state, &mut id_manager, 0, true, false, 0);
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
