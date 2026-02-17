use time::Time;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TodoId(Uuid);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TodoTitle(String);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TodoDueDate(Time);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TodoStatus {
    Unspecified,
    Active,
    Postponed,
    Cancelled,
    Done,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TodoContent {
    Markdown(String),
    Plain(String),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Todo {
    id: TodoId,
    title: TodoTitle,
    due_date: TodoDueDate,
    status: TodoStatus,
    content: TodoContent,
}
