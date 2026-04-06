use std::fmt::Error;

use crate::{
    configuration::Configuration, module::manager::CentreModule, service::DefaultTodoService,
};

pub struct ServiceModule<'a> {
    manager_module: &'a CentreModule,
}

impl<'a> ServiceModule<'a> {
    pub fn new(_: &'a Configuration, manager_module: &'a CentreModule) -> Result<Self, Error> {
        Result::Ok(Self { manager_module })
    }

    pub fn todo_service(&self) -> DefaultTodoService {
        DefaultTodoService::new(self.manager_module.todo_centre())
    }
}
