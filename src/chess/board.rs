use std::collections::HashMap;
use super::cell::Cell;
use super::coordinate::Coordinate;
use super::piece::{Piece, Role, Color};

pub struct Board {
    pub status: HashMap<String, Cell>,
}

impl Board {
    pub fn init(&self) -> Board {
        let status = init();
        Board { status }
    }

    pub fn moveTo(&mut self, from: Coordinate, to: Coordinate) {
        let from_key = format!("{}{}", from.x(), from.y());
        let cell = self.status.get_mut(&from_key).expect("invalid key");
        let piece = cell.piece.as_mut().expect("No piece there");
    }
}

fn init() -> HashMap<String, Cell> {
    let mut map: HashMap<String, Cell> = HashMap::new();
    for alpha in 1..9 {
        for num in 1..9 {
            let key = format!("{}{}", alpha, num);
            let coordinate = Coordinate::new(alpha, num);
            match coordinate {
                Ok(c) => {
                    let mut val = Cell {
                        coordinate: c,
                        piece: None,
                    };

                    spawn_pieces(&mut val, alpha, num);
                    map.entry(key).or_insert(val);
                },
                Err(e) => {
                    panic!("coordinates invalid.");
                },
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
