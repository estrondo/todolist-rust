use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;

use crate::{
    centre::{CentreError, CentreResult, permission::PermissionCentre},
    model::{
        permission::{TodoPermission, TodoPermissionRole},
        todo::{Todo, TodoId},
        user::UserId,
    },
    persistence::TodoRepository,
};

#[async_trait]
#[automock]
pub trait TodoCentre: Send + Sync {
    async fn upsert(&self, todo: &Todo, user_id: &UserId) -> CentreResult<Todo>;

    async fn remove(&self, todo_id: &TodoId, user_id: &UserId) -> CentreResult<Option<Todo>>;
}

pub struct DefaultTodoCentre<T, P> {
    todo: T,
    permission: P,
}

impl<T, P> DefaultTodoCentre<T, P> {
    pub fn new(todo: T, permission: P) -> Self {
        Self { todo, permission }
    }
}

#[async_trait]
impl<T, P> TodoCentre for DefaultTodoCentre<T, Arc<P>>
where
    T: TodoRepository,
    P: PermissionCentre,
{
    async fn upsert(&self, todo: &Todo, user_id: &UserId) -> CentreResult<Todo> {
        log::info!(todo:?=todo.id, user:?=user_id; "Upserting a todo item");
        let inserted = self.todo.upsert(todo).await?;
        match self
            .permission
            .insert_todo_permission(TodoPermission::new_owner(
                todo.id.to_owned(),
                user_id.to_owned(),
            ))
            .await
        {
            Ok(_) => {
                log::info!(todo:?=todo.id, user:?=user_id; "A new todo item was inserted.");
                Ok(inserted)
            }
            Err(cause) => {
                log::warn!(todo:?=todo.id, user:?=user_id; "Unable to insert a new permission for todo item, removing the todo item: {}", cause);
                let _ = self
                    .todo
                    .remove(&inserted.id)
                    .await
                    .inspect_err(|e| log::warn!("Failed to remove the todo permission: {}", e));
                Err(CentreError::Unexpected(
                    "Unable to insert todo permission".to_owned(),
                    Some(Box::new(cause)),
                ))
            }
        }
    }

    async fn remove(&self, todo_id: &TodoId, user_id: &UserId) -> CentreResult<Option<Todo>> {
        let permission = self
            .permission
            .get_todo_permission(todo_id, user_id)
            .await?;

        match &permission {
            Some(permission) if permission.role == TodoPermissionRole::Owner => {
                match self.todo.remove(todo_id).await {
                    Ok(Some(removed)) => {
                        match self.permission.remove_todo_permission(permission).await {
                            Ok(_) => Ok(Some(removed)),
                            Err(error) => {
                                log::warn!(todo:?=todo_id; "Recovering the todo item due to an failure during the permission removing: {}", &error);
                                let _ = self.todo
                                    .upsert(&removed)
                                    .await
                                    .inspect(|_| log::info!(todo:?=todo_id;"Todo item recovered."))
                                    .inspect_err(|error| {
                                        log::error!(todo:?=todo_id;"Unable to recover todo item: {}", error)
                                    });
                                CentreResult::Err(CentreError::Unexpected("Unable ".into(), None))
                            }
                        }
                    }
                    Ok(None) => {
                        log::warn!(todo:?=todo_id;"There is no todo item, but there is a permission for it. Cleaning up it");
                        match self.permission.remove_todo_permission(&permission).await {
                            Ok(_) => {
                                log::info!(todo:?=todo_id;"Permission for todo has been removed.");
                                CentreResult::Ok(None)
                            }
                            Err(error) => {
                                log::warn!(todo:?=todo_id;"There is a problem. A permission exists for the todo item, but unable to remove it");
                                CentreResult::Err(CentreError::Unexpected(
                                    "Unable to remove an orphan permission.".into(),
                                    Some(Box::new(error)),
                                ))
                            }
                        }
                    }
                    Err(cause) => {
                        log::warn!(todo:?=todo_id;"Unable to remove the todo, an error happened: {}", cause);
                        CentreResult::Err(CentreError::from(cause))
                    }
                }
            }
            _ => {
                log::warn!(todo:?=todo_id; "An unauthorized attempt to remove a todo item");
                return CentreResult::Err(CentreError::Unauthorized(
                    "You are not the owner!".into(),
                    None,
                ));
            }
        }
    }
}
