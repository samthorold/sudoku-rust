use std::env;

use backtrack;
use board;

fn main() {
    println!("Sudoku (Rust)");
    let args: Vec<String> = env::args().collect();
    let board_string = &args[1];

    let mut board = board::Board::new(board_string);
    let board_string = board.string();
    println!("{board_string}");

    backtrack::backtrack(&mut board, &board::Addr { row: 1, col: 1 }, 0);

    let board_string = board.string();
    println!("{board_string}");
}
