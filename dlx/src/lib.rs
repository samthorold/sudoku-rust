#[derive(Copy, Clone, Debug)]
struct ColAddr {
    c: usize,
}

impl ColAddr {
    fn new() -> ColAddr {
        ColAddr { c: 0 }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Col {
    root: bool,
    l: ColAddr,
    r: ColAddr,
}

impl Col {
    fn new() -> Col {
        Col {
            root: false,
            l: ColAddr::new(),
            r: ColAddr::new(),
        }
    }

    fn set_l(self, c: usize) -> Col {
        Col {
            l: ColAddr { c },
            ..self
        }
    }

    fn set_r(self, c: usize) -> Col {
        Col {
            r: ColAddr { c },
            ..self
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct NodeAddr {
    r: usize,
    c: usize,
}

impl NodeAddr {
    fn new() -> NodeAddr {
        NodeAddr { r: 0, c: 0 }
    }
}

#[derive(Copy, Clone, Debug)]
struct Node {
    c: ColAddr,
    l: NodeAddr,
    r: NodeAddr,
    u: NodeAddr,
    d: NodeAddr,
}

impl Node {
    fn new() -> Node {
        Node {
            c: ColAddr::new(),
            l: NodeAddr::new(),
            r: NodeAddr::new(),
            u: NodeAddr::new(),
            d: NodeAddr::new(),
        }
    }
}

pub fn from_matrix(matrix: &Vec<Vec<u8>>) -> Col {
    let mut root = Col::new();
    root.root = true;
    let mut cols = <Vec<Col>>::new();
    for (ridx, row) in matrix.into_iter().enumerate() {
        for (cidx, val) in row.into_iter().enumerate() {
            if *val == 1 {
                println!("{ridx} {cidx}")
            }
            if ridx == 0 {
                cols.push(Col::new());
            }
            if cidx > 0 {
                cols[cidx] = cols[cidx].set_l(cidx - 1);
                cols[cidx - 1] = cols[cidx - 1].set_r(cidx);
            }
        }
    }
    return root;
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
        let root = from_matrix(&matrix);
        let root = from_matrix(&matrix);
    }
}
