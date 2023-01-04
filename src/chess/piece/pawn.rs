use super::{Cell, Color, Coordinate};
use std::collections::{HashMap, HashSet};

fn add_pawn_forward_move(status: &HashMap<String, Cell>, to: &mut HashSet<Coordinate>, x: u32, y: u32) {
    let key = format!("{}{}", x, y);
    if let Some(c) = status.get(&key) {
        if let None = c.piece {
            if let Ok(c) = Coordinate::new(x, y) {
                to.insert(c);
            }
        }
    }
}

fn add_pawn_diagonal_move(status: &HashMap<String, Cell>, to: &mut HashSet<Coordinate>, x: u32, y: u32, color: &Color) {
    let key = format!("{}{}", x, y);
    if let Some(c) = status.get(&key) {
        if let Some(p) = c.piece {
            match p.color {
                Color::White => {
                    if color == &Color::Black {
                        if let Ok(c) = Coordinate::new(x, y) {
                            to.insert(c);
                        }
                    }
                }
                Color::Black => {
                    if color == &Color::White {
                        if let Ok(c) = Coordinate::new(x, y) {
                            to.insert(c);
                        }
                    }
                }
            }
        }
    }
}

pub fn pawn_move(
    from: &Coordinate,
    status: &HashMap<String, Cell>,
    color: &Color,
) -> HashSet<Coordinate> {
    let x = from.x();
    let y = from.y();

    let mut to = HashSet::new();

    match color {
        Color::White => {
            if y == 2 {
                add_pawn_forward_move(status, &mut to, x, y + 1);
                add_pawn_forward_move(status, &mut to, x, y + 2);
            }

            add_pawn_forward_move(status, &mut to, x, y + 1);
            add_pawn_diagonal_move(status, &mut to, x + 1, y + 1, color);
            add_pawn_diagonal_move(status, &mut to, x - 1, y + 1, color);
        }
        Color::Black => {
            if y == 7 {
                add_pawn_forward_move(status, &mut to, x, y - 1);
                add_pawn_forward_move(status, &mut to, x, y - 2);
            }

            add_pawn_forward_move(status, &mut to, x, y - 1);
            add_pawn_diagonal_move(status, &mut to, x + 1, y - 1, color);
            add_pawn_diagonal_move(status, &mut to, x - 1, y - 1, color);
        }
    }

    to
}
