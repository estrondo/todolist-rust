use checkito::{FullGenerate, Generate, boxed::Boxed};

use crate::model::{todo::TodoId, user::UserId};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TodoPermission {
    pub todo_id: TodoId,
    pub user_id: UserId,
    pub role: TodoPermissionRole,
}

impl TodoPermission {
    pub fn new(todo_id: TodoId, user_id: UserId, role: TodoPermissionRole) -> Self {
        Self {
            todo_id,
            user_id,
            role,
        }
    }
    pub fn new_owner(todo_id: TodoId, user_id: UserId) -> Self {
        Self::new(todo_id, user_id, TodoPermissionRole::Owner)
    }

    pub fn new_edit(todo_id: TodoId, user_id: UserId) -> Self {
        Self::new(todo_id, user_id, TodoPermissionRole::Edit)
    }

    pub fn new_view(todo_id: TodoId, user_id: UserId) -> Self {
        Self::new(todo_id, user_id, TodoPermissionRole::View)
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TodoPermissionRole {
    Owner,
    View,
    Edit,
}

impl TodoPermissionRole {
    pub fn can_edit(&self) -> bool {
        match self {
            TodoPermissionRole::Owner => true,
            TodoPermissionRole::Edit => true,
            TodoPermissionRole::View => false,
        }
    }
}

impl FullGenerate for TodoPermissionRole {
    type Item = TodoPermissionRole;

    type Generator = Boxed<TodoPermissionRole>;

    fn generator() -> Self::Generator {
        let edit = <()>::generator().map(|_| TodoPermissionRole::Edit);
        let owner = <()>::generator().map(|_| TodoPermissionRole::Owner);
        let view = <()>::generator().map(|_| TodoPermissionRole::View);

        Generate::any((owner, edit, view)).map(|e| e.into()).boxed()
    }
}

impl FullGenerate for TodoPermission {
    type Item = TodoPermission;

    type Generator = Boxed<TodoPermission>;

    fn generator() -> Self::Generator {
        let todo_id = TodoId::generator();
        let user_id = UserId::generator();
        let role = TodoPermissionRole::generator();

        Generate::map((todo_id, user_id, role), |(todo_id, user_id, role)| {
            TodoPermission {
                todo_id,
                user_id,
                role,
            }
        })
        .boxed()
    }
}
