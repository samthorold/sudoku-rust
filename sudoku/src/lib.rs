use std::error::Error;

use backtrack;
use board;

pub struct Config {
    pub method: String,
    pub board_string: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            panic!("Not enough arguments");
        }
        let method = args[1].clone();
        let board_string = args[2].clone();
        Ok(Config {method, board_string })
    }
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut board = board::Board::new(&config.board_string);
    let board_string = board.string();
    println!("{board_string}");

    if config.method == "backtrack" {
        backtrack::backtrack(&mut board, &board::Addr { row: 1, col: 1 }, 0);
    } else {
        println!("Method unknown");
    }

    let board_string = board.string();
    println!("{board_string}");

    Ok(())
}
