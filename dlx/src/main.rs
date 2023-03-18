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
    let mut soln = Vec::new();

    println!("{}", a.choose_col().addr.c);

    dlx::search(&mut a, 0, &mut soln, 3);
    // dlx::cover(&mut a, dlx::ColAddr { c: 1 });
    // dlx::uncover(&mut a, dlx::ColAddr { c: 1 });
}
