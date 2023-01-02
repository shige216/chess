use super::{add_move, Cell, Color, Coordinate};
use std::collections::HashMap;

fn add_pawn_initial_move(status: &HashMap<String, Cell>, to: &mut Vec<Coordinate>, x: u8, y: u8) {
    let key = format!("{}{}", x, y);
    if let Some(c) = status.get(&key) {
        if let None = c.piece {
            if let Ok(c) = Coordinate::new(x, y) {
                to.push(c);
            }
        }
    }
}

pub fn pawn_move(
    from: &Coordinate,
    status: &HashMap<String, Cell>,
    color: &Color,
) -> Vec<Coordinate> {
    let x = from.x();
    let y = from.y();

    let mut to = vec![];

    match color {
        Color::White => {
            if y == 2 {
                add_pawn_initial_move(status, &mut to, x, y + 1);
                add_pawn_initial_move(status, &mut to, x, y + 2);
            }

            add_move(status, &mut to, x + 1, y + 1, Color::Black);
            add_move(status, &mut to, x - 1, y + 1, Color::Black);
        }
        Color::Black => {
            if y == 7 {
                add_pawn_initial_move(status, &mut to, x, y - 1);
                add_pawn_initial_move(status, &mut to, x, y - 2);
            }

            add_move(status, &mut to, x + 1, y - 1, Color::White);
            add_move(status, &mut to, x - 1, y - 1, Color::White);
        }
    }

    to
}
