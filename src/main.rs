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

    // println!("NEXT STACK: {:?}", next_stack);

    let mut best_state = State::initial_state(&next_stack);

    let field = vec![
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

    best_state.field = field.clone();
    best_state.uncleared_field = field;

    loop {
        ui::field_ui::draw_field(&best_state.field);

        best_state.remaining_pieces = lookahead.get_next_stack();

        // best_state.remaining_pieces = vec!['P'];

        println!("STACK: {:?}", best_state.remaining_pieces);

        // best_state = Bot::astar_search(&best_state, &db).unwrap();

        println!("{}", best_state);

        thread::sleep(Duration::from_millis(1000));

        logic::game::update(&mut best_state);

        next_frame().await
    }
}

// fn main() {
//     let mut lookahead = logic::next_shapes::NextShapes::new();
//     let next_stack = Vec::with_capacity(logic::next_shapes::STACK_SIZE);
//     let db = data::pentomino_db::PentominoDB::new();

//     // println!("NEXT STACK: {:?}", next_stack);

//     let mut best_state = State::initial_state(&next_stack);

//     let field = vec![
//         vec![9, 9, state::EMPTY, state::EMPTY, state::EMPTY],
//         vec![9, 9, state::EMPTY, state::EMPTY, state::EMPTY],
//         vec![9, state::EMPTY, state::EMPTY, state::EMPTY, state::EMPTY],
//         vec![
//             state::EMPTY,
//             state::EMPTY,
//             state::EMPTY,
//             state::EMPTY,
//             state::EMPTY,
//         ],
//         vec![
//             state::EMPTY,
//             state::EMPTY,
//             state::EMPTY,
//             state::EMPTY,
//             state::EMPTY,
//         ],
//         vec![
//             state::EMPTY,
//             state::EMPTY,
//             state::EMPTY,
//             state::EMPTY,
//             state::EMPTY,
//         ],
//         vec![
//             state::EMPTY,
//             state::EMPTY,
//             state::EMPTY,
//             state::EMPTY,
//             state::EMPTY,
//         ],
//         vec![
//             state::EMPTY,
//             state::EMPTY,
//             state::EMPTY,
//             state::EMPTY,
//             state::EMPTY,
//         ],
//         vec![
//             state::EMPTY,
//             state::EMPTY,
//             state::EMPTY,
//             state::EMPTY,
//             state::EMPTY,
//         ],
//         vec![
//             state::EMPTY,
//             state::EMPTY,
//             state::EMPTY,
//             state::EMPTY,
//             state::EMPTY,
//         ],
//         vec![
//             state::EMPTY,
//             state::EMPTY,
//             state::EMPTY,
//             state::EMPTY,
//             state::EMPTY,
//         ],
//         vec![
//             state::EMPTY,
//             state::EMPTY,
//             state::EMPTY,
//             state::EMPTY,
//             state::EMPTY,
//         ],
//         vec![
//             state::EMPTY,
//             state::EMPTY,
//             state::EMPTY,
//             state::EMPTY,
//             state::EMPTY,
//         ],
//         vec![1, 1, 1, 1, 1],
//         vec![1, 1, 1, 1, 1],
//     ];

//     best_state.field = field.clone();
//     best_state.uncleared_field = field;

//     loop {
//         best_state.remaining_pieces = lookahead.get_next_stack();

//         // best_state.remaining_pieces = vec!['P'];

//         println!("STACK: {:?}", best_state.remaining_pieces);

//         best_state = Bot::astar_search(&best_state, &db).unwrap();

//         println!("{}", best_state);

//         thread::sleep(Duration::from_millis(1000));
//     }
// }

pub struct Settings {
    pub scale: f32,
}
