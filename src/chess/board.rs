use super::cell::Cell;
use super::coordinate::Coordinate;
use super::error::Error;
use super::piece::{Color, Piece, Role};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Board {
    pub status: HashMap<String, Cell>,
}

impl Board {
    pub fn init() -> Board {
        let status = init();
        Board { status }
    }

    pub fn show(&self) {
        for y in [8, 7, 6, 5, 4, 3, 2, 1] {
            print!("{} ", y);
            for x in 1..9 {
                let key = format!("{}{}", x, y);
                print_board(self, &key);
            }
            println!("");
        }
        println!("  1 2 3 4 5 6 7 8 ");
    }

    pub fn can_move(&self, from: Coordinate, color: Color) -> Result<Vec<Coordinate>, Error> {
        let key = format!("{}{}", from.x(), from.y());
        let cell = self.status.get(&key);
        match cell {
            Some(c) => {
                let piece = c.piece.as_ref();
                match piece {
                    Some(p) => {
                        if p.color == color {
                            let options = p.can_move(&from, &self.status);
                            Ok(options)
                        } else {
                            Err(Error::OpponentPiece)
                        }
                    }
                    None => Err(Error::NoPiece)
                }
            }
            None => Err(Error::InvalidKey)
        }
    }

    pub fn move_to(&mut self, from: Coordinate, to: Coordinate, color: Color) -> Result<(), Error> {
        let key = format!("{}{}", &from.x(), &from.y());
        let options = self.can_move(from, color)?;
        if !options.contains(&to) {
            Err(Error::UnMovable)
        } else {
            let to = format!("{}{}", to.x(), to.y());
            if let Some(c) = self.status.get_mut(&key) {
                if let Some(p) = c.piece.take() {
                    if let Some(to_c) = self.status.get_mut(&to) {
                        to_c.piece = Some(p);
                    }
                }
            }
            Ok(())
        }
    }
}

fn print_board(board: &Board, key: &str) {
    let none = '□';
    if let Some(c) = board.status.get(key) {
        match &c.piece {
            Some(p) => {
                match &p.color {
                    Color::Black => {
                        match &p.role {
                            Role::Bishop => print!("{} ", '♝'),
                            Role::Queen => print!("{} ", '♛'),
                            Role::King => print!("{} ", '♚'),
                            Role::Knight => print!("{} ", '♞'),
                            Role::Rook => print!("{} ", '♜'),
                            Role::Pawn => print!("{} ", '♟'),
                        }
                    }
                    Color::White => {
                        match &p.role {
                            Role::Bishop => print!("{} ", '♗'),
                            Role::Queen => print!("{} ", '♕'),
                            Role::King => print!("{} ", '♔'),
                            Role::Knight => print!("{} ", '♘'),
                            Role::Rook => print!("{} ", '♖'),
                            Role::Pawn => print!("{} ", '♙'),
                        }
                    }
                }
            },
            None => print!("{} ", none),
        }
    }
}

fn init() -> HashMap<String, Cell> {
    let mut map: HashMap<String, Cell> = HashMap::new();
    for alpha in 1..9 {
        for num in 1..9 {
            let key = format!("{}{}", alpha, num);
            if let Ok(c) = Coordinate::new(alpha, num) {
                let mut val = Cell {
                    coordinate: c,
                    piece: None,
                };

                spawn_pieces(&mut val, alpha, num);
                map.entry(key).or_insert(val);
            }
        }
    }

    map
}

fn spawn_pieces(val: &mut Cell, alpha: u8, num: u8) {
    if num == 2 {
        val.piece = Some(Piece {
            role: Role::Pawn,
            is_dead: false,
            color: Color::White,
        });
    } else if num == 7 {
        val.piece = Some(Piece {
            role: Role::Pawn,
            is_dead: false,
            color: Color::Black,
        });
    } else if num == 1 {
        match alpha {
            1 => {
                val.piece = Some(Piece {
                    role: Role::Rook,
                    is_dead: false,
                    color: Color::White,
                })
            }
            2 => {
                val.piece = Some(Piece {
                    role: Role::Knight,
                    is_dead: false,
                    color: Color::White,
                })
            }
            3 => {
                val.piece = Some(Piece {
                    role: Role::Bishop,
                    is_dead: false,
                    color: Color::White,
                })
            }
            4 => {
                val.piece = Some(Piece {
                    role: Role::King,
                    is_dead: false,
                    color: Color::White,
                })
            }
            5 => {
                val.piece = Some(Piece {
                    role: Role::Queen,
                    is_dead: false,
                    color: Color::White,
                })
            }
            6 => {
                val.piece = Some(Piece {
                    role: Role::Bishop,
                    is_dead: false,
                    color: Color::White,
                })
            }
            7 => {
                val.piece = Some(Piece {
                    role: Role::Knight,
                    is_dead: false,
                    color: Color::White,
                })
            }
            8 => {
                val.piece = Some(Piece {
                    role: Role::Rook,
                    is_dead: false,
                    color: Color::White,
                })
            }
            _ => (),
        }
    } else if num == 8 {
        match alpha {
            1 => {
                val.piece = Some(Piece {
                    role: Role::Rook,
                    is_dead: false,
                    color: Color::Black,
                })
            }
            2 => {
                val.piece = Some(Piece {
                    role: Role::Knight,
                    is_dead: false,
                    color: Color::Black,
                })
            }
            3 => {
                val.piece = Some(Piece {
                    role: Role::Bishop,
                    is_dead: false,
                    color: Color::Black,
                })
            }
            4 => {
                val.piece = Some(Piece {
                    role: Role::King,
                    is_dead: false,
                    color: Color::Black,
                })
            }
            5 => {
                val.piece = Some(Piece {
                    role: Role::Queen,
                    is_dead: false,
                    color: Color::Black,
                })
            }
            6 => {
                val.piece = Some(Piece {
                    role: Role::Bishop,
                    is_dead: false,
                    color: Color::Black,
                })
            }
            7 => {
                val.piece = Some(Piece {
                    role: Role::Knight,
                    is_dead: false,
                    color: Color::Black,
                })
            }
            8 => {
                val.piece = Some(Piece {
                    role: Role::Rook,
                    is_dead: false,
                    color: Color::Black,
                })
            }
            _ => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Board, chess::coordinate::Coordinate};

    #[test]
    fn init() {
        let board = Board::init();
        board.show();
    }

    #[test]
    fn can_move() {
        let board = Board::init();
        let options = board.can_move(Coordinate::new(1, 2).unwrap(), crate::chess::piece::Color::White);
        println!("{:?}", options);
        let options = board.can_move(Coordinate::new(1, 7).unwrap(), crate::chess::piece::Color::White);
        println!("{:?}", options);
    }

    #[test]
    fn move_to() {
        let mut board = Board::init();
        board.show();
        println!("");

        board.move_to(
            Coordinate::new(1, 2).unwrap(),
            Coordinate::new(1, 3).unwrap(),
            crate::chess::piece::Color::White
        );
        board.show();
        println!("");

        board.move_to(
            Coordinate::new(1, 3).unwrap(),
            Coordinate::new(1, 4).unwrap(),
            crate::chess::piece::Color::White
        );
        board.show();
        println!("");
    }
}