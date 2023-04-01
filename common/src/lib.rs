use std::ops::Range;

use rand::{
    distributions::{uniform::SampleUniform, Uniform},
    prelude::Distribution,
    thread_rng,
};

use rayon::prelude::*;

pub fn generate_vector<T>(n: usize, range: &Range<T>) -> Vec<T>
where
    <T as SampleUniform>::Sampler: Sync,
    T: SampleUniform + From<i32> + Send + Copy,
{
    let between = Uniform::from(Range {
        start: range.start.into(),
        end: range.end.into(),
    });
    let mut v = Vec::with_capacity(n);
    (0..n)
        .into_par_iter()
        .map(|_| between.sample(&mut thread_rng()))
        .collect_into_vec(&mut v);
    v
}

pub fn generate_square_matrix<T>(n: usize, range: Range<T>) -> Vec<Vec<T>>
where
    <T as SampleUniform>::Sampler: Sync,
    T: SampleUniform + From<i32> + Send + Sync + Copy,
{
    let mut matrix = Vec::with_capacity(n);

    (0..n)
        .into_par_iter()
        .map(|_| generate_vector(n, &range))
        .collect_into_vec(&mut matrix);

    matrix
}
