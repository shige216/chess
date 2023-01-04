use super::error::Error;

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
pub struct Coordinate {
    x: u32,
    y: u32,
}

impl Coordinate {
    pub fn new(x: u32, y: u32) -> Result<Coordinate, Error> {
        if x < 1 || x > 8 || y < 1 || y > 8 {
            Err(Error::OutOfRange)
        } else {
            Ok(Coordinate { x, y })
        }
    }

    pub fn x(&self) -> u32 {
        self.x
    }

    pub fn y(&self) -> u32 {
        self.y
    }
}
