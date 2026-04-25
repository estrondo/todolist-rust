use std::fmt::Debug;

use async_trait::async_trait;

use futures::StreamExt;
#[cfg(test)]
use mockall::{automock, predicate::eq};
use tracing::instrument;

#[cfg(test)]
use crate::model::{permission::TodoPermissionRole, todo::Todo};
use crate::{
    centre::{CentreError, CentreResult},
    model::{permission::TodoPermission, todo::TodoId, user::UserId},
    repositories::permission::TodoPermissionRepository,
};

#[cfg_attr(test, automock)]
#[async_trait]
pub trait PermissionCentre: Send + Sync + Debug {
    async fn has_owner(&self, todo_id: &TodoId) -> CentreResult<bool>;

    async fn get(&self, todo_id: &TodoId, user_id: &UserId)
    -> CentreResult<Option<TodoPermission>>;

    async fn upsert(&self, todo_permission: &TodoPermission) -> CentreResult<TodoPermission>;

    async fn remove(
        &self,
        todo_permission: &TodoPermission,
    ) -> CentreResult<Option<TodoPermission>>;
}

#[derive(Debug)]
pub struct DefaultPermissionCentre<P> {
    permission: P,
}

#[cfg(test)]
impl MockPermissionCentre {
    pub fn once_success_get(&mut self, todo: &Todo, user_id: &UserId, role: TodoPermissionRole) {
        self.expect_get()
            .once()
            .with(eq(todo.id.to_owned()), eq(user_id.to_owned()))
            .returning(move |_a, _b| {
                let role = role.to_owned();
                CentreResult::Ok(Some(TodoPermission {
                    todo_id: _a.to_owned(),
                    user_id: _b.to_owned(),
                    role,
                }))
            });
    }
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
    #[instrument(name="default-permission-centre.get",skip_all, fields(todo.id=%todo_id.0, user.id=%user_id.0))]
    async fn get(
        &self,
        todo_id: &TodoId,
        user_id: &UserId,
    ) -> CentreResult<Option<TodoPermission>> {
        log::debug!("Looking for todo permission");
        match self.permission.get(todo_id, user_id).await? {
            result @ Some(_) => {
                log::debug!("Todo permission was found");
                Ok(result)
            }
            result @ None => {
                log::warn!("Todo permission not found");
                Ok(result)
            }
        }
    }

    #[instrument(name="default-permission-centre.has-owner",skip_all, fields(todo.id=%todo_id.0))]
    async fn has_owner(&self, todo_id: &TodoId) -> CentreResult<bool> {
        // TODO: I just wanted to make something with Stream pretty early (Actually I forgot what I needed and used stream with no brain).
        let stream = self.permission.search_permission_by_todo_id(todo_id).await;

        match stream {
            Ok(mut stream) => {
                log::debug!("Reading permission");

                while let Some(result) = stream.next().await {
                    match result {
                        Ok(_) => return Ok(true),
                        Err(_) => {}
                    }
                }

                CentreResult::Ok(false)
            }
            Err(cause) => {
                log::warn!("Unable to search the permission");
                CentreResult::Err(CentreError::from(cause))
            }
        }
    }

    #[instrument(name="default-permission-centre.upsert",skip_all, fields(todo.id=%todo_permission.todo_id.0, user.id=%todo_permission.user_id.0))]
    async fn upsert(&self, todo_permission: &TodoPermission) -> CentreResult<TodoPermission> {
        log::debug!("Inserting a new todo permission");
        match self.permission.upsert(&todo_permission).await {
            Ok(result) => CentreResult::Ok(result),
            Err(cause) => {
                log::error!("Unable to insert the permission");
                Err(CentreError::Unexpected(
                    "Unable to insert the permission".into(),
                    Some(Box::new(cause)),
                ))
            }
        }
    }

    #[instrument(name="default-permission-centre.remove",skip_all, fields(todo.id=%todo_permission.todo_id.0, user.id=%todo_permission.user_id.0))]
    async fn remove(
        &self,
        todo_permission: &TodoPermission,
    ) -> CentreResult<Option<TodoPermission>> {
        log::info!("Removing todo permission");
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
