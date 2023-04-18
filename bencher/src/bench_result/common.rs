use std::time::{Duration, Instant};

use rayon::{ThreadPool, ThreadPoolBuilder};

fn make_pool(num_threads: usize) -> ThreadPool {
    ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .unwrap()
}

fn get_duration<F: FnOnce()>(f: F) -> Duration {
    let instant = Instant::now();
    f();
    instant.elapsed()
}

pub fn bench_rayon<F: Fn() + Send + Sync>(
    f: F,
    thread_count: usize,
    repeat_count: usize,
) -> Vec<Duration> {
    let pool = make_pool(thread_count);
    let mut durations = Vec::with_capacity(repeat_count);
    for _ in 0..repeat_count {
        let dur = get_duration(|| pool.install(&f));
        durations.push(dur);
    }
    durations.sort_unstable();
    durations
}
