use time::{Date, Duration, Month, Time, UtcDateTime};
use todolist_core::{
    error::{ConvertError, ManagerError},
    model::{TodoContent, TodoDueDate},
};
use tonic::Status;

use crate::api::v1::todo::{Content, DueDate, content::Content as ContentEnum, due_date::When};

pub(crate) fn invalid_request_message(error: ConvertError) -> Status {
    Status::invalid_argument(error.message())
}

pub(crate) fn manager_error_to_status(error: ManagerError) -> Status {
    match error {
        ManagerError::Internal { message } => Status::internal(message),
        ManagerError::CausedByError { message, cause: _ }
        | ManagerError::CausedByPersistence { message, cause: _ } => Status::internal(message),
    }
}

pub(crate) fn unexpected_internal_conversion_error(error: ConvertError) -> Status {
    Status::internal(error.message())
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

    fn try_from(_value: TodoDueDate) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFrom<TodoContent> for Content {
    type Error = ConvertError;

    fn try_from(_value: TodoContent) -> Result<Self, Self::Error> {
        todo!()
    }
}
