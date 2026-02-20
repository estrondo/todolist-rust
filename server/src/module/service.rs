use crate::{
    configuration::Configuration, module::manager::ManagerModule, service::DefaultTodoService,
};
use todolist_core::Result;

pub struct ServiceModule<'a> {
    manager_module: &'a ManagerModule,
}

impl<'a> ServiceModule<'a> {
    pub fn new(_: &'a Configuration, manager_module: &'a ManagerModule) -> Result<Self> {
        Result::Ok(Self { manager_module })
    }

    pub fn todo_service(&self) -> DefaultTodoService {
        DefaultTodoService::new(self.manager_module.todo_manager())
    }
}
