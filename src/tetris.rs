use macroquad::prelude::rand;

use crate::{COLUMNS, ROWS};

pub struct Game {
    pub board: Vec<bool>,
    pub next_piece: u8,
}

pub trait Piece {
    fn get_piece_layout() -> Vec<bool>;
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: vec![false; ROWS * COLUMNS],
            next_piece: rand::RandomRange::gen_range(0, 6),
        }
    }
}
