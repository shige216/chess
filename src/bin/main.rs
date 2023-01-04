extern crate chess;

use std::io::{self, Write};

use chess::{Board, Coordinate, print_error};


fn main() {
    println!("Welcome to CL Chess!");
    println!("You can type commands as they follow.");
    println!("show: show current board.");
    println!("route [from]: show routes of [from] piece");
    println!("move [from] [to]: move a piece from [from] to [to]. [from] or [to] is xy (1<=(x, y)<=8)");
    let mut board = Board::init();

    loop {
        print!("chess> ");
        io::stdout().flush().expect("flush failed");
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("read line failed");

        if buffer.starts_with("show") {
            board.show();
        } else if buffer.starts_with("route") {
            let mut words = buffer.split_whitespace();
            words.next();
            let from = words.next();
            match from {
                Some(s) => {
                    let mut chars = s.chars();
                    let x = chars.next();
                    match x {
                        Some(x) => {
                            let y = chars.next();
                            match y {
                                Some(y) => {
                                    let x = x.to_digit(10);
                                    match x {
                                        Some(x) => {
                                            let y = y.to_digit(10);
                                            match y {
                                                Some(y) => {
                                                    let coordinate = Coordinate::new(x, y);
                                                    match coordinate {
                                                        Ok(c) => {
                                                            let options = board.can_move(c);
                                                            match options {
                                                                Ok(cs) => {
                                                                    for c in cs {
                                                                        print!("{}{}, ", c.x(), c.y());
                                                                    }
                                                                }
                                                                Err(e) => {
                                                                    print_error(e);
                                                                }
                                                            }
                                                        }
                                                        Err(_) => println!("Invalid parameter.")
                                                    }
                                                }
                                                None => println!("Invalid parameter.")
                                            }
                                        }
                                        None => println!("Invalid parameter.")
                                    }
                                }
                                None => println!("Invalid parameter.")
                            }
                        }
                        None => println!("Invalid parameter.")
                    }
                }
                None => println!("Invalid parameter.")
            }
        } else if buffer.starts_with("move") {

        }
    }
}
