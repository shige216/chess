use std::collections::HashSet;

use super::{add_move, Cell, Color, Coordinate, HashMap};

pub fn knight_move(
    from: &Coordinate,
    status: &HashMap<String, Cell>,
    color: &Color,
) -> HashSet<Coordinate> {
    let x = from.x();
    let y = from.y();

    let mut to = HashSet::new();

    match color {
        Color::White => {
            add_move(status, &mut to, x + 1, y + 2, Color::Black);
            add_move(status, &mut to, x + 2, y + 1, Color::Black);
            add_move(status, &mut to, x + 2, y - 1, Color::Black);
            add_move(status, &mut to, x + 1, y - 2, Color::Black);
            add_move(status, &mut to, x - 1, y - 2, Color::Black);
            add_move(status, &mut to, x - 2, y - 1, Color::Black);
            add_move(status, &mut to, x - 2, y + 1, Color::Black);
            add_move(status, &mut to, x - 1, y + 2, Color::Black);
        }
        Color::Black => {
            add_move(status, &mut to, x + 1, y + 2, Color::White);
            add_move(status, &mut to, x + 2, y + 1, Color::White);
            add_move(status, &mut to, x + 2, y - 1, Color::White);
            add_move(status, &mut to, x + 1, y - 2, Color::White);
            add_move(status, &mut to, x - 1, y - 2, Color::White);
            add_move(status, &mut to, x - 2, y - 1, Color::White);
            add_move(status, &mut to, x - 2, y + 1, Color::White);
            add_move(status, &mut to, x - 1, y + 2, Color::White);
        }
    }

    to
}
