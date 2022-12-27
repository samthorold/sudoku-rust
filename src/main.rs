use std::env;

use sudoku_rust;

fn main() {
    println!("Sudoku (Rust)");
    let args: Vec<String> = env::args().collect();
    let board_string = &args[1];

    let board = sudoku_rust::Board::new(board_string);
    let board_string = board.string();
    println!("{board_string}");

    let board = sudoku_rust::solve(board);

    let board_string = board.string();
    println!("{board_string}");
}
