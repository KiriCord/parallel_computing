use std::{
    fmt::{self, Display, Formatter},
    time::Duration,
};

#[derive(Debug)]
pub struct BenchResultSingleThreaded {
    pub duration: Duration,
    pub repeat_count: i32,
}

impl Display for BenchResultSingleThreaded {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "1 поток")?;
        writeln!(
            f,
            "Общее время выполнения: {:.3} секунд",
            self.duration.as_secs_f64()
        )?;
        writeln!(f, "Кол-во выполнений {}", self.repeat_count)?;
        write!(
            f,
            "Среднее время выполнение {:.3} секунд",
            self.duration.as_secs_f64() / (self.repeat_count as f64)
        )
    }
}
