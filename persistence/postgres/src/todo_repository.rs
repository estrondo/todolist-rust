use crate::entities::todo::{Column, Entity, Metadata};

use super::convert::db_err_to_persistence_error;
use super::entities::todo::{ActiveModel, Model};
use async_trait::async_trait;
use migration::OnConflict;
use sea_orm::ActiveValue::Set;
use sea_orm::{DatabaseConnection, EntityTrait};
use time::PrimitiveDateTime;
use todolist_core::error::PersistenceError;
use todolist_core::generator::TimeGenerator;
use todolist_core::model::todo::{Todo, TodoId};
use todolist_core::persistence::TodoRepository;

#[derive(Clone)]
pub struct PostgresTodoRepository<T: TimeGenerator> {
    connection: DatabaseConnection,
    time_generator: T,
}

impl<T: TimeGenerator> PostgresTodoRepository<T> {
    pub fn new(connection: DatabaseConnection, time_generator: T) -> Self {
        Self {
            connection,
            time_generator,
        }
    }
}

#[async_trait]
impl<T: TimeGenerator> TodoRepository for PostgresTodoRepository<T> {
    async fn get(&self, id: &TodoId) -> Result<Option<Todo>, PersistenceError> {
        let opt = Entity::find_by_id(id.0)
            .one(&self.connection)
            .await
            .map_err(db_err_to_persistence_error)?;

        match opt {
            Some(model) => Ok(Some(model.try_into()?)),
            None => Ok(None),
        }
    }

    async fn upsert(&self, todo: &Todo) -> Result<Todo, PersistenceError> {
        let metadata = new_metadata(&self.time_generator);

        let model: Model = (todo, metadata).try_into()?;
        let active_model: ActiveModel = model.into();
        let returned = Entity::insert(active_model)
            .on_conflict(
                OnConflict::column(Column::Id)
                    .update_columns([
                        Column::Title,
                        Column::Status,
                        Column::DueDateWholeDay,
                        Column::DueDatePeriodStart,
                        Column::DueDatePeriodDuration,
                        Column::ContentMarkdown,
                        Column::ContentPlainText,
                        Column::UpdatedAt,
                    ])
                    .to_owned(),
            )
            .exec_with_returning(&self.connection)
            .await
            .map_err(db_err_to_persistence_error)?
            .try_into()?;

        Ok(returned)
    }
    async fn remove(&self, _todo_id: &TodoId) -> Result<Option<Todo>, PersistenceError> {
        let model = Entity::delete(ActiveModel {
            id: Set(_todo_id.0.to_owned()),
            ..Default::default()
        })
        .exec_with_returning(&self.connection)
        .await
        .map_err(db_err_to_persistence_error)?;

        if let Some(value) = model {
            Ok(Some(value.try_into()?))
        } else {
            Result::Ok(None)
        }
    }
}

fn new_metadata<T: TimeGenerator>(time_generator: &T) -> Metadata {
    let now = time_generator.new_utc_date_time();
    let now = PrimitiveDateTime::new(now.date(), now.time());
    Metadata {
        created_at: now,
        updated_at: now,
    }
}
