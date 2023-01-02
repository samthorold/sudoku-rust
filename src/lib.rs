//! Sudoku solver implementation in Rust.

use std::cmp::max;

/// Digit box in a Sudoku board.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
struct Cell {
    row: u8,
    col: u8,
    sqr: u8,
    value: u8,
    tried: [bool; 9],
    og: bool,
}

impl Cell {
    /// New Cell with the next untried value from the possibilities.
    ///
    /// # Arguments
    ///
    /// * `poss` - Possible values given the neighbouring Cell values.
    fn next_value(&self, poss: Vec<u8>) -> Cell {
        let mut tried = self.tried;
        for value in poss {
            let idx = <usize>::try_from(value - 1).unwrap();
            if !tried[idx] {
                tried[idx] = true;
                return Cell {
                    value,
                    tried,
                    ..*self
                };
            }
        }
        return Cell { value: 0, ..*self };
    }
}

/// 9x9 Sudoku board.
#[derive(Copy, Clone)]
pub struct Board {
    cells: [Cell; 81],
    idx: usize,
    direction: i32,
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
    /// *cell* - Target Cell to return neighbouring Cells for.
    fn neighbours(&self, cell: Cell) -> Vec<Cell> {
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

    /// Possible Cell values given the neighbouring Cell.
    ///
    /// # Arguments
    ///
    /// *cell* - Target Cell to return possible values for.
    fn possibilities(&self, cell: Cell) -> Vec<u8> {
        let mut poss: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        for neighbour in self.neighbours(cell) {
            if neighbour.value > 0 {
                let idx = match poss.binary_search(&neighbour.value) {
                    Ok(idx) => idx,
                    Err(_) => continue,
                };
                poss.remove(idx);
            }
        }
        return poss;
    }

    /// Update the value for a single Cell and return a new Board.
    fn next_generation(&self) -> Board {
        let cell: Cell = self.cells[self.idx];
        if cell.og {
            let idx = usize::try_from(max(
                <i32>::try_from(self.idx).unwrap() + 1 * self.direction,
                0,
            ))
            .unwrap();
            let direction = match idx {
                0 => 1,
                _ => self.direction,
            };
            return Board {
                idx,
                direction,
                ..*self
            };
        }
        let poss: Vec<u8> = self.possibilities(cell);
        let mut new_cell: Cell = cell.next_value(poss);
        let mut new_cells: [Cell; 81] = self.cells;
        if new_cell.value > 0 {
            new_cells[self.idx] = new_cell;
            return Board {
                cells: new_cells,
                idx: self.idx + 1,
                direction: 1,
            };
        } else {
            new_cell.tried = [
                false, false, false, false, false, false, false, false, false,
            ];
            new_cells[self.idx] = new_cell;
            let idx = max(self.idx - 1, 0);
            let direction = match idx {
                0 => 1,
                _ => -1,
            };
            return Board {
                cells: new_cells,
                idx,
                direction,
            };
        }
    }

    /// Is the Board completed?
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
                cells.push(Cell {
                    value,
                    row,
                    col,
                    sqr: sqr_idx(col, row),
                    tried: [
                        false, false, false, false, false, false, false, false, false,
                    ],
                    og,
                });
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

pub fn backtrack(board: Board) -> Board {
    let mut b = board;
    for _ in 0..1000000 {
        if b.is_completed() {
            break;
        }
        b = b.next_generation();
    }
    return b;
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
        let board = Board::new(&board_string);
        let solution = backtrack(board);
        assert!(solution.is_completed());
        assert_eq!(solution.string(), exp);
    }
}
