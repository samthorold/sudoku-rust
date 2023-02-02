use sudoku;

pub fn backtrack(board: &mut sudoku::Board, addr: &sudoku::Addr, depth: i32) -> bool {
    for val in &board.legal_values(&addr) {
        board.set(&addr, *val);
        if depth < 80 {
            let finished = backtrack(board, &board.next_addr(&addr), depth + 1);
            if !finished {
                board.unset(&addr);
            } else {
                return true;
            }
        } else {
            return true;
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    const BOARD_STRING: &str = "\
    070030000\
    000060305\
    001000690\
    500200060\
    102000407\
    060004009\
    013000900\
    209050000\
    000080040\
    ";

    #[test]
    fn test_backtrack() {
        let exp = "\n\
        675|938|214\n\
        928|461|375\n\
        431|527|698\n\
        ---+---+---\n\
        594|273|861\n\
        182|695|437\n\
        367|814|529\n\
        ---+---+---\n\
        813|742|956\n\
        249|156|783\n\
        756|389|142\n\
        ";
        let mut board = sudoku::Board::new(&BOARD_STRING);
        backtrack(&mut board, &sudoku::Addr { row: 1, col: 1 }, 0);
        assert_eq!(board.string(), exp);
    }
}
