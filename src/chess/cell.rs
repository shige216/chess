use super::coordinate::Coordinate;
use super::piece::Piece;

#[derive(Debug)]
pub struct Cell {
    pub coordinate: Coordinate,
    pub piece: Option<Piece>,
}
