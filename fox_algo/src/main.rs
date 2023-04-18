use bencher::generate_bench;
use common::generate_square_matrix;
use rayon::prelude::*;

const MATRIX_SIZE: usize = 256;
type Matrix<T> = Vec<Vec<T>>;

fn _multiply(matrix_a: &Matrix<i32>, matrix_b: &Matrix<i32>) -> Matrix<i32> {
    (0..matrix_a.len())
        .into_par_iter()
        .map(move |i| {
            (0..matrix_b.len())
                .into_par_iter()
                .map(move |j| {
                    (0..matrix_b.len())
                        .into_par_iter()
                        .map(move |k| matrix_a[i][k] * matrix_b[k][j])
                        .sum()
                })
                .collect::<Vec<i32>>()
        })
        .collect::<Matrix<i32>>()
}

fn fox_algo(matrix_a: &Matrix<i32>, matrix_b: &Matrix<i32>) -> Matrix<i32> {
    let mut matrix_c = vec![vec![0; MATRIX_SIZE]; MATRIX_SIZE];

    let temp_result: Vec<_> = (0..MATRIX_SIZE)
        .into_par_iter()
        .map(|stage| {
            (0..matrix_a.len()).into_par_iter().map(move |i| {
                let k = (i + stage) % MATRIX_SIZE;
                (0..matrix_b.len())
                    .into_par_iter()
                    .map(move |j| (i, j, matrix_a[i][k] * matrix_b[k][j]))
            })
        })
        .flatten()
        .flatten()
        .collect();

    for (i, j, item) in temp_result.iter() {
        matrix_c[*i][*j] += item;
    }

    matrix_c
}

fn main() {
    let matrix_a = &generate_square_matrix(MATRIX_SIZE, 0..100);
    let matrix_b = &generate_square_matrix(MATRIX_SIZE, 0..100);
    generate_bench!(5, [4, 8, 16], fox_algo, matrix_a, matrix_b);
}
