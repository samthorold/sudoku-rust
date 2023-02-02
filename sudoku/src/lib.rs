//! Sudoku game

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;

/// Address of a Cell on a sudoku board.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Addr {
    pub row: u8,
    pub col: u8,
}

/// Digit box in a Sudoku board.
#[derive(Copy, Clone, Debug, Eq)]
pub struct Cell {
    addr: Addr,
    val: u8,
    og: bool,
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        return self.addr.eq(&other.addr);
    }
}

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.addr.cmp(&other.addr);
    }
}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Cell {
    pub fn new(val: u8, row: u8, col: u8) -> Cell {
        return Cell {
            addr: Addr { row, col },
            og: match val {
                0 => false,
                _ => true,
            },
            val,
        };
    }

    pub fn can_set(&self) -> bool {
        return !self.og;
    }

    pub fn set(&mut self, val: u8) {
        if self.can_set() {
            self.val = val;
        }
    }

    pub fn unset(&mut self) {
        if self.can_set() {
            self.val = 0;
        }
    }

    pub fn is_set(&self) -> bool {
        return self.val > 0;
    }
}

/// 9x9 Sudoku board.
pub struct Board {
    cells: HashMap<Addr, Cell>,
    nhbrs: HashMap<Addr, [Addr; 20]>,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Board {
    pub fn new(board_string: &str) -> Board {
        let mut idx: usize = 0;
        let digits: Vec<u8> = board_string
            .chars()
            .map(|s| s.to_digit(10).expect("parse error") as u8)
            .collect();
        let mut cells = HashMap::new();
        for row in 1..10 {
            for col in 1..10 {
                let val = digits[idx];
                let cell = Cell::new(val, row, col);
                cells.insert(cell.addr, cell);
                idx += 1;
            }
        }
        let mut nhbrs = HashMap::new();
        for (addr, cell) in &cells {
            let cell_nhbrs = match <[Addr; 20]>::try_from(neighbours(&cell, &cells)) {
                Ok(vec) => vec,
                Err(o) => panic!("Could not create neighbours for {:?} {:?}", cell.addr, o),
            };
            nhbrs.insert(*addr, cell_nhbrs);
        }

        return Board { cells, nhbrs };
    }

    /// String representation of a Board.
    pub fn string(&self) -> String {
        let mut cells = Vec::new();
        let important_idx: [u8; 2] = [3, 6];
        let mut s: String = String::from("\n");
        for (_, cell) in &self.cells {
            cells.push(cell);
        }
        cells.sort();
        for cell in cells {
            s.push_str(&cell.val.to_string());
            if important_idx.contains(&cell.addr.col) {
                s.push_str("|")
            }
            if cell.addr.col == 9 {
                s.push_str("\n");
                if important_idx.contains(&cell.addr.row) {
                    s.push_str("---+---+---\n");
                }
            }
        }
        return s;
    }

    pub fn next_addr(&self, addr: &Addr) -> Addr {
        if addr.col == 9 {
            return Addr {
                row: addr.row + 1,
                col: 1,
            };
        }
        return Addr {
            row: addr.row,
            col: addr.col + 1,
        };
    }

    pub fn prev_addr(&self, addr: &Addr) -> Addr {
        if addr.col == 1 {
            return Addr {
                row: addr.row - 1,
                col: 9,
            };
        }
        return Addr {
            row: addr.row,
            col: addr.col - 1,
        };
    }

    pub fn neighbours(&self, addr: &Addr) -> &[Addr; 20] {
        return self.nhbrs.get(addr).expect("No addr {addr:?}");
    }

    pub fn legal_values(&self, addr: &Addr) -> Vec<u8> {
        if !self.can_set(addr) {
            let cell = &self.cells.get(addr).expect("No addr {nghbr:?}");
            return vec![cell.val];
        }
        let mut vals = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        for nghbr in self.neighbours(addr) {
            let cell = &self.cells.get(nghbr).expect("No addr {nghbr:?}");
            if vals.contains(&cell.val) {
                vals.remove(
                    vals.binary_search(&cell.val)
                        .expect("No value {cell.val:?} at addr {addr:?}"),
                );
            }
        }
        return vals;
    }

    pub fn can_set(&self, addr: &Addr) -> bool {
        let cell = *self.cells.get(addr).expect("No addr {addr:?}");
        return cell.can_set();
    }

    pub fn set(&mut self, addr: &Addr, val: u8) {
        let mut cell = *self.cells.get(addr).expect("No addr {addr:?}");
        if !self.can_set(addr) {
            return;
        }
        cell.set(val);
        self.cells.insert(*addr, cell);
    }

    pub fn unset(&mut self, addr: &Addr) {
        let mut cell = *self.cells.get(addr).expect("No addr {addr:?}");
        cell.unset();
        self.cells.insert(*addr, cell);
    }
}

fn neighbours(cell: &Cell, cells: &HashMap<Addr, Cell>) -> Vec<Addr> {
    let mut nghs = Vec::new();
    for (addr, friend) in cells {
        if cell.addr == *addr {
            continue;
        }
        let shared_col = cell.addr.col == friend.addr.col;
        let shared_row = cell.addr.row == friend.addr.row;
        let shared_sqr =
            sqr_idx(cell.addr.col, cell.addr.row) == sqr_idx(friend.addr.col, friend.addr.row);
        if shared_row | shared_col | shared_sqr {
            nghs.push(*addr)
        }
    }
    return nghs;
}

fn sqr_idx(col: u8, row: u8) -> u8 {
    return (col - 1) / 3 + 3 * ((row - 1) / 3) + 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    const BOARD_STRING: &str = "\
    530070000\
    600195000\
    098000060\
    800060003\
    400803001\
    700020006\
    060000280\
    000419005\
    000080079\
    ";

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
        let board = Board::new(&BOARD_STRING);
        assert_eq!(board.string(), exp);
    }

