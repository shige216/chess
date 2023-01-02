use super::{Coordinate, Cell, Color, HashMap};

pub fn bishop_move(from: Coordinate, status: &HashMap<String, Cell>, color: &Color) -> Vec<Coordinate> {
    let x = from.x();
    let y = from.y();

    let mut to = vec![];

    let mut dx = x;
    let mut dy = y;
    loop {
        dx -= 1;
        dy -= 1;
        if dx < 1 || dy < 1 {
            break;
        } else {
            if !add_bishop_move(status, &mut to, dx, dy, color) {
                break;
            }
        }
    }

    dx = x;
    dy = y;
    loop {
        dx -= 1;
        dy += 1;
        if dx < 1 || dy > 8 {
            break;
        } else {
            if !add_bishop_move(status, &mut to, dx, dy, color) {
                break;
            }
        }
    }

    dx = x;
    dy = y;
    loop {
        dx += 1;
        dy += 1;
        if dx > 8 || dy > 8 {
            break;
        } else {
            if !add_bishop_move(status, &mut to, dx, dy, color) {
                break;
            }
        }
    }

    dx = x;
    dy = y;
    loop {
        dx += 1;
        dy -= 1;
        if dx > 8 || dy < 1 {
            break;
        } else {
            if !add_bishop_move(status, &mut to, dx, dy, color) {
                break;
            }
        }
    }

    to
}

fn add_bishop_move(status: &HashMap<String, Cell>, to: &mut Vec<Coordinate>, dx: u8, dy: u8, color: &Color) -> bool {
    let key = format!("{}{}", dx, dy);
    if let Some(c) = status.get(&key) {
        match &c.piece {
            Some(p) => {
                match color {
                    Color::White => {
                        match p.color {
                            Color::White => {
                                return false;
                            },
                            Color::Black => {
                                if let Ok(c) = Coordinate::new(dx, dy) {
                                    to.push(c);
                                    return false;
                                }
                            }
                        }
                    },
                    Color::Black => {
                        match p.color {
                            Color::Black => {
                                return false;
                            },
                            Color::White => {
                                if let Ok(c) = Coordinate::new(dx, dy) {
                                    to.push(c);
                                    return false;
                                }
                            }
                        }
                    },
                }
            },
            None => {
                if let Ok(c) = Coordinate::new(dx, dy) {
                    to.push(c);
                }
            }
        }
    }

    return true;
}
