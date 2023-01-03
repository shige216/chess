extern crate chess;

use std::io;

use chess::Board;


fn main() {
    println!("Welcome to CL Chess!");
    println!("You can type commands as they follow.");
    println!("show: show current board.");
    println!("move [from] [to]: move a piece from [from] to [to]. [from] or [to] is (x, y) (1<=(x, y)<=8)");
    let mut board = Board::init();

    loop {
        print!("chess> ");
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("read line failed");

    }
}
