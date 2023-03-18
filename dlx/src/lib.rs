use std::{thread, time::Duration};

pub struct A {
    root: Col,
    cols: Vec<Col>,
    nodes: Vec<Vec<Node>>,
}

impl A {
    fn get_node(&self, addr: NodeAddr) -> Node {
        if (addr.r < 0) | (addr.c < 0) {
            panic!("Invalid node addr {}{}", addr.r, addr.c);
        }
        self.nodes[addr.r as usize][addr.c as usize]
    }

    fn set_node(&mut self, node: Node) {
        self.nodes[node.addr.r as usize][node.addr.c as usize] = node;
    }

    fn get_col(&self, addr: ColAddr) -> Col {
        match addr.c {
            c if c >= 0 => self.cols[c as usize],
            _ => self.root,
        }
    }

    fn set_col(&mut self, col: Col) {
        self.cols[col.addr.c as usize] = col;
    }

    pub fn choose_col(&self) -> Col {
        let mut s = i32::MAX;
        let mut a = ColAddr::new();
        for col in &self.cols {
            if col.s < s {
                a = col.addr;
                s = col.s;
            }
        }
        self.get_col(a)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ColAddr {
    pub c: i32,
}

impl ColAddr {
    fn new() -> ColAddr {
        ColAddr { c: -1 }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Col {
    root: bool,
    pub addr: ColAddr,
    l: ColAddr,
    r: ColAddr,
    u: NodeAddr,
    d: NodeAddr,
    s: i32,
}

impl Col {
    fn new(c: i32) -> Col {
        Col {
            root: false,
            addr: ColAddr { c },
            l: ColAddr::new(),
            r: ColAddr::new(),
            u: NodeAddr::new(),
            d: NodeAddr::new(),
            s: 0,
        }
    }

    fn incr_s(self) -> Col {
        Col {
            s: self.s + 1,
            ..self
        }
    }

    fn decr_s(self) -> Col {
        Col {
            s: self.s - 1,
            ..self
        }
    }

    fn set_l(self, l: Col) -> Col {
        Col { l: l.addr, ..self }
    }

    fn set_r(self, r: Col) -> Col {
        Col { r: r.addr, ..self }
    }

    fn set_u(self, u: Node) -> Col {
        Col { u: u.addr, ..self }
    }

    fn set_d(self, d: Node) -> Col {
        Col { d: d.addr, ..self }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct NodeAddr {
    r: i32,
    c: i32,
}

impl NodeAddr {
    fn new() -> NodeAddr {
        NodeAddr { r: -1, c: -1 }
    }
}

#[derive(Copy, Clone, Debug)]
struct Node {
    addr: NodeAddr,
    is_legit: bool,
    c: ColAddr, // this is self.addr.c
    l: NodeAddr,
    r: NodeAddr,
    u: NodeAddr,
    d: NodeAddr,
}

impl Node {
    fn new(r: i32, c: i32) -> Node {
        Node {
            addr: NodeAddr { r, c },
            is_legit: false,
            c: ColAddr::new(),
            l: NodeAddr::new(),
            r: NodeAddr::new(),
            u: NodeAddr::new(),
            d: NodeAddr::new(),
        }
    }

    fn set_c(self, c: Col) -> Node {
        Node { c: c.addr, ..self }
    }

    fn set_l(self, l: Node) -> Node {
        Node { l: l.addr, ..self }
    }

    fn set_r(self, r: Node) -> Node {
        Node { r: r.addr, ..self }
    }

    fn set_u(self, u: Node) -> Node {
        Node { u: u.addr, ..self }
    }

    fn set_d(self, d: Node) -> Node {
        Node { d: d.addr, ..self }
    }
}

/// Create a problem, `A`, from a matrix of 0s and 1s.
pub fn from_matrix(matrix: &Vec<Vec<u8>>) -> A {
    // the "root" col is used to start off each iteration of covering
    let mut root = Col::new(0);
    root.root = true;

    let mut cols = <Vec<Col>>::new();
    let mut nodes = <Vec<Vec<Node>>>::new();

    for (r, row) in matrix.into_iter().enumerate() {
        let mut row_nodes = <Vec<Node>>::new();

        for (c, val) in row.into_iter().enumerate() {
            if r == 0 {
                cols.push(Col::new(c as i32));
                if c > 0 {
                    cols[c] = cols[c].set_l(cols[c - 1]);
                    cols[c - 1] = cols[c - 1].set_r(*cols.last().unwrap());
                }
            }
            row_nodes.push(Node::new(r as i32, c as i32));
            if *val == 1 {
                // println!("{r}{c}");
                row_nodes[c].is_legit = true;

                cols[c] = cols[c].incr_s();
                row_nodes[c] = row_nodes[c].set_c(cols[c]);

                if cols[c].d.r < 0 {
                    // column's down addr will be initialised to -1, -1
                    // so down.right -1 means no down addr set yet for a column
                    cols[c] = cols[c].set_d(row_nodes[c]);
                } else {
                    // otherwise we need to find the last legit node
                    // and connect the last legit node and the current node
                    let mut u = r as i32;
                    loop {
                        u = u - 1;
                        if u < 0 {
                            break;
                        }
                        if nodes[u as usize][c].is_legit {
                            nodes[u as usize][c] =
                                nodes[u as usize][c].set_d(*row_nodes.last().unwrap());
                            row_nodes[c] = row_nodes[c].set_u(nodes[u as usize][c]);
                            // println!("  u={u}, d={r}");
                            break;
                        }
                    }
                }

                if c > 0 {
                    let mut l = c as i32;
                    loop {
                        l = l - 1;
                        if l < 0 {
                            break;
                        }
                        if row_nodes[l as usize].is_legit {
                            row_nodes[c] = row_nodes[c].set_l(row_nodes[l as usize]);
                            row_nodes[l as usize] = row_nodes[l as usize].set_r(row_nodes[c]);
                            // println!("  l={l}, r={c}");
                            break;
                        }
                    }
                }
            }
        }

        // get to the rightmost legit node
        let mut r_ = row_nodes.len();
        loop {
            r_ = r_ - 1;
            if row_nodes[r_].is_legit {
                break;
            }
        }

        // get to the leftmost legit node
        let mut l_ = 0;
        loop {
            if row_nodes[l_].is_legit {
                break;
            }
            l_ = l_ + 1;
        }

        row_nodes[r_] = row_nodes[r_].set_r(row_nodes[l_]);
        row_nodes[l_] = row_nodes[l_].set_l(row_nodes[r_]);

        nodes.push(row_nodes);
    }

    let h = nodes.len();
    let w = cols.len();

    for j in 0..w {
        // println!("Setting c={j} ud ...");
        let u = cols[j].d;
        for i in (0..h).rev() {
            if nodes[i][j].is_legit {
                nodes[i][j] = nodes[i][j].set_d(nodes[u.r as usize][j]);
                nodes[u.r as usize][j] = nodes[u.r as usize][j].set_u(nodes[i][j]);
                cols[j] = cols[j].set_u(nodes[i][j]);
                // println!("  u={i}, d={}", u.r);
                break;
            }
        }
    }

    root = root.set_r(cols[0]);
    root = root.set_l(*cols.last().unwrap());

    // TODO: does this prevent wrapping around?
    // -1 is not an address
    cols[0] = cols[0].set_l(root);
    cols[w - 1] = cols[w - 1].set_r(root);
    // END TODO

    return A { root, cols, nodes };
}

pub fn cover(a: &mut A, c: ColAddr) {
    let col = a.get_col(c);
    let og_size = col.s;
    if col.s == 0 {
        println!("Col {} as no nodes", col.addr.c);
        return;
    }
    let l = a.get_col(col.l);
    let r = a.get_col(col.r);
    a.set_col(l.set_r(r));
    a.set_col(r.set_l(l));

    let cover_node = a.get_node(col.d);
    let mut cn = cover_node;
    // let mut cinit = true;
    let mut ri = 1;
    while ri <= og_size {
        // cinit = false;
        let node = a.get_node(cn.r);
        let mut n = node;
        let mut ninit = true;
        while ninit | (n.addr != node.addr) {
            ninit = false;
            let u = a.get_node(n.u);
            let d = a.get_node(n.d);
            a.set_node(u.set_d(d));
            a.set_node(d.set_u(u));
            a.set_col(col.decr_s());
            println!(
                "col {}\n  cover node {:#?}\n  cn {:#?}\n  node {:#?}\n  n {:#?}\n---",
                col.addr.c, cover_node.addr, cn.addr, node.addr, n.addr
            );
            // thread::sleep(Duration::from_millis(1000));
            n = a.get_node(n.r);
        }
        cn = a.get_node(cn.d);
        ri += 1;
    }
}

pub fn uncover(a: &mut A, c: ColAddr) {
    let col = a.get_col(c);
    let cover_node = a.get_node(col.u);
    let mut cn = cover_node;
    let mut cinit = true;
    while cinit | (cn.addr < cover_node.addr) {
        cinit = false;
        let node = a.get_node(cover_node.l);
        let mut n = node;
        let mut ninit = true;
        while ninit | (n.addr != node.addr) {
            ninit = false;
            let u = a.get_node(n.u);
            let d = a.get_node(n.d);
            a.set_node(u.set_d(n));
            a.set_node(d.set_u(n));
            a.set_col(col.incr_s());
            // thread::sleep(Duration::from_millis(1000));
            n = a.get_node(n.l);
        }
        cn = a.get_node(cn.u);
    }

    let l = a.get_col(col.l);
    let r = a.get_col(col.r);
    a.set_col(l.set_r(col));
    a.set_col(r.set_l(col));
}

pub fn search(a: &mut A, depth: usize, soln: &mut Vec<usize>, soln_length: usize) {
    // if depth > 3 {
    //     return;
    // }
    println!("\ndepth={depth}");
    if a.get_col(a.root.r).root {
        println!("Only root.");
        return;
    }

    let col = a.choose_col();

    println!("\n  cover {}", col.addr.c);
    cover(a, col.addr);
    println!("  covered {:#?}", col);

    let down = col.d;

    println!("  down {:#?}", down);
    let mut d = down;
    let mut dinit = true;

    while dinit | (d != down) {
        dinit = false;
        println!("\n  d.r={}", d.r);
        // come back up the rows
        // if !dinit & (d.r <= down.r) {
        //     break;
        // }
        if (soln.len() == 0) | (depth >= soln.len()) {
            println!("  push {}", d.r);
            // Include this row in the solution.
            soln.push(d.r as usize);
        } else {
            println!("  reset {}", d.r);
            soln[depth] = d.r as usize;
        }
        let right = a.get_node(d).r;
        println!("  right {:#?}", right);
        let mut r = right;

        let mut rinit = true;
        while rinit | (r != right) {
            rinit = false;
            println!("\n  r {:#?}", r);
            // come back round the cols
            // if !rinit & (r.c <= right.c) {
            //     break;
            // }
            println!("\n  cover {:#?}", a.get_node(r).c);
            cover(a, a.get_node(r).c);
            println!("\n  covered {:#?}", a.get_node(r).c);
            r = a.get_node(r).r;
        }
        // return;
        search(a, depth + 1, soln, soln_length);
        println!("soln {:#?}", soln);
        if soln.len() == soln_length {
            return;
        }
        let left = a.get_node(down).l;
        let mut l = left;
        let mut linit = true;
        while linit | (l != left) {
            linit = false;
            // come back round the cols
            // if !linit & (l.c >= left.c) {
            //     break;
            // }
            println!("\n  uncover {}", a.get_node(l).c.c);
            uncover(a, a.get_node(l).c);
            println!("\n  uncovered {:#?}", a.get_node(l));

            l = a.get_node(l).l;
        }
        d = a.get_node(d).d;
    }
    uncover(a, col.addr);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_from_matrix() {
        let matrix = vec![
            vec![0, 0, 1, 0, 1, 1, 0],
            vec![1, 0, 0, 1, 0, 0, 1],
            vec![0, 1, 1, 0, 0, 1, 0],
            vec![1, 0, 0, 1, 0, 0, 0],
            vec![0, 1, 0, 0, 0, 0, 1],
            vec![0, 0, 0, 1, 1, 0, 1],
        ];
        let mut a = from_matrix(&matrix);
        let mut soln = Vec::new();
        search(&mut a, 0, &mut soln, 3);
    }
}
