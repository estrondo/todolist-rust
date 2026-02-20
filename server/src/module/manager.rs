use std::sync::Arc;

use todolist_core::manager::{PersistentTodoManager, TodoManager};

use crate::{configuration::Configuration, module::repository::RepositoryModule};
use todolist_core::Result;

pub struct ManagerModule {
    todo_manager: Arc<dyn TodoManager>,
}

impl<'c> ManagerModule {
    pub fn new(
        _configuration: &Configuration,
        repository_module: &RepositoryModule,
    ) -> Result<Self> {
        let todo_manager = Arc::new(PersistentTodoManager::new(
            repository_module.todo_repository(),
        ));
        Result::Ok(Self { todo_manager })
    }

    pub fn todo_manager(&self) -> Arc<dyn TodoManager> {
        self.todo_manager.clone()
    }
}
