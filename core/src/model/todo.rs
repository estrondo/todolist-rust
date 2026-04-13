use log::kv::{ToValue, Value};
use time::{Date, Duration, UtcDateTime};
use uuid::Uuid;

pub mod foreign;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct TodoId(pub Uuid);

impl ToValue for TodoId {
    fn to_value(&self) -> log::kv::Value<'_> {
        Value::from_display(&self.0)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct TodoTitle(pub String);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum TodoDueDate {
    WholeDay(Date),
    Period(UtcDateTime, Duration),
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum TodoStatus {
    Unspecified = 0,
    Active = 1,
    Postponed = 2,
    Cancelled = 3,
    Done = 4,
}

impl TryFrom<u8> for TodoStatus {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TodoStatus::Unspecified),
            1 => Ok(TodoStatus::Active),
            2 => Ok(TodoStatus::Postponed),
            3 => Ok(TodoStatus::Cancelled),
            4 => Ok(TodoStatus::Done),
            _ => Err(value),
        }
    }
}

impl From<&TodoStatus> for u8 {
    fn from(value: &TodoStatus) -> Self {
        match value {
            TodoStatus::Unspecified => 0,
            TodoStatus::Active => 1,
            TodoStatus::Postponed => 2,
            TodoStatus::Cancelled => 3,
            TodoStatus::Done => 4,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum TodoContent {
    Markdown(String),
    Plain(String),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Todo {
    pub id: TodoId,
    pub title: TodoTitle,
    pub due_date: TodoDueDate,
    pub status: TodoStatus,
    pub content: TodoContent,
}
