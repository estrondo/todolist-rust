use uuid::Uuid;

use crate::{
    error::ConvertError,
    model::{TodoContent, TodoDueDate, TodoId, TodoStatus, TodoTitle},
};

impl TryFrom<String> for TodoId {
    type Error = ConvertError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let uuid = Uuid::try_parse(&value)?;
        Result::Ok(TodoId(uuid))
    }
}

impl TryFrom<TodoId> for String {
    type Error = ConvertError;

    fn try_from(value: TodoId) -> Result<Self, Self::Error> {
        Result::Ok(value.0.to_string())
    }
}

impl TryFrom<String> for TodoTitle {
    type Error = ConvertError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if !value.is_empty() {
            Result::Ok(TodoTitle(value))
        } else {
            Result::Err(ConvertError::from("An empty title."))
        }
    }
}

impl TryFrom<TodoTitle> for String {
    type Error = ConvertError;

    fn try_from(value: TodoTitle) -> Result<Self, Self::Error> {
        Result::Ok(value.0.to_owned())
    }
}

impl TryFrom<i32> for TodoStatus {
    type Error = ConvertError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TodoStatus::Unspecified),
            1 => Ok(TodoStatus::Active),
            2 => Ok(TodoStatus::Postponed),
            3 => Ok(TodoStatus::Cancelled),
            4 => Ok(TodoStatus::Done),
            _ => Err(ConvertError::from(format!("Invalid status code: {value}."))),
        }
    }
}

impl<T> TryFrom<Option<T>> for TodoDueDate
where
    T: TryInto<TodoDueDate, Error = ConvertError>,
{
    type Error = ConvertError;

    fn try_from(value: Option<T>) -> Result<Self, Self::Error> {
        match value {
            Some(value) => value.try_into(),
            None => Result::Err(ConvertError::from(
                "Unable to convert an empty Option to TodoDueDate!",
            )),
        }
    }
}

impl<T> TryFrom<TodoDueDate> for Option<T>
where
    T: TryFrom<TodoDueDate, Error = ConvertError>,
{
    type Error = ConvertError;

    fn try_from(value: TodoDueDate) -> Result<Self, Self::Error> {
        let value: T = value.try_into()?;
        Result::Ok(Some(value))
    }
}

impl TryFrom<TodoStatus> for i32 {
    type Error = ConvertError;

    fn try_from(value: TodoStatus) -> Result<Self, Self::Error> {
        Result::Ok(value as i32)
    }
}

impl<T> TryFrom<Option<T>> for TodoContent
where
    T: TryInto<TodoContent, Error = ConvertError>,
{
    type Error = ConvertError;

    fn try_from(value: Option<T>) -> Result<Self, Self::Error> {
        match value {
            Some(value) => Result::Ok(value.try_into()?),
            None => Result::Err(ConvertError::from(
                "Unable to convert an empty Option to TodoContent!",
            )),
        }
    }
}

impl<T> TryFrom<TodoContent> for Option<T>
where
    T: TryFrom<TodoContent, Error = ConvertError>,
{
    type Error = ConvertError;

    fn try_from(value: TodoContent) -> Result<Self, Self::Error> {
        let value: T = value.try_into()?;
        Result::Ok(Some(value))
    }
}
