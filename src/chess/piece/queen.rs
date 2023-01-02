use super::{bishop_move, rook_move, Cell, Color, Coordinate, HashMap};

pub fn queen_move(
    from: &Coordinate,
    status: &HashMap<String, Cell>,
    color: &Color,
) -> Vec<Coordinate> {
    let mut bishop_move = bishop_move(from, status, color);
    let mut rook_move = rook_move(from, status, color);

    let mut to = vec![];
    to.append(&mut bishop_move);
    to.append(&mut rook_move);

    to
}
