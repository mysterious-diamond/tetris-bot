use macroquad::{
    color::{BLUE, Color, DARKBLUE, GREEN, ORANGE, PURPLE, RED, WHITE, YELLOW},
    prelude::rand,
    shapes::{draw_line, draw_rectangle},
};

use crate::{CELL_SIZE, COLUMNS, ROWS, SCREEN_HEIGHT, SCREEN_WIDTH};

pub struct Game {
    pub board: Vec<(bool, Color)>,
    pub cur_piece: Piece,
    pub next_piece: u8,
    pub is_dropping_piece: bool,
}

pub struct Piece {
    pub position: u8,
    pub piece_layout: Vec<bool>,
    pub color: Color,
}

impl Game {
    pub fn new() -> Game {
        let cur_piece_type: u8 = rand::RandomRange::gen_range(0, 6);
        let next_piece: u8 = rand::RandomRange::gen_range(0, 6);
        Game {
            board: Vec::new(),
            cur_piece: Piece::new(cur_piece_type),
            next_piece,
            is_dropping_piece: false,
        }
    }

    pub fn step(&mut self) {
        if !self.is_dropping_piece {
            self.is_dropping_piece = true;

            let new_piece: Piece = Piece::new(self.next_piece);
            self.cur_piece = new_piece;

            self.next_piece = rand::RandomRange::gen_range(0, 6);
        }
    }

    pub fn render(&self) {
        let piece: &Piece = &self.cur_piece;
        for (i, val) in piece.piece_layout.iter().enumerate() {
            if !val {
                continue;
            }

            let x: f32 = ((i % 4) as f32 + (piece.position as usize % COLUMNS) as f32) * CELL_SIZE;
            let y: f32 = ((i / 4) as f32 + (piece.position as usize / COLUMNS) as f32) * CELL_SIZE;

            draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, piece.color);
        }

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
    }
}

impl Piece {
    pub fn new(piece_type: u8) -> Piece {
        let position: u8 = COLUMNS as u8 + (COLUMNS as u8 / 2) - 2;
        if piece_type == 0 {
            Piece {
                piece_layout: vec![
                    false, true, false, false, true, true, true, false, false, false, false, false,
                    false, false, false, false,
                ],
                position,
                color: RED,
            }
        } else if piece_type == 1 {
            Piece {
                piece_layout: vec![
                    false, false, true, false, true, true, true, false, false, false, false, false,
                    false, false, false, false,
                ],
                position,
                color: BLUE,
            }
        } else if piece_type == 2 {
            Piece {
                piece_layout: vec![
                    true, false, false, false, true, true, true, false, false, false, false, false,
                    false, false, false, false,
                ],
                position,
                color: ORANGE,
            }
        } else if piece_type == 3 {
            Piece {
                piece_layout: vec![
                    false, true, true, false, true, true, false, false, false, false, false, false,
                    false, false, false, false,
                ],
                position,
                color: PURPLE,
            }
        } else if piece_type == 4 {
            Piece {
                piece_layout: vec![
                    true, true, false, false, false, true, true, false, false, false, false, false,
                    false, false, false, false,
                ],
                position,
                color: YELLOW,
            }
        } else if piece_type == 5 {
            Piece {
                piece_layout: vec![
                    true, true, false, false, true, true, false, false, false, false, false, false,
                    false, false, false, false,
                ],
                position,
                color: GREEN,
            }
        } else if piece_type == 6 {
            Piece {
                piece_layout: vec![
                    false, false, false, false, true, true, true, true, false, false, false, false,
                    false, false, false, false,
                ],
                position,
                color: DARKBLUE,
            }
        } else {
            panic!("Invalid piece type in Piece::new function");
        }
    }
}
