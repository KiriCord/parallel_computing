use bencher::generate_bench;
use common::generate_square_matrix;
use rayon::prelude::*;

const MATRIX_SIZE: usize = 25000;
type Matrix<T> = Vec<Vec<T>>;

fn sum_max_el_matrix(matrix: &Matrix<i32>) -> i32 {
    matrix.par_iter().map(|x| x.par_iter().max().unwrap()).sum()
}

fn main() {
    let matrix = &generate_square_matrix(MATRIX_SIZE, 0..10000);
    generate_bench!(10, [4, 8, 16], sum_max_el_matrix, matrix);
}
