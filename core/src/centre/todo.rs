use std::{fmt::Debug, sync::Arc};

use async_trait::async_trait;

#[cfg(test)]
use mockall::automock;
use tracing::instrument;

use crate::{
    centre::{CentreError, CentreResult, permission::PermissionCentre},
    model::{
        permission::{TodoPermission, TodoPermissionRole},
        todo::{Todo, TodoId},
        user::UserId,
    },
    repositories::todo::TodoRepository,
};

#[async_trait]
#[cfg_attr(test, automock)]
pub trait TodoCentre: Send + Sync + Debug {
    async fn upsert(&self, todo: &Todo, user_id: &UserId) -> CentreResult<Todo>;

    async fn remove(&self, todo_id: &TodoId, user_id: &UserId) -> CentreResult<Option<Todo>>;
}

#[derive(Debug)]
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
    #[instrument(name="default-todo-centre.upsert",skip_all, fields(todo.id=%todo.id.0, user.id=%user_id.0))]
    async fn upsert(&self, todo: &Todo, user_id: &UserId) -> CentreResult<Todo> {
        let todo_permission = self
            .permission
            .get(&todo.id, user_id)
            .await
            .map_err(|cause| {
                CentreError::Unexpected("Unable to get permission".into(), Some(Box::new(cause)))
            })?;

        match todo_permission {
            Some(todo_permission) if todo_permission.role.can_edit() => {
                log::info!("Upserting todo");
                match self.todo.upsert(todo).await {
                    Ok(upserted) => Ok(upserted),
                    Err(cause) => Err(CentreError::from(cause)),
                }
            }
            Some(_) => {
                log::warn!("An unauthorised attempt to edit a todo item");
                Err(CentreError::Unauthorized(
                    "Unable to edit the todo item".into(),
                    None,
                ))
            }
            None => match self.permission.has_owner(&todo.id).await {
                Ok(true) => {
                    log::error!("Security error. An attempt to edit an invalid todo item");
                    Err(CentreError::Unauthorized("Not allowed".into(), None))
                }
                Ok(_) => {
                    log::info!("Considering the todo item as a brand new one");
                    match self
                        .permission
                        .upsert(&TodoPermission::new_owner(
                            todo.id.to_owned(),
                            user_id.to_owned(),
                        ))
                        .await
                    {
                        Ok(_) => {
                            log::info!("A new permission was added, upserting todo item.");
                            match self.todo.upsert(todo).await {
                                Ok(inserted) => Ok(inserted),
                                Err(cause) => Err(CentreError::from(cause)),
                            }
                        }
                        Err(cause) => {
                            log::error!("Unable to create a new permission: {cause}");
                            Err(CentreError::from(cause))
                        }
                    }
                }
                Err(cause) => {
                    log::error!("Unable to check permissions: {cause}");
                    Err(CentreError::from(cause))
                }
            },
        }
    }

    #[instrument(name="default-todo-centre.remove",skip_all, fields(todo.id=%todo_id.0, user.id=%user_id.0))]
    async fn remove(&self, todo_id: &TodoId, user_id: &UserId) -> CentreResult<Option<Todo>> {
        let permission = self.permission.get(todo_id, user_id).await?;

        match &permission {
            Some(permission) if permission.role == TodoPermissionRole::Owner => {
                match self.todo.remove(todo_id).await {
                    Ok(Some(removed)) => match self.permission.remove(permission).await {
                        Ok(_) => Ok(Some(removed)),
                        Err(error) => {
                            log::warn!(todo:?=todo_id; "Recovering the todo item due to an failure during the permission removing: {}", &error);
                            let _ = self
                                .todo
                                .upsert(&removed)
                                .await
                                .inspect(|_| log::info!("Todo item recovered."))
                                .inspect_err(|error| {
                                    log::error!("Unable to recover todo item: {}", error)
                                });
                            CentreResult::Err(CentreError::Unexpected("Unable ".into(), None))
                        }
                    },
                    Ok(None) => {
                        log::warn!(
                            "There is no todo item, but there is a permission for it. Cleaning up it"
                        );
                        match self.permission.remove(&permission).await {
                            Ok(_) => {
                                log::info!("Permission for todo has been removed.");
                                CentreResult::Ok(None)
                            }
                            Err(error) => {
                                log::warn!(
                                    "There is a problem. A permission exists for the todo item, but unable to remove it"
                                );
                                CentreResult::Err(CentreError::Unexpected(
                                    "Unable to remove an orphan permission.".into(),
                                    Some(Box::new(error)),
                                ))
                            }
                        }
                    }
                    Err(cause) => {
                        log::warn!("Unable to remove the todo, an error happened: {}", cause);
                        CentreResult::Err(CentreError::from(cause))
                    }
                }
            }
            _ => {
                log::warn!("An unauthorized attempt to remove a todo item");
                return CentreResult::Err(CentreError::Unauthorized(
                    "You are not the owner!".into(),
                    None,
                ));
            }
        }
    }
}
