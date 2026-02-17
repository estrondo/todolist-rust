use crate::service::todo::DefaultTodoService;
use crate::{configuration::Configuration, module::manager::ManagerModule};

pub struct ServiceModule<'c> {
    configuration: &'c Configuration,
    manager_module: &'c ManagerModule<'c>,
}

impl<'c> ServiceModule<'c> {
    pub fn new(configuration: &'c Configuration, manager_module: &'c ManagerModule<'c>) -> Self {
        Self {
            configuration,
            manager_module,
        }
    }

    pub fn todo_service(&self) -> DefaultTodoService {
        DefaultTodoService::new()
    }
}
