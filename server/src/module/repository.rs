use todolist_persistence_postgres::todo::PostgresTodoRepository;

use crate::configuration::Configuration;

#[derive(Debug)]
pub struct RepositoryModule<'c> {
    configuration: &'c Configuration,
}

impl<'c> RepositoryModule<'c> {
    pub fn new(configuration: &'c Configuration) -> Self {
        Self { configuration }
    }

    pub fn todo_repository(&self) -> PostgresTodoRepository {
        unimplemented!()
    }
}
