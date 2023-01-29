use std::env;

use sudoku;

fn main() {
    println!("Sudoku (Rust)");
    let args: Vec<String> = env::args().collect();
    let board_string = &args[1];

    let board = sudoku::Board::new(board_string);
    let board_string = board.string();
    println!("{board_string}");

    //sudoku::backtrack(&mut board);

    let board_string = board.string();
    println!("{board_string}");
}
