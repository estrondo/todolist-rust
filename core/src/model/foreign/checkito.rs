use core::ops::RangeInclusive;

use checkito::{FullGenerate, Generate, boxed::Boxed, letter};
use orn::or2::Or;
use time::{Date, Duration, Month, Time, UtcDateTime};
use uuid::Uuid;

use crate::model::{Todo, TodoContent, TodoDueDate, TodoId, TodoStatus, TodoTitle};

fn date_limits() -> RangeInclusive<i64> {
    let (d0, d1) = (
        Date::from_calendar_date(2000, Month::January, 1).unwrap(),
        Date::from_calendar_date(2050, Month::December, 31).unwrap(),
    );

    let (t0, t1) = (
        Time::from_hms(0, 0, 0).unwrap(),
        Time::from_hms(23, 59, 59).unwrap(),
    );

    UtcDateTime::new(d0, t0).unix_timestamp()..=UtcDateTime::new(d1, t1).unix_timestamp()
}

fn non_empty_string_generator() -> Boxed<String> {
    Generate::collect_with(letter(), 1..32).boxed()
}

fn uuid_generator() -> Boxed<Uuid> {
    // <(u64, u64)>::generator()
    //     .map(|(h, l)| Uuid::from_u64_pair(h, l))
    //     .boxed()
    Generate::map((), |_| Uuid::now_v7()).boxed()
}

fn date_generator() -> Boxed<Date> {
    Generate::map(date_limits(), |x| {
        UtcDateTime::from_unix_timestamp(x).unwrap().date()
    })
    .boxed()
}

fn utc_date_time_generator() -> Boxed<UtcDateTime> {
    Generate::map(date_limits(), |x| {
        UtcDateTime::from_unix_timestamp(x).unwrap()
    })
    .boxed()
}

fn duration_generator() -> Boxed<Duration> {
    Generate::map(1i64..=(3600 * 2), |x| Duration::seconds(x)).boxed()
}

fn todo_id_generator() -> Boxed<TodoId> {
    uuid_generator().map(|uuid| TodoId(uuid)).boxed()
}

fn todo_title_generator() -> Boxed<TodoTitle> {
    non_empty_string_generator()
        .map(|title| TodoTitle(title))
        .boxed()
}

fn status_generator() -> Boxed<TodoStatus> {
    Generate::map(0..=4, |x| match x {
        0 => TodoStatus::Unspecified,
        1 => TodoStatus::Active,
        2 => TodoStatus::Postponed,
        3 => TodoStatus::Cancelled,
        4 => TodoStatus::Done,
        _ => panic!("Invalid TodoStatus:{x}"),
    })
    .boxed()
}

fn content_generator() -> Boxed<TodoContent> {
    let markdown = non_empty_string_generator();
    let plain = non_empty_string_generator();
    Generate::any((markdown, plain))
        .map(|or| match or {
            Or::T0(mark_down) => TodoContent::Markdown(mark_down),
            Or::T1(plain) => TodoContent::Plain(plain),
        })
        .boxed()
}

fn todo_due_date_generator() -> Boxed<TodoDueDate> {
    let date = date_generator();
    let date_time = utc_date_time_generator();
    let duration = duration_generator();

    Generate::any((date, (date_time, duration)))
        .map(|or| match or {
            Or::T0(date) => TodoDueDate::WholeDay(date),
            Or::T1((date_time, duration)) => TodoDueDate::Period(date_time, duration),
        })
        .boxed()
}

impl FullGenerate for Todo {
    type Item = Todo;

    type Generator = Boxed<Todo>;

    fn generator() -> Self::Generator {
        let id = todo_id_generator();
        let title = todo_title_generator();
        let due_date = todo_due_date_generator();
        let status = status_generator();
        let content = content_generator();

        Generate::map(
            (id, title, due_date, status, content),
            |(id, title, due_date, status, content)| Todo {
                id,
                title,
                due_date,
                status,
                content,
            },
        )
        .boxed()
    }
}
