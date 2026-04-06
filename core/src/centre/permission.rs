use async_trait::async_trait;
use mockall::automock;

use crate::{
    centre::CentreResult,
    model::{permission::TodoPermission, todo::TodoId, user::UserId},
    persistence::TodoPermissionRepository,
};

#[async_trait]
#[automock]
pub trait PermissionCentre: Send + Sync {
    async fn get_todo_permission(
        &self,
        todo_id: &TodoId,
        user_id: &UserId,
    ) -> CentreResult<Option<TodoPermission>>;

    async fn insert_todo_permission(
        &self,
        todo_permission: TodoPermission,
    ) -> CentreResult<TodoPermission>;

    async fn remove_todo_permission(
        &self,
        todo_permission: &TodoPermission,
    ) -> CentreResult<Option<TodoPermission>>;
}

pub struct DefaultPermissionCentre<P> {
    _permission: P,
}

impl<P> DefaultPermissionCentre<P> {
    pub fn new(permission: P) -> Self {
        Self {
            _permission: permission,
        }
    }
}

#[async_trait]
impl<P> PermissionCentre for DefaultPermissionCentre<P>
where
    P: TodoPermissionRepository,
{
    async fn get_todo_permission(
        &self,
        _todo_id: &TodoId,
        _user_id: &UserId,
    ) -> CentreResult<Option<TodoPermission>> {
        todo!()
    }

    async fn insert_todo_permission(
        &self,
        _todo_permission: TodoPermission,
    ) -> CentreResult<TodoPermission> {
        unimplemented!()
    }

    async fn remove_todo_permission(
        &self,
        _todo_permission: &TodoPermission,
    ) -> CentreResult<Option<TodoPermission>> {
        unimplemented!()
    }
}
