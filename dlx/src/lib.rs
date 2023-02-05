
struct Col {
    i: i32,
    l: Option<Box<Col>>,
    r: Option<Box<Col>>,
    u: Option<Box<Node>>,
    d: Option<Box<Node>>,
}

impl Col {
    fn default() -> Col {
        Col {i: 0, l: None, r: None, u: None, d: None}
    }

    fn left(&self) -> Option<Col> {
        match self.l {
            None => None,
            Some(l) => Some(*l),
        }
    }
}

struct Node {

    i: i32,
    c: Option<Box<Col>>,
    l: Option<Box<Node>>,
    r: Option<Box<Node>>,
    u: Option<Box<Node>>,
    d: Option<Box<Node>>,
}

impl Node {
    fn default() -> Node {
        Node{i: 0, c: None, l: None, r: None, u: None, d: None}
    }
}

fn from_matrix(matrix: &Vec<Vec<i32>>) -> Col {
    let mut root = Col::default();
    let mut cols = <Vec<Col>>::new();
    for (ridx, row) in matrix.into_iter().enumerate() {
        if ridx == 1 {
            cols[0].l = Some(Box::new(root));
            cols[cols.len()].r = Some(Box::new(root));
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
    }
}
