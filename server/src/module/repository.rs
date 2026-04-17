use std::time::Duration;

use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use todolist_core::generator::DefaultTimeGenerator;
use todolist_persistence_postgres::{PostgresTodoPermissionRepository, PostgresTodoRepository};

use crate::configuration::Configuration;

#[derive(Debug)]
pub struct RepositoryModule {
    connection: DatabaseConnection,
}

async fn start_migration(con: &DatabaseConnection) -> Result<(), DbErr> {
    Migrator::up(con, None).await
}

impl RepositoryModule {
    pub async fn new(configuration: &Configuration) -> Result<Self, DbErr> {
        let postgres = &configuration.postgres;
        let opt = ConnectOptions::new(format!(
            "postgres://{}:{}@{}/{}",
            postgres.username, postgres.password, postgres.address, postgres.database
        ))
        .acquire_timeout(Duration::from_secs(5))
        .to_owned();

        let connection = Database::connect(opt).await?;

        log::info!("Starting migration.");
        start_migration(&connection)
            .await
            .inspect_err(|_| log::error!("Migration failed."))?;

        Ok(RepositoryModule {
            connection: connection,
        })
    }

    pub fn todo_repository(&self) -> PostgresTodoRepository<DefaultTimeGenerator> {
        PostgresTodoRepository::new(self.connection.clone(), DefaultTimeGenerator::default())
    }

    pub fn todo_permission_repository(
        &self,
    ) -> PostgresTodoPermissionRepository<DefaultTimeGenerator> {
        PostgresTodoPermissionRepository::new(
            self.connection.clone(),
            DefaultTimeGenerator::default(),
        )
    }
}
