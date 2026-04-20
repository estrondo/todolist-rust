use async_trait::async_trait;

use futures::Stream;
#[cfg(test)]
use mockall::automock;
#[cfg(test)]
use mockall::predicate::eq;

use crate::error::PersistenceError;
use crate::model::permission::TodoPermission;
use crate::model::todo::{Todo, TodoId};
use crate::model::user::{User, UserId};

pub type PersistenceResult<T> = Result<T, PersistenceError>;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait TodoRepository: Send + Sync {
    async fn get(&self, id: &TodoId) -> PersistenceResult<Option<Todo>>;
    async fn upsert(&self, todo: &Todo) -> PersistenceResult<Todo>;
    async fn remove(&self, todo_id: &TodoId) -> PersistenceResult<Option<Todo>>;
}

#[cfg(test)]
impl MockTodoRepository {
    pub fn once_success_upsert(&mut self, todo: &Todo) {
        let todo = todo.to_owned();
        self.expect_upsert()
            .once()
            .with(eq(todo.to_owned()))
            .returning(|a| PersistenceResult::Ok(a.to_owned()));
    }
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait TodoPermissionRepository: Send + Sync {
    async fn get(
        &self,
        todo_id: &TodoId,
        user_id: &UserId,
    ) -> PersistenceResult<Option<TodoPermission>>;

    async fn upsert(&self, todo_permission: &TodoPermission) -> PersistenceResult<TodoPermission>;

    async fn search_permissions<'s>(
        &'s self,
        todo_id: &TodoId,
    ) -> PersistenceResult<
        Box<dyn Stream<Item = PersistenceResult<TodoPermission>> + 's + Send + Unpin>,
    >;

    async fn remove(
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
