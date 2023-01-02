use super::{add_loop_move, Cell, Color, Coordinate, HashMap};

pub fn rook_move(
    from: &Coordinate,
    status: &HashMap<String, Cell>,
    color: &Color,
) -> Vec<Coordinate> {
    let x = from.x();
    let y = from.y();

    let mut to = vec![];

    let mut dx = x;
    let mut dy = y;
    loop {
        dy += 1;
        if dy > 8 {
            break;
        } else {
            if !add_loop_move(&status, &mut to, dx, dy, color) {
                break;
            }
        }
    }

    dx = x;
    dy = y;
    loop {
        dy -= 1;
        if dy < 1 {
            break;
        } else {
            if !add_loop_move(&status, &mut to, dx, dy, color) {
                break;
            }
        }
    }

    dx = x;
    dy = y;
    loop {
        dx -= 1;
        if dx < 1 {
            break;
        } else {
            if !add_loop_move(&status, &mut to, dx, dy, color) {
                break;
            }
        }
    }

    dx = x;
    dy = y;
    loop {
        dx += 1;
        if dx > 8 {
            break;
        } else {
            if !add_loop_move(&status, &mut to, dx, dy, color) {
                break;
            }
        }
    }

    to
}
