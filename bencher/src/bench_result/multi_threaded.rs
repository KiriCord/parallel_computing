use std::fmt::{self, Display, Formatter};

use super::{common::bench_rayon, stats::Statistics, BenchResultSingleThreaded};

#[derive(Debug)]
pub struct BenchResultMultiThreaded {
    pub stats: Statistics,
    pub thread_count: usize,
    pub speedup: f64,
    pub effectiveness: f64,
}

impl BenchResultMultiThreaded {
    pub fn bench_rayon<F: Fn() + Send + Sync>(
        f: F,
        thread_count: usize,
        repeat_count: usize,
        single_threaded_result: &BenchResultSingleThreaded,
    ) -> BenchResultMultiThreaded {
        let stats = Statistics::from(bench_rayon(&f, thread_count, repeat_count));

        let speedup =
            single_threaded_result.stats.total_time.as_secs_f64() / stats.total_time.as_secs_f64();
        let effectiveness = speedup / (thread_count as f64);

        BenchResultMultiThreaded {
            stats,
            speedup,
            thread_count,
            effectiveness,
        }
    }
}

impl Display for BenchResultMultiThreaded {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(
            f,
            "{} {}",
            self.thread_count,
            match self.thread_count {
                2 | 3 | 4 => "потока",
                _ => "потоков",
            }
        )?;

        writeln!(f, "{}", self.stats)?;

        writeln!(f, "Ускорение: {:.3}", self.speedup)?;
        write!(f, "Эффективность: {:.3}", self.effectiveness)
    }
}
