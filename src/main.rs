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
    request_new_screen_size(SCREEN_WIDTH, SCREEN_HEIGHT);

    loop {
        clear_background(BLACK);
        draw_board(&game);
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

fn draw_board(game: &Game) {
    for i in 0..ROWS {
        draw_line(
            0.0,
            i as f32 * CELL_SIZE,
            SCREEN_WIDTH,
            i as f32 * CELL_SIZE,
            1.0,
            WHITE,
        );
    }

    for i in 0..COLUMNS {
        draw_line(
            i as f32 * CELL_SIZE,
            0.0,
            i as f32 * CELL_SIZE,
            SCREEN_HEIGHT,
            1.0,
            WHITE,
        );
    }

    for i in 0..ROWS * COLUMNS {
        if game.board[i] {
            let x: usize = i % COLUMNS;
            let y: usize = i / ROWS;
            draw_rectangle(x as f32, y as f32, CELL_SIZE, CELL_SIZE, RED);
        }
    }
}
