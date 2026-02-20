use std::sync::Arc;

use async_trait::async_trait;
use sea_orm::DatabaseConnection;
use todolist_core::Result;
use todolist_core::error::PersistenceError;
use todolist_core::{
    model::{Todo, TodoId},
    persistence::{TodoRepository, UpsertResult},
};

pub struct PostgresTodoRepository {
    connection: Arc<DatabaseConnection>,
}

impl PostgresTodoRepository {
    pub fn new(connection: Arc<DatabaseConnection>) -> Self {
        Self { connection }
    }
}

#[async_trait]
impl TodoRepository for PostgresTodoRepository {
    async fn upsert(&self, todo: &Todo) -> Result<UpsertResult<Todo>, PersistenceError> {
        unimplemented!()
    }
    async fn delete(&self, todo_id: &TodoId) -> Result<Option<Todo>, PersistenceError> {
        unimplemented!()
    }
}
