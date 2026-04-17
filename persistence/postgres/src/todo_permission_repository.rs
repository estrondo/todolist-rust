use async_trait::async_trait;
use migration::OnConflict;
use sea_orm::{ActiveValue::Set, DatabaseConnection, EntityTrait};
use todolist_core::{
    generator::TimeGenerator,
    model::{permission::TodoPermission, todo::TodoId, user::UserId},
    persistence::{PersistenceResult, TodoPermissionRepository},
};

use crate::{
    convert::db_err_to_persistence_error,
    entities::todo_permission::{ActiveModel, Column, Entity, Metadata, Model},
};

#[derive(Debug, Clone)]
pub struct PostgresTodoPermissionRepository<T: TimeGenerator> {
    connection: DatabaseConnection,
    time_generator: T,
}

impl<T: TimeGenerator> PostgresTodoPermissionRepository<T> {
    pub fn new(connection: DatabaseConnection, time_generator: T) -> Self {
        Self {
            connection,
            time_generator,
        }
    }
}

#[async_trait]
impl<T: TimeGenerator> TodoPermissionRepository for PostgresTodoPermissionRepository<T> {
    async fn get(
        &self,
        todo_id: &TodoId,
        user_id: &UserId,
    ) -> PersistenceResult<Option<TodoPermission>> {
        let opt = Entity::find_by_id((todo_id.0, user_id.0))
            .one(&self.connection)
            .await
            .map_err(db_err_to_persistence_error)?;

        match opt {
            Some(model) => Ok(Some(TodoPermission::try_from(model)?)),
            None => Ok(None),
        }
    }

    async fn upsert(&self, todo_permission: &TodoPermission) -> PersistenceResult<TodoPermission> {
        let metadata = new_metadata(&self.time_generator);
        let model = Model::try_from((todo_permission, metadata))?;

        let active_model: ActiveModel = model.into();

        let model = Entity::insert(active_model)
            .on_conflict(
                OnConflict::columns([Column::TodoId, Column::UserId])
                    .update_columns([Column::Role])
                    .to_owned(),
            )
            .exec_with_returning(&self.connection)
            .await
            .map_err(db_err_to_persistence_error)?;

        Ok(model.try_into()?)
    }

    async fn remove(
        &self,
        todo_id: &TodoId,
        user_id: &UserId,
    ) -> PersistenceResult<Option<TodoPermission>> {
        let result = Entity::delete(ActiveModel {
            todo_id: Set(todo_id.0),
            user_id: Set(user_id.0),
            ..Default::default()
        })
        .exec_with_returning(&self.connection)
        .await
        .map_err(db_err_to_persistence_error)?;

        match result {
            Some(model) => Ok(Some(model.try_into()?)),
            None => Ok(None),
        }
    }
}

fn new_metadata<T: TimeGenerator>(generator: &T) -> Metadata {
    let now = generator.new_utc_primitive_date_time();
    Metadata {
        created_at: now,
        updated_at: now,
    }
}
