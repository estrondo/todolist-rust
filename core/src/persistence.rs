use async_trait::async_trait;

use crate::Result;
use crate::error::PersistenceError;
use crate::model::{Todo, TodoId};

#[async_trait]
pub trait TodoRepository {
    async fn get(&self, id: &TodoId) -> Result<Option<Todo>, PersistenceError>;
    async fn upsert(&self, todo: &Todo) -> Result<Todo, PersistenceError>;
    async fn delete(&self, todo_id: &TodoId) -> Result<Option<Todo>, PersistenceError>;
}
