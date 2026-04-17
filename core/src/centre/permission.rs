use async_trait::async_trait;

#[cfg(test)]
use mockall::automock;

use crate::{
    centre::{CentreError, CentreResult},
    model::{permission::TodoPermission, todo::TodoId, user::UserId},
    persistence::TodoPermissionRepository,
};

#[async_trait]
#[cfg_attr(test, automock)]
pub trait PermissionCentre: Send + Sync {
    async fn get_todo_permission(
        &self,
        todo_id: &TodoId,
        user_id: &UserId,
    ) -> CentreResult<Option<TodoPermission>>;

    async fn insert_todo_permission(
        &self,
        todo_permission: &TodoPermission,
    ) -> CentreResult<TodoPermission>;

    async fn remove_todo_permission(
        &self,
        todo_permission: &TodoPermission,
    ) -> CentreResult<Option<TodoPermission>>;
}

pub struct DefaultPermissionCentre<P> {
    permission: P,
}

impl<P> DefaultPermissionCentre<P> {
    pub fn new(permission: P) -> Self {
        Self { permission }
    }
}

#[async_trait]
impl<P> PermissionCentre for DefaultPermissionCentre<P>
where
    P: TodoPermissionRepository,
{
    async fn get_todo_permission(
        &self,
        todo_id: &TodoId,
        user_id: &UserId,
    ) -> CentreResult<Option<TodoPermission>> {
        log::debug!(todo:?=todo_id, user:?=user_id; "Looking for todo permission");
        match self.permission.get(todo_id, user_id).await? {
            result @ Some(_) => {
                log::debug!(todo:?=todo_id, user:?=user_id; "Todo permission was found");
                Ok(result)
            }
            result @ None => {
                log::warn!(todo:?=todo_id, user:?=user_id;"Todo permission not found");
                Ok(result)
            }
        }
    }

    async fn insert_todo_permission(
        &self,
        todo_permission: &TodoPermission,
    ) -> CentreResult<TodoPermission> {
        log::debug!(todo:?=todo_permission.todo_id,user:?=todo_permission.user_id;"Inserting a new todo permission");
        match self.permission.upsert(&todo_permission).await {
            Ok(result) => CentreResult::Ok(result),
            Err(cause) => {
                log::error!(todo:?=todo_permission.todo_id,user:?=todo_permission.user_id;"Unable to insert the permission");
                Err(CentreError::Unexpected(
                    "Unable to insert the permission".into(),
                    Some(Box::new(cause)),
                ))
            }
        }
    }

    async fn remove_todo_permission(
        &self,
        todo_permission: &TodoPermission,
    ) -> CentreResult<Option<TodoPermission>> {
        log::info!(todo:?=todo_permission.todo_id,user:?=todo_permission.user_id; "Removing todo permission");
        match self
            .permission
            .remove(&todo_permission.todo_id, &todo_permission.user_id)
            .await
        {
            Ok(result @ Some(_)) => {
                log::info!("Todo permission was removed");
                Ok(result)
            }
            Ok(None) => {
                log::info!("Todo permission was not found");
                Ok(None)
            }
            Err(cause) => {
                log::error!("Unable to remove todo permission: {}", &cause);
                Err(CentreError::from(cause))
            }
        }
    }
}
