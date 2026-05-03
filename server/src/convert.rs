use time::{Date, Duration, Month, Time, UtcDateTime};
use todolist_core::{
    centre::CentreError,
    error::ConvertError,
    model::todo::{TodoContent, TodoDueDate},
};
use tonic::Status;

use crate::api::v1::{
    Date as TodoDate, Period,
    todo::{
        Content, DueDate, MarkdownContent, PlainContent, content::Content as ContentEnum,
        due_date::When,
    },
};

pub(crate) fn invalid_request_message(error: ConvertError) -> Status {
    Status::invalid_argument(error.0)
}

pub(crate) fn unexpected_internal_conversion_error(error: ConvertError) -> Status {
    Status::internal(error.0)
}

pub(crate) fn centre_error_to_status(error: CentreError) -> Status {
    match error {
        CentreError::Unexpected(_, _) => Status::internal("An internal error happened"),
        CentreError::Unauthorized(message, _) => Status::permission_denied(message),
    }
}

impl TryInto<TodoContent> for Content {
    type Error = ConvertError;

    fn try_into(self) -> Result<TodoContent, Self::Error> {
        match self.content {
            Some(ContentEnum::Plain(value)) => Ok(TodoContent::Plain(value.content)),
            Some(ContentEnum::Markdown(value)) => Ok(TodoContent::Markdown(value.content)),
            None => Err(ConvertError::from("Empty Content!")),
        }
    }
}

fn to_date(y: i32, m: u8, d: u8) -> Result<Date, ConvertError> {
    let month = Month::try_from(m)
        .map_err(|e| ConvertError::from(format!("Invalid moth: {}", e.to_string())))?;
    Date::from_calendar_date(y, month, d)
        .map_err(|e| ConvertError::from(format!("Invalid date: {}", e.to_string())))
}

fn to_time(h: u8, m: u8, s: u8) -> Result<Time, ConvertError> {
    Time::from_hms(h, m, s)
        .map_err(|e| ConvertError::from(format!("Invalid time: {}", e.to_string())))
}

fn to_month(m: u8) -> Result<Month, ConvertError> {
    Month::try_from(m).map_err(|_| ConvertError::from(format!("Invalid month {m}.")))
}

impl TryFrom<DueDate> for TodoDueDate {
    type Error = ConvertError;

    fn try_from(value: DueDate) -> Result<Self, Self::Error> {
        match value.when {
            Some(When::WholeDay(whole_day)) => {
                let date = Date::from_calendar_date(
                    whole_day.year as i32,
                    to_month(whole_day.month as u8)?,
                    whole_day.day as u8,
                )
                .map_err(|e| ConvertError::from(e.to_string()))?;
                Ok(Self::WholeDay(date))
            }
            Some(When::Period(period)) => {
                let date = to_date(period.year as i32, period.month as u8, period.day as u8)?;
                let time = to_time(period.hour as u8, period.minute as u8, 0)?;
                Ok(TodoDueDate::Period(
                    UtcDateTime::new(date, time),
                    Duration::minutes(period.minutes as i64),
                ))
            }
            None => Err(ConvertError::from("Empty DueDate.")),
        }
    }
}

impl TryFrom<TodoDueDate> for DueDate {
    type Error = ConvertError;

    fn try_from(value: TodoDueDate) -> Result<Self, Self::Error> {
        match value {
            TodoDueDate::WholeDay(date) => Ok(DueDate {
                when: Some(When::WholeDay(TodoDate {
                    year: date.year() as u32,
                    month: date.month() as u32,
                    day: date.day() as u32,
                })),
            }),
            TodoDueDate::Period(utc_date_time, duration) => Ok(DueDate {
                when: Some(When::Period(Period {
                    year: utc_date_time.year() as u32,
                    month: utc_date_time.month() as u32,
                    day: utc_date_time.day() as u32,
                    hour: utc_date_time.hour() as u32,
                    minute: utc_date_time.minute() as u32,
                    minutes: duration.whole_seconds() as u32,
                })),
            }),
        }
    }
}

impl TryFrom<TodoContent> for Content {
    type Error = ConvertError;

    fn try_from(value: TodoContent) -> Result<Self, Self::Error> {
        match value {
            TodoContent::Markdown(content) => Ok(Content {
                content: Some(ContentEnum::Markdown(MarkdownContent { content })),
            }),
            TodoContent::Plain(content) => Ok(Content {
                content: Some(ContentEnum::Plain(PlainContent { content })),
            }),
        }
    }
}
