pub struct A {
    cols: Vec<Col>,
    nodes: Vec<Vec<Node>>,
}

#[derive(Copy, Clone, Debug)]
struct ColAddr {
    c: i32,
}

impl ColAddr {
    fn new() -> ColAddr {
        ColAddr { c: -1 }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Col {
    root: bool,
    a: ColAddr,
    l: ColAddr,
    r: ColAddr,
    d: NodeAddr,
    s: i32,
}

impl Col {
    fn new(c: i32) -> Col {
        Col {
            root: false,
            a: ColAddr { c },
            l: ColAddr::new(),
            r: ColAddr::new(),
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

    fn set_l(self, c: i32) -> Col {
        Col {
            l: ColAddr { c },
            ..self
        }
    }

    fn set_r(self, c: i32) -> Col {
        Col {
            r: ColAddr { c },
            ..self
        }
    }

    fn set_d(self, r: i32) -> Col {
        Col {
            d: NodeAddr { r, ..self.d },
            ..self
        }
    }
}

#[derive(Copy, Clone, Debug)]
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
    a: NodeAddr,
    c: ColAddr,
    l: NodeAddr,
    r: NodeAddr,
    u: NodeAddr,
    d: NodeAddr,
}

impl Node {
    fn new(r: i32, c: i32) -> Node {
        Node {
            a: NodeAddr { r, c },
            c: ColAddr::new(),
            l: NodeAddr::new(),
            r: NodeAddr::new(),
            u: NodeAddr::new(),
            d: NodeAddr::new(),
        }
    }

    fn set_c(self, c: i32) -> Node {
        Node {
            c: ColAddr { c },
            ..self
        }
    }

    fn set_l(self, c: i32) -> Node {
        Node {
            l: NodeAddr { c, ..self.l },
            ..self
        }
    }

    fn set_r(self, c: i32) -> Node {
        Node {
            r: NodeAddr { c, ..self.r },
            ..self
        }
    }

    fn set_u(self, r: i32) -> Node {
        Node {
            u: NodeAddr { r, ..self.u },
            ..self
        }
    }

    fn set_d(self, r: i32) -> Node {
        Node {
            d: NodeAddr { r, ..self.d },
            ..self
        }
    }
}

pub fn from_matrix(matrix: &Vec<Vec<u8>>) -> A {
    let mut root = Col::new(0);
    root.root = true;
    let mut cols = <Vec<Col>>::new();
    let mut nodes = <Vec<Vec<Node>>>::new();
    for (r, row) in matrix.into_iter().enumerate() {
        let mut row_nodes = <Vec<Node>>::new();
        let r32 = r.try_into().unwrap();
        for (c, val) in row.into_iter().enumerate() {
            let c32 = c.try_into().unwrap();
            if r == 0 {
                cols.push(Col::new(c32));
            }
            if c > 0 {
                cols[c] = cols[c].set_l(c32 - 1);
                cols[c - 1] = cols[c - 1].set_r(c32);
            }
            let mut node = Node::new(r32, c32);
            if *val == 1 {
                cols[c] = cols[c].incr_s();
                node = node.set_c(c.try_into().unwrap());

                println!("{:#?} setting d ...", node.a);
                if cols[c].d.r < 0 {
                    // println!("col.d < 0 ...");
                    cols[c] = cols[c].set_d(r32)
                } else {
                    let mut u = cols[c].d.r;
                    loop {
                        let next_u = nodes[u as usize][c].d.r;
                        // println!("u={u}, next_u={next_u}");
                        if next_u < 0 {
                            node = node.set_u(u);
                            nodes[u as usize][c] = nodes[u as usize][c].set_d(r32);
                            println!("u={u}, d={r}");
                            break;
                        } else {
                            // println!("setting u={u} to next_u={next_u}");
                            u = next_u;
                        }
                    }
                }
            }
            // println!("{:#?}", node.a);
            row_nodes.push(node);
        }
        nodes.push(row_nodes);

        // first Col will have initialised to l: 0
        // final Col will have initialised to r: 0
        // but root needs to know about first and last Col
        root.set_r(cols[0].a.c);
        root.set_l(cols[cols.len() - 1].a.c);
    }
    return A { cols, nodes };
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
        let a = from_matrix(&matrix);
        let a = from_matrix(&matrix);
    }
}
