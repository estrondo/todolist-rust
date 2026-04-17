use sea_orm::entity::prelude::*;
use todolist_core::{
    error::PersistenceError,
    model::{permission::TodoPermission, todo::TodoId, user::UserId},
};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "todo_permission")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub todo_id: Uuid,
    #[sea_orm(primary_key)]
    pub user_id: Uuid,
    pub role: i16,
    pub created_at: TimeDateTime,
    pub updated_at: TimeDateTime,
}

impl ActiveModelBehavior for ActiveModel {}

impl TryFrom<(&TodoPermission, Metadata)> for Model {
    type Error = PersistenceError;

    fn try_from((value, metadata): (&TodoPermission, Metadata)) -> Result<Self, Self::Error> {
        Ok(Model {
            todo_id: value.todo_id.0,
            user_id: value.user_id.0,
            role: (<u8>::from(&value.role).into()),
            created_at: metadata.created_at,
            updated_at: metadata.updated_at,
        })
    }
}

impl TryFrom<Model> for TodoPermission {
    type Error = PersistenceError;

    fn try_from(value: Model) -> Result<Self, Self::Error> {
        Ok(Self {
            todo_id: TodoId(value.todo_id),
            user_id: UserId(value.user_id),
            role: (value.role as u8).try_into().map_err(|x| {
                PersistenceError::InvalidState(format!("Invalid TodoPermissionRole: {x}"))
            })?,
        })
    }
}

pub(crate) struct Metadata {
    pub created_at: TimeDateTime,
    pub updated_at: TimeDateTime,
}
