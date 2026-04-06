use async_trait::async_trait;
use mockall::automock;

use crate::error::PersistenceError;
use crate::model::permission::TodoPermission;
use crate::model::todo::{Todo, TodoId};
use crate::model::user::{User, UserId};

pub type PersistenceResult<T> = Result<T, PersistenceError>;

#[async_trait]
#[automock]
pub trait TodoRepository: Send + Sync {
    async fn get(&self, id: &TodoId) -> PersistenceResult<Option<Todo>>;
    async fn upsert(&self, todo: &Todo) -> PersistenceResult<Todo>;
    async fn remove(&self, todo_id: &TodoId) -> PersistenceResult<Option<Todo>>;
}

#[async_trait]
#[automock]
pub trait TodoPermissionRepository: Send + Sync {
    async fn get<'a>(
        &self,
        todo_id: &'a TodoId,
        user_id: &'a UserId,
    ) -> PersistenceResult<Option<TodoPermission>>;
}

#[async_trait]
#[automock]
pub trait AuthRepository: Send + Sync {
    async fn search(&self, id: &UserId) -> PersistenceResult<Option<User>>;
}
