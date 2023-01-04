#[derive(Debug)]
pub enum Error {
    OutOfRange,
    InvalidKey,
    NoPiece,
    OpponentPiece,
    UnMovable,
}

pub fn print_error(error: Error) {
    match error {
        Error::InvalidKey => println!("Invalid key."),
        Error::NoPiece => println!("No piece there."),
        Error::OpponentPiece => println!("That's an opponent piece."),
        Error::OutOfRange => println!("Invalid key."),
        Error::UnMovable => println!("Can't move it there"),
    }
}