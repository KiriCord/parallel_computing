use bencher::{rayon::*, THREAD_COUNT};
use common::generate_square_matrix;
use rayon::prelude::*;

const MATRIX_SIZE: usize = 25000;
const REPEAT_COUNT: i32 = 10;
type Matrix<T> = Vec<Vec<T>>;

fn sum_max_el_matrix(matrix: &Matrix<i32>) -> i32 {
    matrix.par_iter().map(|x| x.par_iter().max().unwrap()).sum()
}

fn main() {
    let matrix = &generate_square_matrix(MATRIX_SIZE, 0..10000);

    let single_threaded_result = bench_single_threaded(
        move || {
            sum_max_el_matrix(matrix);
        },
        REPEAT_COUNT,
    );

    println!("{}\n", single_threaded_result);
    for thread_count in THREAD_COUNT {
        let bench_result = bench(
            move || {
                sum_max_el_matrix(matrix);
            },
            thread_count,
            REPEAT_COUNT,
            &single_threaded_result,
        );
        println!("{}\n", bench_result);
    }
}
