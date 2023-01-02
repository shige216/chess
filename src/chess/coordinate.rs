use super::error::Error;

pub struct Coordinate {
    x: u8,
    y: u8,
}

impl Coordinate {
    pub fn new(x: u8, y: u8) -> Result<Coordinate, Error> {
        if x < 1 || x > 8 || y < 1 || y > 8 {
            Err(Error::OutOfRange)
        } else {
            Ok(Coordinate { x, y })
        }
    }

    pub fn x(&self) -> u8 {
        self.x
    }

    pub fn y(&self) -> u8 {
        self.y
    }
}
