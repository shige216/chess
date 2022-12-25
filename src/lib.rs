struct Coordinate(u8, u8);

struct Cell {
  coordinate: Coordinate,
  piece: Option<Box<Piece>>,
}

struct Piece {
  role: Role,
  is_dead: bool,
  stand_at: Option<Cell>,
}

impl Piece {
  fn can_move_to(&self, coordinate: Coordinate) -> bool {
    true
  }

  fn move_to(&mut self, coordinate: Coordinate) {

  }
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
  status: Vec<Cell>,
}

impl Board {
  fn init(&self) -> Board {

    Board { status: vec![] }
  }
}

fn init() -> Vec<Cell> {
  let a1 = Cell {
    coordinate: Coordinate(0, 0),
    piece: None,
  };
  let a2 = Cell {
    coordinate: Coordinate(1, 0),
    piece: None,
  };
  let a3 = Cell {
    coordinate: Coordinate(2, 0),
    piece: None,
  };
  let a4 = Cell {
    coordinate: Coordinate(3, 0),
    piece: None,
  };
  let a5 = Cell {
    coordinate: Coordinate(4, 0),
    piece: None,
  };
  let a6 = Cell {
    coordinate: Coordinate(5, 0),
    piece: None,
  };
  let a7 = Cell {
    coordinate: Coordinate(6, 0),
    piece: None,
  };
  let a8 = Cell {
    coordinate: Coordinate(7, 0),
    piece: None,
  };

  let b1 = Cell {
    coordinate: Coordinate(0, 1),
    piece: None,
  };
  let b2 = Cell {
    coordinate: Coordinate(1, 1),
    piece: None,
  };
  let b3 = Cell {
    coordinate: Coordinate(2, 1),
    piece: None,
  };
  let b4 = Cell {
    coordinate: Coordinate(3, 1),
    piece: None,
  };
  let b5 = Cell {
    coordinate: Coordinate(4, 1),
    piece: None,
  };
  let b6 = Cell {
    coordinate: Coordinate(5, 1),
    piece: None,
  };
  let b7 = Cell {
    coordinate: Coordinate(6, 1),
    piece: None,
  };
  let b8 = Cell {
    coordinate: Coordinate(7, 1),
    piece: None,
  };
  vec![]
}