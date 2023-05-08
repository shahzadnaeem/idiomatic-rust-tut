//
// How to transpose in Rust. Started from an invalid sample
// from this Open Large Language Model - https://chat.lmsys.org/?model=koala-13b
//

fn transpose_matrix(matrix: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let rows = matrix.len();
    let cols = matrix[0].len();

    (0..cols)
        .map(|j| (0..rows).map(|i| matrix[i][j]).collect())
        .collect()
}

fn main() {
    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let transposed = transpose_matrix(&matrix);

    println!("Transposed matrix:");
    for row in transposed {
        println!("{:?}", row);
    }
}
