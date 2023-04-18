use bencher::generate_bench;
use common::generate_square_matrix;
use rayon::prelude::*;
use std::sync::mpsc::channel;

const MATRIX_SIZE: usize = 256;
type Matrix<T> = Vec<Vec<T>>;

fn multiply(matrix_a: &Matrix<i32>, matrix_b: &Matrix<i32>) -> Matrix<i32> {
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

    let (sender, receiver) = channel();
    (0..MATRIX_SIZE)
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
        .for_each_with(sender, |sender, item| sender.send(item).unwrap());

    for (i, j, item) in receiver.iter() {
        matrix_c[i][j] += item;
    }

    matrix_c

    // for stage in 0..MATRIX_SIZE {
    //     for i in 0..matrix_a.len() {
    //         let mut k = (i + stage) % MATRIX_SIZE;
    //         for j in 0..matrix_b.len() {
    //             matrix_c[i][j] += matrix_a[i][k] * matrix_b[k][j];
    //         }
    //     }
    // }
    // matrix_c
}

fn main() {
    let matrix_a = &generate_square_matrix(MATRIX_SIZE, 0..100);
    let matrix_b = &generate_square_matrix(MATRIX_SIZE, 0..100);
    generate_bench!(2, [2, 4], fox_algo, matrix_a, matrix_b);
}
