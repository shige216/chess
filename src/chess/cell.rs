use super::coordinate::Coordinate;
use super::piece::Piece;

#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct Cell {
    pub coordinate: Coordinate,
    pub piece: Option<Piece>,
}
