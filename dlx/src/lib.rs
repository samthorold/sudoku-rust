pub struct A {
    root: Node,
    headers: Vec<Node>,
    nodes: Vec<Vec<Node>>,
}

impl A {
    fn get_node(&self, addr: Addr) -> Node {
        if addr.row < 0 {
            match addr.col {
                c if c >= 0 => self.headers[c as usize],
                _ => self.root,
            }
        } else {
            if (addr.row < 0) | (addr.col < 0) {
                panic!("Invalid node addr {}{}", addr.row, addr.col);
            }
            self.nodes[addr.row as usize][addr.col as usize]
        }
    }

    fn set_node(&mut self, node: Node) {
        if node.root {
            self.root = node;
        } else if node.addr.row < 0 {
            self.headers[node.addr.col as usize] = node;
        } else {
            self.nodes[node.addr.row as usize][node.addr.col as usize] = node;
        }
    }

    fn set_left(&mut self, node: Addr, other: Addr) {
        self.set_node(self.get_node(node).set_left(self.get_node(other)));
    }

    fn set_right(&mut self, node: Addr, other: Addr) {
        self.set_node(self.get_node(node).set_right(self.get_node(other)));
    }

    fn set_up(&mut self, node: Addr, other: Addr) {
        self.set_node(self.get_node(node).set_up(self.get_node(other)));
    }

    fn set_down(&mut self, node: Addr, other: Addr) {
        self.set_node(self.get_node(node).set_down(self.get_node(other)));
    }

