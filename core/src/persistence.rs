use async_trait::async_trait;

#[cfg(test)]
use mockall::automock;

use crate::error::PersistenceError;
use crate::model::permission::TodoPermission;
use crate::model::todo::{Todo, TodoId};
use crate::model::user::{User, UserId};

pub type PersistenceResult<T> = Result<T, PersistenceError>;

#[async_trait]
#[cfg_attr(test, automock)]
pub trait TodoRepository: Send + Sync {
    async fn get(&self, id: &TodoId) -> PersistenceResult<Option<Todo>>;
    async fn upsert(&self, todo: &Todo) -> PersistenceResult<Todo>;
    async fn remove(&self, todo_id: &TodoId) -> PersistenceResult<Option<Todo>>;
}

#[async_trait]
#[cfg_attr(test, automock)]
pub trait TodoPermissionRepository: Send + Sync {
    async fn get(
        &self,
        todo_id: &TodoId,
        user_id: &UserId,
    ) -> PersistenceResult<Option<TodoPermission>>;
}

#[async_trait]
#[cfg_attr(test, automock)]
pub trait AuthRepository: Send + Sync {
    async fn search(&self, id: &UserId) -> PersistenceResult<Option<User>>;
}
