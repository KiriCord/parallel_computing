use std::{fmt::Display, time::Duration};

#[derive(Debug)]
pub struct Statistics {
    pub total_time: Duration,
    pub repeat_count: usize,
    pub average_time: Duration,
    pub median: Duration,
}

impl From<&Vec<Duration>> for Statistics {
    fn from(value: &Vec<Duration>) -> Self {
        let total_time: Duration = value.iter().sum();
        let repeat_count = value.len();
        let average_time = total_time / (repeat_count as u32);
        let mid = repeat_count / 2;
        let median = if repeat_count % 2 == 0 {
            (value[mid - 1] + value[mid]) / 2
        } else {
            value[mid]
        };

        Statistics {
            total_time,
            repeat_count,
            average_time,
            median,
        }
    }
}

impl From<Vec<Duration>> for Statistics {
    fn from(value: Vec<Duration>) -> Self {
        Statistics::from(&value)
    }
}

impl Display for Statistics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Общее время выполнения: {:.3} секунд",
            self.total_time.as_secs_f64()
        )?;

        writeln!(f, "Кол-во выполнений: {}", self.repeat_count)?;
        writeln!(
            f,
            "Среднее время выполнение: {:.3} секунд",
            self.average_time.as_secs_f64()
        )?;
        write!(f, "Медиана: {:.3} секунд", self.median.as_secs_f64())
    }
}
