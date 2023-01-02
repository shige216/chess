use super::coordinate::Coordinate;
use super::piece::Piece;

pub struct Cell {
    pub coordinate: Coordinate,
    pub piece: Option<Piece>,
}
