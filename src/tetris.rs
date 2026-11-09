use macroquad::{
    color::{BLACK, BLUE, Color, DARKBLUE, GREEN, ORANGE, PURPLE, RED, WHITE, YELLOW},
    input::KeyCode,
    prelude::rand,
    shapes::{draw_line, draw_rectangle},
};

use crate::{BASE_TICKS_INTERVAL, CELL_SIZE, COLUMNS, ROWS, SCREEN_HEIGHT, SCREEN_WIDTH};

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
            board: vec![(false, BLACK); ROWS * COLUMNS],
            cur_piece: Piece::new(cur_piece_type),
            next_piece,
            is_dropping_piece: true,
        }
    }

    pub fn step(&mut self, key_pressed_this_tick: KeyCode, ticks: &mut f32) {
        let mut dir_side: i8 = 0;
        let mut dir_rotate: i8 = 0;
        let mut fast_fall: bool = false;

        if key_pressed_this_tick == KeyCode::A || key_pressed_this_tick == KeyCode::Left {
            dir_side = -1;
        } else if key_pressed_this_tick == KeyCode::D || key_pressed_this_tick == KeyCode::Right {
            dir_side = 1;
        } else if key_pressed_this_tick == KeyCode::S || key_pressed_this_tick == KeyCode::Down {
            fast_fall = true;
        } else if key_pressed_this_tick == KeyCode::Q {
            dir_rotate = -1;
        } else if key_pressed_this_tick == KeyCode::E {
            dir_rotate = 1;
        }

        self.try_rotate_to_side(dir_rotate);
        self.try_move_to_side(dir_side);
        self.cur_piece.position += COLUMNS as u8;
        if fast_fall {
            *ticks = BASE_TICKS_INTERVAL / 2.5;
        } else {
            *ticks = BASE_TICKS_INTERVAL;
        }

        let on_ground: bool = self.is_cur_piece_on_ground();

        if !on_ground {
            return;
        }

        self.insert_piece_to_game_board();

        if !self.is_dropping_piece {
            self.is_dropping_piece = true;

            let new_piece: Piece = Piece::new(self.next_piece);
            self.cur_piece = new_piece;

            self.next_piece = rand::RandomRange::gen_range(0, 7);
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

        self.draw_board();
        self.draw_grid();
    }

    fn try_rotate_to_side(&mut self, dir_rotate: i8) {
        let cur_piece: &mut Piece = &mut self.cur_piece;

        if dir_rotate == -1 {
            let mut new_layout = vec![false; 4 * 4];
            for (i, val) in cur_piece.piece_layout.iter().enumerate() {
                if !val {
                    continue;
                }

                let row: u8 = i as u8 / 4;
                let column: u8 = i as u8 % 4;

                let new_index: u8 = (3 - column) * 4 + row;

                let new_real_row: u8 = (cur_piece.position / COLUMNS as u8) + (new_index / 4);
                let new_real_column: u8 = (cur_piece.position % COLUMNS as u8) + (new_index % 4);
                let new_real_pos: u8 = new_real_column + new_real_row * COLUMNS as u8;

                if new_real_pos as usize >= ROWS * COLUMNS || self.board[new_real_pos as usize].0 {
                    return;
                }
                new_layout[new_index as usize] = true;
            }

            for i in 0..4u8 {
                let supposed_row: u8 = cur_piece.position / COLUMNS as u8 + i;
                for j in 1..4u8 {
                    let row = (cur_piece.position + j) / COLUMNS as u8 + i;

                    if supposed_row != row {
                        return;
                    }
                }
            }

            cur_piece.piece_layout = new_layout;
        }
    }
    // Tries to move the current piece to either the left or Right
    // dir_side: 1 -> Right
    // dir_side: 0 -> Nothing
    // dir_side: -1 -> Left
    fn try_move_to_side(&mut self, dir_side: i8) {
        let cur_piece: &mut Piece = &mut self.cur_piece;
        let new_piece_pos: u8 = (cur_piece.position as i8 + dir_side) as u8;

        for (i, val) in cur_piece.piece_layout.iter().enumerate() {
            if !val {
                continue;
            }

            let layout_row: u8 = (i / 4) as u8;
            let layout_column: u8 = (i % 4) as u8;
            let relative_layout_pos: u8 = layout_column + layout_row * COLUMNS as u8;

            let real_pos: u8 = cur_piece.position + relative_layout_pos;
            let new_real_pos: u8 = new_piece_pos + relative_layout_pos;

            if new_real_pos as usize >= ROWS * COLUMNS || self.board[new_real_pos as usize].0 {
                return;
            }

            if real_pos / COLUMNS as u8 != new_real_pos / COLUMNS as u8 {
                println!("Not same row!, {}, {}", real_pos, new_real_pos);
                return;
            }
        }

        cur_piece.position = (cur_piece.position as i8 + dir_side) as u8;
    }

    fn is_cur_piece_on_ground(&self) -> bool {
        let cur_piece: &Piece = &self.cur_piece;
        for (i, val) in cur_piece.piece_layout.iter().enumerate() {
            if !val {
                continue;
            }

            let x: u8 = (cur_piece.position % COLUMNS as u8) + (i % 4) as u8;
            let y: u8 = (cur_piece.position / COLUMNS as u8) + (i / 4) as u8 - 1;

            let cell_pos: usize = x as usize + y as usize * COLUMNS;
            let below: usize = cell_pos + COLUMNS;

            if below >= ROWS * COLUMNS || self.board[below].0 {
                return true;
            }
        }

        false
    }

    fn insert_piece_to_game_board(&mut self) {
        let cur_piece: &Piece = &self.cur_piece;
        for (i, val) in cur_piece.piece_layout.iter().enumerate() {
            if !val {
                continue;
            }

            let x: u8 = (cur_piece.position % COLUMNS as u8) + (i % 4) as u8;
            let y: u8 = (cur_piece.position / COLUMNS as u8) + (i / 4) as u8 - 1;

            let id: usize = x as usize + y as usize * COLUMNS;

            self.board[id] = (true, cur_piece.color);
            self.is_dropping_piece = false;
        }
    }

    fn draw_grid(&self) {
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
    fn draw_board(&self) {
        for (i, val) in self.board.iter().enumerate() {
            let filled: bool = val.0;

            if !filled {
                continue;
            }

            let x: f32 = (i % COLUMNS) as f32 * CELL_SIZE;
            let y: f32 = (i / COLUMNS) as f32 * CELL_SIZE;

            draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, val.1);
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