    pub fn choose_header(&self) -> Node {
        let mut size = usize::MAX;
        let mut addr = Addr::new();
        let mut header = self.get_node(self.root.right);
        while !header.root {
            if header.size < size {
                addr = header.addr;
                size = header.size;
            }
            header = self.get_node(header.right);
        }
        self.get_node(addr)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Addr {
    pub row: i32,
    pub col: i32,
}

impl Addr {
    fn new() -> Addr {
        Addr { row: -1, col: -1 }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Node {
    root: bool,
    pub addr: Addr,
    is_header: bool,
    is_legit: bool,
    size: usize,
    header: Addr,
    left: Addr,
    right: Addr,
    up: Addr,
    down: Addr,
}

impl Node {
    fn new(row: i32, col: i32, is_header: bool) -> Node {
        Node {
            root: false,
            addr: Addr { row, col },
            is_header,
            is_legit: false,
            size: 0,
            header: Addr::new(),
            left: Addr::new(),
            right: Addr::new(),
            up: Addr::new(),
            down: Addr::new(),
        }
    }

    fn set_header(self, node: Node) -> Node {
        if self.is_header | self.root {
            panic!("Cannot set the header of a header or the root node")
        } else {
            Node {
                header: node.addr,
                ..self
            }
        }
    }

    fn incr_size(self) -> Node {
        if self.is_header {
            Node {
                size: self.size + 1,
                ..self
            }
        } else {
            panic!("Cannot set the size of a non-header node")
        }
    }

    fn decr_size(self) -> Node {
        if self.is_header {
            Node {
                size: self.size - 1,
                ..self
            }
        } else {
            panic!("Cannot set the size of a non-header node")
        }
    }

    fn set_left(self, node: Node) -> Node {
        Node {
            left: node.addr,
            ..self
        }
    }

    fn set_right(self, node: Node) -> Node {
        Node {
            right: node.addr,
            ..self
        }
    }

    fn set_up(self, node: Node) -> Node {
        Node {
            up: node.addr,
            ..self
        }
    }

    fn set_down(self, node: Node) -> Node {
        Node {
            down: node.addr,
            ..self
        }
    }
}

/// Create a problem, `A`, from a matrix of 0s and 1s.
pub fn from_matrix(matrix: &Vec<Vec<u8>>) -> A {
    // the "root" col is used to start off each iteration of covering
    let mut root = Node::new(-1, -1, true);
    root.root = true;

    let mut headers = <Vec<Node>>::new();
    let mut nodes = <Vec<Vec<Node>>>::new();

    for (r, row) in matrix.into_iter().enumerate() {
        let mut row_nodes = <Vec<Node>>::new();

        for (c, val) in row.into_iter().enumerate() {
            if r == 0 {
                headers.push(Node::new(-1, c as i32, true));
                if c > 0 {
                    headers[c] = headers[c].set_left(headers[c - 1]);
                    headers[c - 1] = headers[c - 1].set_right(headers[c]);
                }
            }
            row_nodes.push(Node::new(r as i32, c as i32, false));
            if *val == 1 {
                row_nodes[c].is_legit = true;

                headers[c] = headers[c].incr_size();
                row_nodes[c] = row_nodes[c].set_header(headers[c]);

                if headers[c].down.row < 0 {
                    // column's down addr will be initialised to -1, -1
                    // so down.right -1 means no down addr set yet for a column
                    headers[c] = headers[c].set_down(row_nodes[c]);
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
                                nodes[u as usize][c].set_down(*row_nodes.last().unwrap());
                            row_nodes[c] = row_nodes[c].set_up(nodes[u as usize][c]);
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
                            row_nodes[c] = row_nodes[c].set_left(row_nodes[l as usize]);
                            row_nodes[l as usize] = row_nodes[l as usize].set_right(row_nodes[c]);
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

        row_nodes[r_] = row_nodes[r_].set_right(row_nodes[l_]);
        row_nodes[l_] = row_nodes[l_].set_left(row_nodes[r_]);

        nodes.push(row_nodes);
    }

    let h = nodes.len();
    let w = headers.len();

    for j in 0..w {
        let u = headers[j].down;
        nodes[u.row as usize][j] = nodes[u.row as usize][j].set_up(headers[j]);
        for i in (0..h).rev() {
            if nodes[i][j].is_legit {
                nodes[i][j] = nodes[i][j].set_down(headers[j]);
                headers[j] = headers[j].set_up(nodes[i][j]);
                break;
            }
        }
    }

    root = root.set_right(headers[0]);
    root = root.set_left(*headers.last().unwrap());

    headers[0] = headers[0].set_left(root);
    headers[w - 1] = headers[w - 1].set_right(root);

    return A {
        root,
        headers,
        nodes,
    };
}

pub fn cover(a: &mut A, header: Node) {
    a.set_right(header.left, header.right);
    a.set_left(header.right, header.left);
    let mut cover_node = a.get_node(header.down);
    while !cover_node.is_header {
        let mut node = a.get_node(cover_node.right);
        while node.addr != cover_node.addr {
            a.set_down(node.up, node.down);
            a.set_up(node.down, node.up);
            a.set_node(a.get_node(node.header).decr_size());
            node = a.get_node(node.right);
        }
        cover_node = a.get_node(cover_node.down);
    }
}

pub fn uncover(a: &mut A, header: Node) {
    let mut cover_node = a.get_node(header.up);
    while !cover_node.is_header {
        let mut node = a.get_node(cover_node.left);
        while node.addr != cover_node.addr {
            a.set_down(node.up, node.addr);
            a.set_up(node.down, node.addr);
            a.set_node(a.get_node(node.header).incr_size());
            node = a.get_node(node.left);
        }
        cover_node = a.get_node(cover_node.up);
    }
    a.set_left(header.right, header.addr);
    a.set_right(header.left, header.addr);
}

pub fn search(a: &mut A, depth: usize, soln: &mut Vec<usize>, soln_length: usize) {
    if a.get_node(a.root.right).root {
        return;
    }
    let header = a.choose_header();
    cover(a, header);
    let mut down = a.get_node(header.down);
    while !down.is_header {
        if (soln.len() == 0) | (depth >= soln.len()) {
            soln.push(down.addr.row as usize);
        } else {
            soln[depth] = down.addr.row as usize;
        }
        let mut right = a.get_node(down.right);
        while right.addr != down.addr {
            cover(a, a.get_node(right.header));
            right = a.get_node(right.right);
        }
        search(a, depth + 1, soln, soln_length);
        if soln.len() == soln_length {
            return;
        }
        let mut left = a.get_node(down.left);
        while left.addr != down.addr {
            uncover(a, a.get_node(left.header));
            left = a.get_node(left.left);
        }
        down = a.get_node(down.down);
    }
    uncover(a, header);
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
        let _ = from_matrix(&matrix);
    }
    #[test]
    fn test_search() {
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
        assert_eq!(soln, vec![3, 0, 4]);
    }
}
