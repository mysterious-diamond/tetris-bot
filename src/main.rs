use macroquad::{prelude::*, window::request_new_screen_size};

use crate::tetris::Game;

mod tetris;

const ROWS: usize = 20;
const COLUMNS: usize = 10;
const CELL_SIZE: f32 = 40.0;

const SCREEN_WIDTH: f32 = COLUMNS as f32 * CELL_SIZE;
const SCREEN_HEIGHT: f32 = ROWS as f32 * CELL_SIZE;

#[macroquad::main(window_conf)]
async fn main() {
    let mut game: Game = Game::new();
    let mut last_game_step: f32 = 0.0;

    // Interval between game steps (in milliseconds)
    let game_frame_interval_ms: f32 = 0.5;
    request_new_screen_size(SCREEN_WIDTH, SCREEN_HEIGHT);

    loop {
        clear_background(BLACK);

        if last_game_step >= game_frame_interval_ms {
            last_game_step = 0.0;
            game.step();
        } else {
            last_game_step += get_frame_time();
        }

        game.render();
        next_frame().await;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Tetris-bot".to_owned(),
        window_width: SCREEN_WIDTH as i32,
        window_height: SCREEN_HEIGHT as i32,
        window_resizable: false,
        ..Default::default()
    }
}
