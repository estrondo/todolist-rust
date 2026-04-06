use std::{fmt::Error, sync::Arc};

use crate::{configuration::Configuration, module::repository::RepositoryModule};
use todolist_core::centre::{
    permission::{DefaultPermissionCentre, PermissionCentre},
    todo::{DefaultTodoCentre, TodoCentre},
};

pub struct CentreModule {
    todo: Arc<dyn TodoCentre>,
    permission: Arc<dyn PermissionCentre>,
}

impl<'c> CentreModule {
    pub fn new(
        _configuration: &Configuration,
        repository_module: &RepositoryModule,
    ) -> Result<Self, Error> {
        let permission = Arc::new(DefaultPermissionCentre::new(
            repository_module.todo_permission_repository(),
        ));

        let todo = Arc::new(DefaultTodoCentre::new(
            repository_module.todo_repository(),
            permission.clone(),
        ));

        Result::Ok(Self { todo, permission })
    }

    pub fn todo_centre(&self) -> Arc<dyn TodoCentre> {
        self.todo.clone()
    }

    pub fn permission_centre(&self) -> Arc<dyn PermissionCentre> {
        self.permission.clone()
    }
}
