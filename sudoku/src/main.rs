use std::env;
use std::process;

use sudoku;

fn main() {
    println!("Sudoku (Rust)");
    let args: Vec<String> = env::args().collect();

    let config = sudoku::Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing args {err}");
        process::exit(1);
    });

    if let Err(e) = sudoku::run(config) {
        eprintln!("App err: {e}");
        process::exit(1);
    }
}
