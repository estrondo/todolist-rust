use async_trait::async_trait;
use sea_orm::DatabaseConnection;
use todolist_core::{
    model::{permission::TodoPermission, todo::TodoId, user::UserId},
    persistence::{PersistenceResult, TodoPermissionRepository},
};

pub struct PostgresTodoPermissionRepository {
    _connection: DatabaseConnection,
}

impl PostgresTodoPermissionRepository {
    pub fn new(connection: DatabaseConnection) -> Self {
        Self { _connection: connection }
    }
}

#[async_trait]
impl TodoPermissionRepository for PostgresTodoPermissionRepository {
    async fn get<'a>(
        &self,
        _todo_id: &'a TodoId,
        _user_id: &'a UserId,
    ) -> PersistenceResult<Option<TodoPermission>> {
        todo!()
    }
}
