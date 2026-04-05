use async_trait::async_trait;
use mockall::automock;

use crate::Result;
use crate::error::PersistenceError;
use crate::model::todo::{Todo, TodoId};

#[async_trait]
#[automock]
pub trait TodoRepository: Send + Sync {
    async fn get(&self, id: &TodoId) -> Result<Option<Todo>, PersistenceError>;
    async fn upsert(&self, todo: &Todo) -> Result<Todo, PersistenceError>;
    async fn delete(&self, todo_id: &TodoId) -> Result<Option<Todo>, PersistenceError>;
}
