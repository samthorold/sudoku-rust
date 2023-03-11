use dlx;

fn main() {
    let matrix = vec![
        vec![0, 0, 1, 0, 1, 1, 0],
        vec![1, 0, 0, 1, 0, 0, 1],
        vec![0, 1, 1, 0, 0, 1, 0],
        vec![1, 0, 0, 1, 0, 0, 0],
        vec![0, 1, 0, 0, 0, 0, 1],
        vec![0, 0, 0, 1, 1, 0, 1],
    ];
    dlx::from_matrix(&matrix);
}
