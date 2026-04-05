use sea_orm::entity::prelude::*;
use time::{Date, Duration, UtcDateTime};
use todolist_core::{error::PersistenceError, model::todo::{Todo, TodoContent, TodoDueDate, TodoId, TodoStatus, TodoTitle}};

use crate::field::{F, FO};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "todo")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: F<TodoId>,
    pub title: F<TodoTitle>,
    pub due_date_whole_day: FO<Date>,
    pub due_date_period_start: FO<UtcDateTime>,
    pub due_date_period_duration: FO<Duration>,
    pub status: F<TodoStatus>,
    pub content_markdown: FO<String>,
    pub content_plain_text: FO<String>,
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
            (FO::Some(F(whole_day)), FO::None, FO::None) => TodoDueDate::WholeDay(whole_day),
            (FO::None, FO::Some(F(period_start)), FO::Some(F(period_duration))) => {
                TodoDueDate::Period(period_start, period_duration)
            }
            _ => {
                return Err(PersistenceError::InvalidState {
                    message: String::from("Invalid due_date."),
                });
            }
        };

        let content = match (value.content_markdown, value.content_plain_text) {
            (FO::Some(F(markdown)), FO::None) => TodoContent::Markdown(markdown),
            (FO::None, FO::Some(F(plain_text))) => TodoContent::Plain(plain_text),
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
        let id = F::from(&value.id);
        let title = F::from(&value.title);
        let (due_date_whole_day, due_date_period_start, due_date_period_duration) =
            match &value.due_date {
                TodoDueDate::WholeDay(date) => (FO::Some(F::from(date)), FO::None, FO::None),
                TodoDueDate::Period(utc_date_time, duration) => (
                    FO::None,
                    FO::Some(F::from(utc_date_time)),
                    FO::Some(F::from(duration)),
                ),
            };

        let (content_markdown, content_plain_text) = match &value.content {
            TodoContent::Markdown(value) => (FO::Some(F::from(value)), FO::None),
            TodoContent::Plain(value) => (FO::None, FO::Some(F::from(value))),
        };

        let status = F::from(&value.status);

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
