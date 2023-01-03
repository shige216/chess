#[derive(Debug)]
pub enum Error {
    OutOfRange,
    InvalidKey,
    NoPiece,
    OpponentPiece,
    UnMovable,
}
