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
    game,
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
    let mut lookahead = logic::next_shapes::NextShapes::new();
    let next_stack = Vec::with_capacity(logic::next_shapes::STACK_SIZE);
    let db = data::pentomino_db::PentominoDB::new();
    let mut id_manager = logic::id_manager::IdManager::new();

    // println!("NEXT STACK: {:?}", next_stack);

    let mut state = State::initial_state(&next_stack);

    let field1 = vec![
        vec![9, 9, state::EMPTY, state::EMPTY, state::EMPTY],
        vec![9, 9, state::EMPTY, state::EMPTY, state::EMPTY],
        vec![9, state::EMPTY, state::EMPTY, state::EMPTY, state::EMPTY],
        vec![
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
        ],
        vec![
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
        ],
        vec![
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
        ],
        vec![
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
        ],
        vec![
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
        ],
        vec![
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
        ],
        vec![
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
        ],
        vec![9, 9, state::EMPTY, state::EMPTY, state::EMPTY],
        vec![9, 9, state::EMPTY, state::EMPTY, state::EMPTY],
        vec![9, state::EMPTY, state::EMPTY, state::EMPTY, state::EMPTY],
        vec![1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1],
    ];

    let p_composite_id = logic::id_manager::create_composite_id(&9, &0);
    let l_composite_id = logic::id_manager::create_composite_id(&8, &1);

    let gravity_test_field = vec![
        vec![
            p_composite_id,
            p_composite_id,
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
        ],
        vec![
            p_composite_id,
            p_composite_id,
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
        ],
        vec![
            p_composite_id,
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
        ],
        vec![
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
        ],
        vec![
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
        ],
        vec![
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
        ],
        vec![
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
        ],
        vec![
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
        ],
        vec![
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
        ],
        vec![
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
        ],
        vec![
            l_composite_id,
            l_composite_id,
            l_composite_id,
            l_composite_id,
            state::EMPTY,
        ],
        vec![
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
        ],
        vec![
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
        ],
        vec![
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
        ],
        vec![
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
            state::EMPTY,
        ],
    ];

    state.field = gravity_test_field;

    let runs = 100;
    let mut total_time = Duration::new(0, 0);

    for i in 0..runs {
        state.remaining_pieces = lookahead.get_next_stack();

        // state.remaining_pieces = vec!['P'];

        println!("STACK: {:?}", state.remaining_pieces);

        let start_time = Instant::now();

        // app hangs until heuristic_search returns
        // match Bot::heuristic_search(&state, &db, &mut id_manager) {
        //     Some(solution) => state = solution,
        //     None => {
        //         println!("No solution found");
        //         break;
        //     }
        // };

        let end_time = Instant::now();

        let elapsed = end_time - start_time;

        println!("run {i}: {:?}", elapsed);

        println!("{}", state);
        thread::sleep(Duration::from_millis(500));

        game::gravity(&mut state, &mut id_manager);

        total_time += elapsed;
    }

    println!("avg: {:?}", total_time / runs);
}
