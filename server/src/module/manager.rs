use todolist_core::manager::{TodoManager, default::PersistentTodoManager};

use crate::{configuration::Configuration, module::repository::RepositoryModule};

pub struct ManagerModule<'c> {
    configuration: &'c Configuration,
    repository_module: &'c RepositoryModule<'c>,
}

impl<'c> ManagerModule<'c> {
    pub fn new(
        configuration: &'c Configuration,
        repository_module: &'c RepositoryModule<'c>,
    ) -> Self {
        Self {
            configuration,
            repository_module,
        }
    }

    pub fn todo_manager(&self) -> impl TodoManager {
        PersistentTodoManager::new(self.repository_module.todo_repository())
    }
}
