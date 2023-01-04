use std::collections::HashSet;

use super::{bishop_move, rook_move, Cell, Color, Coordinate, HashMap};

pub fn queen_move(
    from: &Coordinate,
    status: &HashMap<String, Cell>,
    color: &Color,
) -> HashSet<Coordinate> {
    let bishop_move = bishop_move(from, status, color);
    let rook_move = rook_move(from, status, color);

    let mut to = HashSet::new();

    for c in bishop_move {
        to.insert(c);
    }

    for c in rook_move {
        to.insert(c);
    }

    to
}
