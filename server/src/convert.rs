use time::{Date, Month, Time};
use todolist_core::{
    error::{ConvertError, ManagerError},
    model::{TodoContent, TodoDueDate},
};
use tonic::Status;

use crate::api::v1::todo::{Content, DueDate};

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
        todo!()
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

impl TryFrom<DueDate> for TodoDueDate {
    type Error = ConvertError;

    fn try_from(value: DueDate) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFrom<TodoDueDate> for DueDate {
    type Error = ConvertError;

    fn try_from(value: TodoDueDate) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFrom<TodoContent> for Content {
    type Error = ConvertError;

    fn try_from(value: TodoContent) -> Result<Self, Self::Error> {
        todo!()
    }
}
