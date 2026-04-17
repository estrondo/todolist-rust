use time::{PrimitiveDateTime, UtcDateTime};

pub trait TimeGenerator: Send + Sync + Clone {
    fn new_utc_date_time(&self) -> UtcDateTime;

    fn new_utc_primitive_date_time(&self) -> PrimitiveDateTime {
        let date_time = self.new_utc_date_time();
        PrimitiveDateTime::new(date_time.date(), date_time.time())
    }
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
