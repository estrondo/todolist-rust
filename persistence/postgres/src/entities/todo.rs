use sea_orm::entity::prelude::*;
use time::{Date, Duration};
use todolist_core::{
    error::PersistenceError,
    model::todo::{Todo, TodoContent, TodoDueDate, TodoId, TodoStatus, TodoTitle},
};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "todo")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub title: String,
    pub due_date_whole_day: Option<Date>,
    pub due_date_period_start: Option<TimeDateTime>,
    pub due_date_period_duration: Option<u32>,
    pub status: i16,
    pub content_markdown: Option<String>,
    pub content_plain_text: Option<String>,
    pub created_at: TimeDateTime,
    pub updated_at: TimeDateTime,
}

pub(crate) struct Metadata {
    pub created_at: TimeDateTime,
    pub updated_at: TimeDateTime,
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
            (Some(whole_day), None, None) => TodoDueDate::WholeDay(whole_day),
            (None, Some(period_start), Some(period_duration)) => TodoDueDate::Period(
                period_start.as_utc(),
                Duration::seconds(period_duration.into()),
            ),
            _ => {
                return Err(PersistenceError::InvalidState(String::from(
                    "Invalid due_date.",
                )));
            }
        };

        let content = match (value.content_markdown, value.content_plain_text) {
            (Some(markdown), None) => TodoContent::Markdown(markdown),
            (None, Some(plain_text)) => TodoContent::Plain(plain_text),
            _ => {
                return Err(PersistenceError::InvalidState(String::from(
                    "Invalid content.",
                )));
            }
        };

        Ok(Todo {
            id: TodoId(value.id),
            title: TodoTitle(value.title),
            due_date,
            status: TodoStatus::try_from(value.status as u8).map_err(|value| {
                PersistenceError::InvalidState(format!("Invalid status value: {value}"))
            })?,
            content,
        })
    }
}

impl TryFrom<(&Todo, Metadata)> for Model {
    type Error = PersistenceError;

    fn try_from((value, metadata): (&Todo, Metadata)) -> Result<Self, Self::Error> {
        let id = value.id.0.to_owned();
        let title = value.title.0.to_owned();
        let (due_date_whole_day, due_date_period_start, due_date_period_duration) =
            match &value.due_date {
                TodoDueDate::WholeDay(date) => (Some(date.to_owned()), None, None),
                TodoDueDate::Period(utc_date_time, duration) => (
                    None,
                    Some(TimeDateTime::new(
                        utc_date_time.date(),
                        utc_date_time.time(),
                    )),
                    Some(duration.whole_seconds() as u32),
                ),
            };

        let (content_markdown, content_plain_text) = match &value.content {
            TodoContent::Markdown(value) => (Some(value.into()), None),
            TodoContent::Plain(value) => (None, Some(value.into())),
        };

        let status = u8::from(&value.status) as i16;

        Ok(Self {
            id,
            title,
            due_date_whole_day,
            due_date_period_start,
            due_date_period_duration,
            status,
            content_markdown,
            content_plain_text,
            created_at: metadata.created_at,
            updated_at: metadata.updated_at,
        })
    }
}
