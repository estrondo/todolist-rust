use time::UtcDateTime;

pub trait TimeGenerator: Send + Sync {
    fn new_utc_date_time(&self) -> UtcDateTime;
}

/// Default implementation of [`TimeGenerator`] that generates UTC timestamps.
#[derive(Default, Clone)]
pub struct DefaultTimeGenerator {}

impl TimeGenerator for DefaultTimeGenerator {
    /// Generates the current UTC datetime truncated to millisecond precision.
    ///
    /// # Returns
    ///
    /// The current UTC time as a [`UtcDateTime`] with millisecond precision.
    fn new_utc_date_time(&self) -> UtcDateTime {
        UtcDateTime::now().truncate_to_millisecond()
    }
}
