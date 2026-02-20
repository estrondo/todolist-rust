use std::{sync::Arc, time::Duration};

use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use todolist_core::Result;
use todolist_persistence_postgres::PostgresTodoRepository;

use crate::configuration::Configuration;

#[derive(Debug)]
pub struct RepositoryModule {
    connection: Arc<DatabaseConnection>,
}

impl RepositoryModule {
    pub async fn new(configuration: &Configuration) -> Result<Self> {
        let postgres = &configuration.postgres;
        let opt = ConnectOptions::new(format!(
            "postgres://{}:{}@{}/{}",
            postgres.username, postgres.password, postgres.address, postgres.database
        ))
        .acquire_timeout(Duration::from_secs(5))
        .to_owned();

        let connection = Database::connect(opt).await?;

        Ok(RepositoryModule {
            connection: Arc::new(connection),
        })
    }

    pub fn todo_repository(&self) -> PostgresTodoRepository {
        PostgresTodoRepository::new(self.connection.clone())
    }
}
