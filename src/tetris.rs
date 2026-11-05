use macroquad::prelude::rand;

use crate::{COLUMNS, ROWS};

pub struct Game {
    pub board: Vec<Piece>,
    pub next_piece: u8,
    pub is_dropping_piece: bool,
}

pub struct Piece {
    pub position: u8,
    pub piece_layout: Vec<bool>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Vec::new(),
            next_piece: rand::RandomRange::gen_range(0, 6),
            is_dropping_piece: false,
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
            }
        } else if piece_type == 1 {
            Piece {
                piece_layout: vec![
                    false, false, true, false, true, true, true, false, false, false, false, false,
                    false, false, false, false,
                ],
                position,
            }
        } else if piece_type == 2 {
            Piece {
                piece_layout: vec![
                    true, false, false, false, true, true, true, false, false, false, false, false,
                    false, false, false, false,
                ],
                position,
            }
        } else if piece_type == 3 {
            Piece {
                piece_layout: vec![
                    false, true, true, false, true, true, false, false, false, false, false, false,
                    false, false, false, false,
                ],
                position,
            }
        } else if piece_type == 4 {
            Piece {
                piece_layout: vec![
                    true, true, false, false, false, true, true, false, false, false, false, false,
                    false, false, false, false,
                ],
                position,
            }
        } else if piece_type == 5 {
            Piece {
                piece_layout: vec![
                    true, true, false, false, true, true, false, false, false, false, false, false,
                    false, false, false, false,
                ],
                position,
            }
        } else if piece_type == 6 {
            Piece {
                piece_layout: vec![
                    false, false, false, false, true, true, true, true, false, false, false, false,
                    false, false, false, false,
                ],
                position,
            }
        } else {
            panic!("Invalid piece type in Piece::new function");
        }
    }
}
