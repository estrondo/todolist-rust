use crate::entities::todo::{Column, Entity};
use crate::field::F;

use super::convert::db_err_to_persistence_error;
use super::entities::todo::{ActiveModel, Model};
use async_trait::async_trait;
use migration::OnConflict;
use sea_orm::{DatabaseConnection, EntityTrait};
use todolist_core::error::PersistenceError;
use todolist_core::model::todo::{Todo, TodoId};
use todolist_core::persistence::TodoRepository;

#[derive(Clone)]
pub struct PostgresTodoRepository {
    connection: DatabaseConnection,
}

impl PostgresTodoRepository {
    pub fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }
}

#[async_trait]
impl TodoRepository for PostgresTodoRepository {
    async fn get(&self, id: &TodoId) -> Result<Option<Todo>, PersistenceError> {
        let opt = Entity::find_by_id(F::from(id))
            .one(&self.connection)
            .await
            .map_err(db_err_to_persistence_error)?;

        match opt {
            Some(model) => model.try_into().map(Some),
            None => Ok(None),
        }
    }

    async fn upsert(&self, todo: &Todo) -> Result<Todo, PersistenceError> {
        let model: Model = todo.try_into()?;
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
        unimplemented!()
    }
}
