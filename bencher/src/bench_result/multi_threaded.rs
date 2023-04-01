use std::{
    fmt::{self, Display, Formatter},
    time::Duration,
};

#[derive(Debug)]
pub struct BenchResultMultiThreaded {
    pub thread_count: usize,
    pub duration: Duration,
    pub speedup: f64,
    pub effectiveness: f64,
    pub repeat_count: i32,
}

impl Display for BenchResultMultiThreaded {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let thread = match self.thread_count {
            2 | 3 | 4 => "потока",
            _ => "потоков",
        };

        writeln!(f, "{} {}", self.thread_count, thread)?;

        writeln!(
            f,
            "Время выполнения: {:.3} секунд",
            self.duration.as_secs_f64()
        )?;
        writeln!(f, "Кол-во выполнений {}", self.repeat_count)?;
        writeln!(
            f,
            "Среднее время выполнение {:.3} секунд",
            self.duration.as_secs_f64() / (self.repeat_count as f64)
        )?;

        writeln!(f, "Ускорение: {:.3}", self.speedup)?;

        write!(f, "Эффективность: {:.3}", self.effectiveness)
    }
}
