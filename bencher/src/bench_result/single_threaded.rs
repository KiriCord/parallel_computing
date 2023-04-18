use std::fmt::{self, Display, Formatter};

use super::{common::bench_rayon, stats::Statistics};

#[derive(Debug)]
pub struct BenchResultSingleThreaded {
    pub stats: Statistics,
}

impl BenchResultSingleThreaded {
    pub fn bench_rayon<F: Fn() + Send + Sync>(
        f: F,
        repeat_count: usize,
    ) -> BenchResultSingleThreaded {
        let stats = bench_rayon(&f, 1, repeat_count).into();

        BenchResultSingleThreaded { stats }
    }
}

impl Display for BenchResultSingleThreaded {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "1 поток")?;

        write!(f, "{}", self.stats)
    }
}
