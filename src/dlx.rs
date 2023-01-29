//! Dancing links implementation in Rust.
//!
//! 0010110
//! 1001001
//! 0110010
//! 1001000
//! 0100001
//! 0001101

/// Column in the exact cover problem.
struct Column {
    is_root: bool,
    l: Column,
    r: Column,
    cell: Cell,
}

/// Occupied cell in the exact cover problem.
struct Cell {
    c: Column,
    u: Cell,
    r: Cell,
    d: Cell,
    l: Cell,
}

fn choose_col(root: Column) -> Column {
    return root.r
}

fn cover(col: Column) -> bool {
    return true
}

pub fn dlx(root: Column, soln: [u64; 81], k: u64) -> [u64; 81] {
    if root.r == root {
        return soln;
    }
    let col = choose_col(root);
    cover(col);
    // while (row := cell.d)
    //   add row idx to the soln
    //   while (next_col := row.r)
    //     cover(next_col)
    //   dlx(k+1)
}
