mod data;
mod logic;
mod ui;

use std::{thread, time::Duration};

use logic::{
    bot::Bot,
    state::{self, State},
};
use macroquad::prelude::*;

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

    let mut best_state = State::initial_state(&next_stack);

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

    let field2 = vec![
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

    best_state.field = field2.clone();
    best_state.uncleared_field = field2;

    loop {
        ui::field_ui::draw_field(&best_state.field);

        next_frame().await;

        // best_state.remaining_pieces = lookahead.get_next_stack();

        best_state.remaining_pieces = vec!['P'];

        println!("STACK: {:?}", best_state.remaining_pieces);

        // app hangs until heuristic_search returns
        best_state = Bot::heuristic_search(&best_state, &db, &mut id_manager).unwrap_or(best_state);

        // println!("{}", best_state);

        logic::game::update(&mut best_state);

        thread::sleep(Duration::from_millis(200));
    }
}
