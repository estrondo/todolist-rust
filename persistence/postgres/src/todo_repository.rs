use super::convert::db_err_to_persistence_error;
use super::entities::todo::{ActiveModel, Model};
use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, DatabaseConnection};
use todolist_core::Result;
use todolist_core::error::PersistenceError;
use todolist_core::{
    model::{Todo, TodoId},
    persistence::{TodoRepository, UpsertResult},
};

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
    async fn upsert(&self, todo: &Todo) -> Result<UpsertResult<Todo>, PersistenceError> {
        let model: Model = todo.try_into()?;
        let active_model: ActiveModel = model.into();
        let inserted: Todo = active_model
            .insert(&self.connection)
            .await
            .map_err(db_err_to_persistence_error)?
            .try_into()?;

        Ok(UpsertResult::Inserted(inserted))
    }
    async fn delete(&self, _todo_id: &TodoId) -> Result<Option<Todo>, PersistenceError> {
        unimplemented!()
    }
}
