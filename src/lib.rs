use std::collections::HashMap;

struct Coordinate(char, u8);

struct Cell {
  coordinate: Coordinate,
  piece: Option<Piece>,
}

struct Piece {
  role: Role,
  is_dead: bool,
  color: Color,
}

impl Piece {
  fn can_move_to(&self, coordinate: Coordinate) -> bool {
    true
  }

  fn move_to(&mut self, coordinate: Coordinate) {

  }
}

enum Color {
  White,
  Black,
}

enum Role {
  Pawn,
  Knight,
  Bishop,
  Rook,
  Queen,
  King,
}

struct Board {
  status: HashMap<String, Cell>,
}

impl Board {
  fn init(&self) -> Board {
    let status = init();
    Board { status }
  }
}

fn init() -> HashMap<String, Cell> {
  let mut map: HashMap<String, Cell> = HashMap::new();
  let arr = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
  for alpha in arr {
    for num in 1..9 {
      let key = format!("{}{}", alpha, num);
      let mut val = Cell {
        coordinate: Coordinate(alpha, num),
        piece: None,
      };

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
            'a' => val.piece = Some(Piece {
              role: Role::Rook,
              is_dead: false,
              color: Color::White,
            }),
            'b' => val.piece = Some(Piece {
              role: Role::Knight,
              is_dead: false,
              color: Color::White,
            }),
            'c' => val.piece = Some(Piece {
              role: Role::Bishop,
              is_dead: false,
              color: Color::White,
            }),
            'd' => val.piece = Some(Piece {
              role: Role::King,
              is_dead: false,
              color: Color::White,
            }),
            'e' => val.piece = Some(Piece {
              role: Role::Queen,
              is_dead: false,
              color: Color::White,
            }),
            'f' => val.piece = Some(Piece {
              role: Role::Bishop,
              is_dead: false,
              color: Color::White,
            }),
            'g' => val.piece = Some(Piece {
              role: Role::Knight,
              is_dead: false,
              color: Color::White,
            }),
            'h' => val.piece = Some(Piece {
              role: Role::Rook,
              is_dead: false,
              color: Color::White,
            }),
            _ => (),
        }
      } else if num == 8 {
        match alpha {
            'a' => val.piece = Some(Piece {
              role: Role::Rook,
              is_dead: false,
              color: Color::Black,
            }),
            'b' => val.piece = Some(Piece {
              role: Role::Knight,
              is_dead: false,
              color: Color::Black,
            }),
            'c' => val.piece = Some(Piece {
              role: Role::Bishop,
              is_dead: false,
              color: Color::Black,
            }),
            'd' => val.piece = Some(Piece {
              role: Role::King,
              is_dead: false,
              color: Color::Black,
            }),
            'e' => val.piece = Some(Piece {
              role: Role::Queen,
              is_dead: false,
              color: Color::Black,
            }),
            'f' => val.piece = Some(Piece {
              role: Role::Bishop,
              is_dead: false,
              color: Color::Black,
            }),
            'g' => val.piece = Some(Piece {
              role: Role::Knight,
              is_dead: false,
              color: Color::Black,
            }),
            'h' => val.piece = Some(Piece {
              role: Role::Rook,
              is_dead: false,
              color: Color::Black,
            }),
            _ => (),
        }
      }

      map.entry(key).or_insert(val);
    }
  }

  map
}