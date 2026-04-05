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
