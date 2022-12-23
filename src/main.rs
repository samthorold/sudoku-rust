struct Cell {
    value: u8,
}

// impl Cell {
//     fn clone(&self) -> Cell {
//         return Cell { value: self.value };
//     }
// }

struct Board {
    cells: Vec<Cell>,
}

impl Board {
    fn string(&self) -> String {
        let important_idx: [u8; 2] = [3, 6];
        let mut col: u8 = 1;
        let mut row: u8 = 1;
        let mut s: String = String::from("\n");
        let cells = &self.cells[0..81];
        for cell in cells {
            s.push_str(&cell.value.to_string());
            if important_idx.contains(&col) {
                s.push_str("|")
            }
            if col == 9 {
                s.push_str("\n");
                if important_idx.contains(&row) {
                    s.push_str("---+---+---\n");
                }
                row = row + 1;
                col = 1;
            } else {
                col = col + 1;
            }
        }
        return s;
    }
}

fn new_board() -> Board {
    let mut cells: Vec<Cell> = Vec::new();
    for _ in 0..81 {
        cells.push(Cell { value: 0 })
    }
    return Board { cells };
}

fn main() {
    println!("Sudoku (Rust)");
    let board = new_board();
    let board_string = board.string();
    println!("{board_string}")
}
