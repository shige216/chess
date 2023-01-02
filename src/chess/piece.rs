mod pawn;
mod bishop;
mod queen;
mod king;
mod knight;
mod rook;

use std::collections::HashMap;
use super::{coordinate::Coordinate, cell::Cell};
use self::pawn::pawn_move;
use self::bishop::bishop_move;
use self::king::king_move;
use self::knight::knight_move;

pub struct Piece {
    pub role: Role,
    pub is_dead: bool,
    pub color: Color,
}

impl Piece {
    pub fn can_move(&self, from: Coordinate, status: HashMap<String, Cell>) -> Vec<Coordinate> {
        match self.role {
            Role::Bishop => bishop_move(from, &status, &self.color),
            Role::King => king_move(from, &status, &self.color),
            Role::Knight => knight_move(from, &status, &self.color),
            Role::Pawn => pawn_move(from, &status, &self.color),
            Role::Queen => vec![],
            Role::Rook => vec![],
        }
    }
}

pub enum Color {
    White,
    Black,
}

pub enum Role {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

fn add_move(status: &HashMap<String, Cell>, to: &mut Vec<Coordinate>, x: u8, y: u8, color: Color) {
    let key = format!("{}{}", x, y);
    if let Some(c) = status.get(&key) {
        if let Some(p) = &c.piece {
            match color {
                Color::Black => {
                    if let Color::White = p.color {
                        if let Ok(c) = Coordinate::new(x, y) {
                            to.push(c);
                        }
                    }
                },
                Color::White => {
                    if let Color::Black = p.color {
                        if let Ok(c) = Coordinate::new(x, y) {
                            to.push(c);
                        }
                    }
                },
            }
        }
    }
}
