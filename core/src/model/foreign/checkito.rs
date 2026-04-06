use std::ops::RangeInclusive;

use checkito::{Generate, boxed::Boxed, letter};
use time::{Date, Duration, Month, Time, UtcDateTime};
use uuid::Uuid;

pub fn date_range() -> RangeInclusive<i64> {
    let (d0, d1) = (
        Date::from_calendar_date(1950, Month::January, 1).unwrap(),
        Date::from_calendar_date(2050, Month::December, 31).unwrap(),
    );

    let (t0, t1) = (
        Time::from_hms(0, 0, 0).unwrap(),
        Time::from_hms(23, 59, 59).unwrap(),
    );

    UtcDateTime::new(d0, t0).unix_timestamp()..=UtcDateTime::new(d1, t1).unix_timestamp()
}

pub fn non_empty_string_generator() -> Boxed<String> {
    Generate::collect_with(letter(), 1..32).boxed()
}

pub fn uuid_generator() -> Boxed<Uuid> {
    // <(u64, u64)>::generator()
    //     .map(|(h, l)| Uuid::from_u64_pair(h, l))
    //     .boxed()
    Generate::map((), |_| Uuid::now_v7()).boxed()
}

pub fn date_generator() -> Boxed<Date> {
    Generate::map(date_range(), |x| {
        UtcDateTime::from_unix_timestamp(x).unwrap().date()
    })
    .boxed()
}

pub fn utc_date_time_generator() -> Boxed<UtcDateTime> {
    Generate::map(date_range(), |x| {
        UtcDateTime::from_unix_timestamp(x).unwrap()
    })
    .boxed()
}

pub fn duration_generator() -> Boxed<Duration> {
    Generate::map(1i64..=(3600 * 2), |x| Duration::seconds(x)).boxed()
}
