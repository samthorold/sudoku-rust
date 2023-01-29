//! Sudoku game

use std::fmt;

/// Digit box in a Sudoku board.
#[derive(Copy, Clone, Debug)]
pub struct Cell {
    row: u8,
    col: u8,
    sqr: u8,
    val: u8,
    og: bool,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "c{}r{}v{}", self.col, self.row, self.val)
    }
}

impl Cell {
    /// Create a new `Cell`.
    ///
    /// Arguments are 0-indexed.
    ///
    /// # Arguments
    ///
    /// * `value` - value, 0-9 where 0 means unset.
    /// * `row` - row or y-coordinate on the board.
    /// * `col` - col or x-coordinate on the board.
    /// * `og` - did the `Cell` have a value when the `Board` was created?
    ///
    /// # Examples
    ///
    /// ```
    /// use sudoku::Cell;
    /// let cell = Cell::new(1, 1, 1);
    /// ```
    pub fn new(val: u8, row: u8, col: u8) -> Cell {
        let og = match val {
            0 => false,
            _ => true,
        };
        return Cell {
            row,
            col,
            sqr: sqr_idx(col, row),
            og,
            val,
        };
    }

    pub fn can_set(&self) -> bool {
        return !self.og;
    }

    pub fn set(&mut self, val: u8) {
        if !self.can_set() {
            panic!("Cannot set an OG Cell");
        }
        self.val = val;
    }

    pub fn is_set(&self) -> bool {
        return self.val > 0;
    }

    pub fn unset(&mut self) {
        if !self.can_set() {
            panic!("Cannot reset an OG Cell");
        }
        self.val = 0;
    }
}

/// 9x9 Sudoku board.
pub struct Board {
    cells: [Cell; 81],
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Board {
    pub fn new(board_string: &String) -> Board {
        let mut idx: usize = 0;
        let digits: Vec<u8> = board_string
            .chars()
            .map(|s| s.to_digit(10).expect("parse error") as u8)
            .collect();
        let mut cells: Vec<Cell> = Vec::new();
        for row in 1..10 {
            for col in 1..10 {
                let val = digits[idx];
                cells.push(Cell::new(val, row, col));
                idx += 1;
            }
        }
        let cells_array = match <[Cell; 81]>::try_from(cells) {
            Ok(arr) => arr,
            Err(_) => panic!("Could not convert cells to array."),
        };
        return Board { cells: cells_array };
    }

    /// String representation of a Board.
    pub fn string(&self) -> String {
        let important_idx: [u8; 2] = [3, 6];
        let mut s: String = String::from("\n");
        for cell in self.cells {
            s.push_str(&cell.val.to_string());
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

    /// Cells sharing a column, row, or square.
    ///
    /// # Arguments
    ///
    /// *cell* - Target `Cell` to return neighbouring `Cells` for.
    pub fn neighbours(&self, cell: &Cell) -> Vec<Cell> {
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

fn sqr_idx(col: u8, row: u8) -> u8 {
    return (col - 1) / 3 + 3 * ((row - 1) / 3) + 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_board() {
        let exp = "\n\
        530|070|000\n\
        600|195|000\n\
        098|000|060\n\
        ---+---+---\n\
        800|060|003\n\
        400|803|001\n\
        700|020|006\n\
        ---+---+---\n\
        060|000|280\n\
        000|419|005\n\
        000|080|079\n\
        ";
        let board_string = String::from(
            "\
            530070000\
            600195000\
            098000060\
            800060003\
            400803001\
            700020006\
            060000280\
            000419005\
            000080079\
            ",
        );
        let board = Board::new(&board_string);
        assert_eq!(board.string(), exp);
    }
}
