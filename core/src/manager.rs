use async_trait::async_trait;

use crate::Result;
use crate::error::ManagerError;
use crate::model::{Todo, TodoId};

mod default;

pub type ManagerResult<T> = Result<T, ManagerError>;

pub use default::PersistentTodoManager;

#[async_trait]
pub trait TodoManager: Send + Sync {
    async fn upsert<'a>(&self, todo: &'a Todo) -> ManagerResult<Todo>;

    async fn remove<'a>(&self, todo_id: &'a TodoId) -> ManagerResult<Option<Todo>>;
}
