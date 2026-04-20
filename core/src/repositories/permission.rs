use async_trait::async_trait;

use super::PersistenceResult;
use crate::model::permission::TodoPermission;
use crate::model::todo::TodoId;
use crate::model::user::UserId;
use futures::Stream;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait TodoPermissionRepository: Send + Sync {
    async fn get(
        &self,
        todo_id: &TodoId,
        user_id: &UserId,
    ) -> PersistenceResult<Option<TodoPermission>>;

    async fn upsert(&self, todo_permission: &TodoPermission) -> PersistenceResult<TodoPermission>;

    async fn search_permission_by_todo_id<'s>(
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
