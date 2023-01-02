mod bishop;
mod king;
mod knight;
mod pawn;
mod queen;
mod rook;

use self::bishop::bishop_move;
use self::king::king_move;
use self::knight::knight_move;
use self::pawn::pawn_move;
use self::queen::queen_move;
use self::rook::rook_move;
use super::{cell::Cell, coordinate::Coordinate};
use std::collections::HashMap;

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

pub struct Piece {
    pub role: Role,
    pub is_dead: bool,
    pub color: Color,
}

impl Piece {
    pub fn can_move(&self, from: &Coordinate, status: &HashMap<String, Cell>) -> Vec<Coordinate> {
        match self.role {
            Role::Bishop => bishop_move(from, status, &self.color),
            Role::King => king_move(from, status, &self.color),
            Role::Knight => knight_move(from, status, &self.color),
            Role::Pawn => pawn_move(from, status, &self.color),
            Role::Queen => queen_move(from, status, &self.color),
            Role::Rook => rook_move(from, status, &self.color),
        }
    }
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
                }
                Color::White => {
                    if let Color::Black = p.color {
                        if let Ok(c) = Coordinate::new(x, y) {
                            to.push(c);
                        }
                    }
                }
            }
        }
    }
}

fn add_loop_move(
    status: &HashMap<String, Cell>,
    to: &mut Vec<Coordinate>,
    dx: u8,
    dy: u8,
    color: &Color,
) -> bool {
    let key = format!("{}{}", dx, dy);
    if let Some(c) = status.get(&key) {
        match &c.piece {
            Some(p) => match color {
                Color::White => match p.color {
                    Color::White => {
                        return false;
                    }
                    Color::Black => {
                        if let Ok(c) = Coordinate::new(dx, dy) {
                            to.push(c);
                            return false;
                        }
                    }
                },
                Color::Black => match p.color {
                    Color::Black => {
                        return false;
                    }
                    Color::White => {
                        if let Ok(c) = Coordinate::new(dx, dy) {
                            to.push(c);
                            return false;
                        }
                    }
                },
            },
            None => {
                if let Ok(c) = Coordinate::new(dx, dy) {
                    to.push(c);
                }
            }
        }
    }

    return true;
}
