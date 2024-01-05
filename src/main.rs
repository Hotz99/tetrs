mod data;
mod logic;
mod ui;

use logic::{bot::Bot, state::State};
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Tetrs"),
        window_width: 1200,
        window_height: 600,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        let mut game_shapes = logic::next_shapes::NextShapes::new();
        let next_stack = game_shapes.get_next_stack();

        Bot::astar_search(
            &State::initial_state(),
            &next_stack,
            &data::pentomino_db::PentominoDB::new(),
        );

        next_frame().await
    }
}

pub struct Settings {
    pub scale: f32,
}
