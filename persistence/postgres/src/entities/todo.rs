use sea_orm::entity::prelude::*;
use time::{Date, Duration, UtcDateTime};
use todolist_core::{
    error::PersistenceError,
    model::{Todo, TodoContent, TodoDueDate, TodoId, TodoStatus, TodoTitle},
};

use crate::Field;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "todo")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Field<TodoId>,
    pub title: Field<TodoTitle>,
    #[sea_orm(nullable)]
    pub due_date_whole_day: Option<Field<Date>>,
    #[sea_orm(nullable)]
    pub due_date_period_start: Option<Field<UtcDateTime>>,
    #[sea_orm(nullable)]
    pub due_date_period_duration: Option<Field<Duration>>,
    pub status: Field<TodoStatus>,
    #[sea_orm(nullable)]
    pub content_markdown: Option<Field<String>>,
    #[sea_orm(nullable)]
    pub content_plain_text: Option<Field<String>>,
}

impl ActiveModelBehavior for ActiveModel {}

impl TryFrom<Model> for Todo {
    type Error = PersistenceError;

    fn try_from(value: Model) -> Result<Self, Self::Error> {
        let due_date = match (
            value.due_date_whole_day,
            value.due_date_period_start,
            value.due_date_period_duration,
        ) {
            (Some(whole_day), None, None) => TodoDueDate::WholeDay(whole_day.0),
            (None, Some(period_start), Some(period_duration)) => {
                TodoDueDate::Period(period_start.0, period_duration.0)
            }
            _ => {
                return Err(PersistenceError::InvalidState {
                    message: String::from("Invalid due_date."),
                });
            }
        };

        let content = match (value.content_markdown, value.content_plain_text) {
            (Some(markdown), None) => TodoContent::Markdown(markdown.0),
            (None, Some(plain_text)) => TodoContent::Plain(plain_text.0),
            _ => {
                return Err(PersistenceError::InvalidState {
                    message: String::from("Invalid content."),
                });
            }
        };

        Ok(Todo {
            id: value.id.0,
            title: value.title.0,
            due_date,
            status: value.status.0,
            content,
        })
    }
}

impl TryFrom<&Todo> for Model {
    type Error = PersistenceError;

    fn try_from(value: &Todo) -> Result<Self, Self::Error> {
        let id = Field::from_owned(&value.id);
        let title = Field::from_owned(&value.title);
        let (due_date_whole_day, due_date_period_start, due_date_period_duration) =
            match &value.due_date {
                TodoDueDate::WholeDay(date) => (Some(Field::from_owned(date)), None, None),
                TodoDueDate::Period(utc_date_time, duration) => (
                    None,
                    Some(Field::from_owned(utc_date_time)),
                    Some(Field::from_owned(duration)),
                ),
            };

        let (content_markdown, content_plain_text) = match &value.content {
            TodoContent::Markdown(value) => (Some(Field::from_owned(value)), None),
            TodoContent::Plain(value) => (None, Some(Field::from_owned(value))),
        };

        let status = Field::from_owned(&value.status);

        Ok(Self {
            id,
            title,
            due_date_whole_day,
            due_date_period_start,
            due_date_period_duration,
            status,
            content_markdown,
            content_plain_text,
        })
    }
}
