use std::time::{Duration, Instant};

pub(crate) fn get_duration<F: FnOnce()>(f: F) -> Duration {
    let instant = Instant::now();
    f();
    instant.elapsed()
}
