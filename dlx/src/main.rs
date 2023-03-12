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
    let mut a = dlx::from_matrix(&matrix);
    dlx::cover(&mut a, 1);
}