    #[test]
    fn test_nhbrs() {
        let exp: [Addr; 20] = [
            Addr { row: 1, col: 2 },
            Addr { row: 1, col: 3 },
            Addr { row: 1, col: 4 },
            Addr { row: 1, col: 5 },
            Addr { row: 1, col: 6 },
            Addr { row: 1, col: 7 },
            Addr { row: 1, col: 8 },
            Addr { row: 1, col: 9 },
            Addr { row: 2, col: 1 },
            Addr { row: 2, col: 2 },
            Addr { row: 2, col: 3 },
            Addr { row: 3, col: 1 },
            Addr { row: 3, col: 2 },
            Addr { row: 3, col: 3 },
            Addr { row: 4, col: 1 },
            Addr { row: 5, col: 1 },
            Addr { row: 6, col: 1 },
            Addr { row: 7, col: 1 },
            Addr { row: 8, col: 1 },
            Addr { row: 9, col: 1 },
        ];
        let board = Board::new(&BOARD_STRING);
        let mut got = *board.neighbours(&Addr { row: 1, col: 1 });
        got.sort();
        assert_eq!(got.iter().eq(exp.iter()), true);
    }

    #[test]
    fn test_next_addr() {
        let board = Board::new(&BOARD_STRING);

        let mut got = board.next_addr(&Addr { row: 1, col: 1 });
        assert_eq!(got, Addr { row: 1, col: 2 });

        got = board.next_addr(&Addr { row: 1, col: 9 });
        assert_eq!(got, Addr { row: 2, col: 1 });

        got = board.next_addr(&Addr { row: 5, col: 9 });
        assert_eq!(got, Addr { row: 6, col: 1 });
    }

    #[test]
    fn test_prev_addr() {
        let board = Board::new(&BOARD_STRING);

        let mut got = board.prev_addr(&Addr { row: 9, col: 9 });
        assert_eq!(got, Addr { row: 9, col: 8 });

        got = board.prev_addr(&Addr { row: 6, col: 1 });
        assert_eq!(got, Addr { row: 5, col: 9 });
    }

    #[test]
    fn test_legal_values() {
        let mut board = Board::new(&BOARD_STRING);

        let mut got = board.legal_values(&Addr { row: 1, col: 3 });
        assert_eq!(got, vec![1, 2, 4]);

        board.set(&Addr { row: 2, col: 2 }, 4);

        got = board.legal_values(&Addr { row: 1, col: 3 });
        assert_eq!(got, vec![1, 2]);

        board.unset(&Addr { row: 2, col: 2 });

        got = board.legal_values(&Addr { row: 1, col: 3 });
        assert_eq!(got, vec![1, 2, 4]);
    }
}
