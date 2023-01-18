//! Sudoku solver implementation in Rust.

use std::cmp::max;
use std::fmt;

/// Digit box in a Sudoku board.
#[derive(Copy, Clone, Debug)]
pub struct Cell {
    row: u8,
    col: u8,
    sqr: u8,
    value: u8,
    tried: [bool; 9],
    poss: [bool; 9],
    og: bool,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "c{}r{}v{}", self.col, self.row, self.value)
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
    /// * `sqr` - 3x3 box the `Cell` sits in.
    /// * `og` - did the `Cell` have a value when the `Board` was created?
    ///
    /// # Examples
    ///
    /// ```
    /// use sudoku_rust::Cell;
    /// let cell = Cell::new(1, 1, 1, 1, true);
    /// ```
    pub fn new(value: u8, row: u8, col: u8, sqr: u8, og: bool) -> Cell {
        return Cell {
            row,
            col,
            sqr,
            og,
            value,
            tried: [
                false, false, false, false, false, false, false, false, false,
            ],
            poss: [true, true, true, true, true, true, true, true, true],
        };
    }

    pub fn can_set(&self) -> bool {
        return !self.og;
    }

    /// Set the possible values for the Cell given its neighbours.
    ///
    /// # Arguments
    ///
    /// * neighbours - Neighbour cells from the same row, column, and square.
    pub fn set_possibilities(&mut self, neighbours: &Vec<Cell>) {
        for neighbour in neighbours {
            if neighbour.is_set() {
                self.poss[usize::try_from(neighbour.value - 1).unwrap()] = false;
            }
        }
    }

    /// Update Cell value with the next untried value from the possibilities.
    pub fn set_value(&mut self) {
        for (idx, p) in self.poss.iter().enumerate() {
            if (!self.tried[idx]) && *p {
                self.tried[idx] = true;
                self.value = u8::try_from(idx + 1).unwrap();
                return;
            }
        }
        self.value = 0;
        return;
    }

    pub fn is_set(&self) -> bool {
        return self.value > 0;
    }

    /// Reset the Cell - nothing tried and all values possible.
    pub fn reset(&mut self) {
        if self.og {
            panic!("Cannot reset an OG Cell");
        }
        self.tried = [
            false, false, false, false, false, false, false, false, false,
        ];
        self.poss = [true, true, true, true, true, true, true, true, true];
        self.value = 0;
    }
}

/// 9x9 Sudoku board.
pub struct Board {
    cells: [Cell; 81],
    idx: usize,
    direction: i32,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.direction, self.idx)
    }
}

impl Board {
    /// String representation of a Board.
    pub fn string(&self) -> String {
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

    /// Update the value for a single Cell.
    // Really, this is part of the backtrack algorithm
    fn next_generation(&mut self) {
        let mut cell = self.cells[self.idx];
        if !cell.can_set() {
            let idx = usize::try_from(max(
                <i32>::try_from(self.idx).unwrap() + 1 * self.direction,
                0,
            ))
            .unwrap();
            let direction = match idx {
                0 => 1,
                _ => self.direction,
            };
            self.idx = idx;
            self.direction = direction;
            return;
        }
        let neighbours = self.neighbours(&cell);
        cell.set_possibilities(&neighbours);
        cell.set_value();
        if cell.is_set() {
            self.cells[self.idx] = cell;
            self.idx += 1;
            self.direction = 1;
        } else {
            cell.reset();
            self.cells[self.idx] = cell;
            let idx = max(self.idx - 1, 0);
            let direction = match idx {
                0 => 1,
                _ => -1,
            };
            self.idx = idx;
            self.direction = direction;
        }
    }

    /// Is the Board completed?
    // Really, this is part of the backtrack algorithm
    fn is_completed(&self) -> bool {
        return self.idx > 80;
    }

    pub fn new(board_string: &String) -> Board {
        let mut idx: usize = 0;
        let digits: Vec<u8> = board_string
            .chars()
            .map(|s| s.to_digit(10).expect("parse error") as u8)
            .collect();
        let mut cells: Vec<Cell> = Vec::new();
        for row in 1..10 {
            for col in 1..10 {
                let value = digits[idx];
                let og = match value {
                    0 => false,
                    _ => true,
                };
                cells.push(Cell::new(value, row, col, sqr_idx(col, row), og));
                idx += 1;
            }
        }
        let cells_array = match <[Cell; 81]>::try_from(cells) {
            Ok(arr) => arr,
            Err(_) => panic!("Could not convert cells to array."),
        };
        return Board {
            cells: cells_array,
            idx: 0,
            direction: 1,
        };
    }
}
fn sqr_idx(col: u8, row: u8) -> u8 {
    return (col - 1) / 3 + 3 * ((row - 1) / 3) + 1;
}

pub fn backtrack(board: &mut Board) {
    for _ in 0..10000000 {
        if board.is_completed() {
            return;
        }
        board.next_generation();
    }
    panic!("Did not find solution.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backtrack() {
        let exp = "\n\
        534|678|912\n\
        672|195|348\n\
        198|342|567\n\
        ---+---+---\n\
        859|761|423\n\
        426|853|791\n\
        713|924|856\n\
        ---+---+---\n\
        961|537|284\n\
        287|419|635\n\
        345|286|179\n\
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
        let mut board = Board::new(&board_string);
        backtrack(&mut board);
        assert!(board.is_completed());
        assert_eq!(board.string(), exp);
    }
}
