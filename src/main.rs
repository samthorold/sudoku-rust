#[derive(Copy, Clone, Debug)]
struct Cell {
    col: u8,
    row: u8,
    sqr: u8,
    value: u8,
}

impl Cell {
    fn with_value(self, value: u8) -> Cell {
        return Cell { value, ..self };
    }
}

#[derive(Copy, Clone)]
struct Board {
    cells: [Cell; 81],
}

impl Board {
    fn string(self) -> String {
        let important_idx: [u8; 2] = [3, 6];
        let mut s: String = String::from("\n");
        for cell in self.cells {
            s.push_str(&cell.value.to_string());
            if important_idx.contains(&cell.col) {
                s.push_str("|")
            }
            if cell.col == 9 {
                s.push_str("\n");
                if important_idx.contains(&cell.row) {
                    s.push_str("---+---+---\n");
                }
            }
        }
        return s;
    }
}

impl Board {
    fn neighbours(self, cell: Cell) -> Vec<Cell> {
        let mut nghs = Vec::new();
        for friend in self.cells {
            if (cell.row == friend.row) & (cell.col == friend.col) {
                continue;
            }
            if (cell.row == friend.row) | (cell.col == friend.col) | (cell.sqr == friend.sqr) {
                nghs.push(friend)
            }
        }
        return nghs;
    }
}

fn new_board() -> Board {
    let mut cells: Vec<Cell> = Vec::new();
    for row in 1..10 {
        for col in 1..10 {
            cells.push(Cell {
                value: 0,
                row,
                col,
                sqr: sqr_idx(col, row),
            })
        }
    }
    let cells_array = match <[Cell; 81]>::try_from(cells) {
        Ok(arr) => arr,
        Err(_) => panic!("Could not convert cells to array."),
    };
    return Board { cells: cells_array };
}

fn sqr_idx(col: u8, row: u8) -> u8 {
    return (col - 1) / 3 + 3 * ((row - 1) / 3) + 1;
}

fn main() {
    println!("Sudoku (Rust)");
    let board = new_board();
    let board_string = board.string();
    println!("{board_string}");
    let nghs = board.neighbours(Cell {
        value: 0,
        col: 1,
        row: 1,
        sqr: 1,
    });
    println!("{nghs:#?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_with_value() {
        let col: u8 = 1;
        let row: u8 = 1;
        let sqr: u8 = 1;
        let cell = Cell {
            value: 0,
            col,
            row,
            sqr,
        };

        let new_cell = cell.with_value(1);
        assert_eq!(1, new_cell.value);
        assert_eq!(1, new_cell.col);
        assert_eq!(1, new_cell.row);
        assert_eq!(1, new_cell.sqr);
    }

    #[test]
    fn test_new_board() {
        let board = new_board();
    }

    #[test]
    fn test_sqr_idx() {
        let exp: u8 = 1;
        let got: u8 = sqr_idx(1, 1);
        assert_eq!(exp, got);

        let exp: u8 = 9;
        let got: u8 = sqr_idx(9, 9);
        assert_eq!(exp, got);

        let exp: u8 = 4;
        let got: u8 = sqr_idx(1, 4);
        assert_eq!(exp, got);

        let exp: u8 = 8;
        let got: u8 = sqr_idx(5, 9);
        assert_eq!(exp, got);
    }
}
