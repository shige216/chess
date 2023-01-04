use std::collections::HashSet;

use super::{add_move, Cell, Color, Coordinate, HashMap};

pub fn king_move(
    from: &Coordinate,
    status: &HashMap<String, Cell>,
    color: &Color,
) -> HashSet<Coordinate> {
    let x = from.x();
    let y = from.y();

    let mut to = HashSet::new();

    match color {
        Color::White => {
            add_move(status, &mut to, x, y + 1, Color::Black);
            add_move(status, &mut to, x + 1, y + 1, Color::Black);
            add_move(status, &mut to, x + 1, y, Color::Black);
            add_move(status, &mut to, x + 1, y, Color::Black);
            add_move(status, &mut to, x + 1, y - 1, Color::Black);
            add_move(status, &mut to, x, y - 1, Color::Black);
            add_move(status, &mut to, x - 1, y - 1, Color::Black);
            add_move(status, &mut to, x - 1, y, Color::Black);
            add_move(status, &mut to, x - 1, y + 1, Color::Black);
        }
        Color::Black => {
            add_move(status, &mut to, x, y + 1, Color::White);
            add_move(status, &mut to, x + 1, y + 1, Color::White);
            add_move(status, &mut to, x + 1, y, Color::White);
            add_move(status, &mut to, x + 1, y, Color::White);
            add_move(status, &mut to, x + 1, y - 1, Color::White);
            add_move(status, &mut to, x, y - 1, Color::White);
            add_move(status, &mut to, x - 1, y - 1, Color::White);
            add_move(status, &mut to, x - 1, y, Color::White);
            add_move(status, &mut to, x - 1, y + 1, Color::White);
        }
    }

    to
}
