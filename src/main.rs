use macroquad::{prelude::*, window::request_new_screen_size};

use crate::tetris::{Game, Piece};

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

        if !game.is_dropping_piece {
            game.is_dropping_piece = true;

            let new_piece: Piece = Piece::new(game.next_piece);
            game.board.push(new_piece);

            game.next_piece = rand::RandomRange::gen_range(0, 6);
        }
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

    for piece in &game.board {
        for (i, val) in piece.piece_layout.iter().enumerate() {
            if !val {
                continue;
            }

            let x: f32 = (i % 4) as f32 + (piece.position as usize % COLUMNS) as f32 * CELL_SIZE;
            let y: f32 = (i / 4) as f32 + (piece.position as usize / COLUMNS) as f32 * CELL_SIZE;

            draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, RED);
        }
    }
}
