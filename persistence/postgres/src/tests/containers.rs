use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use testcontainers_modules::postgres::Postgres;
use testcontainers_modules::testcontainers::core::error::Result;
use testcontainers_modules::testcontainers::runners::AsyncRunner;
use testcontainers_modules::testcontainers::{ContainerAsync, ImageExt, TestcontainersError};
#[derive(Debug)]
pub struct PostgresContainer {
    _container: ContainerAsync<Postgres>,
    connection: DatabaseConnection,
}

impl PostgresContainer {
    pub async fn new() -> Result<Self> {
        let container = Postgres::default()
            .with_password("todolist")
            .with_db_name("todolist")
            .with_user("todolist")
            .with_name("docker.io/postgis/postgis")
            .with_tag("17-3.6-alpine")
            .start()
            .await?;

        let connection = Database::connect(format!(
            "postgres://todolist:todolist@localhost:{}/todolist",
            container.get_host_port_ipv4(5432).await?
        ))
        .await
        .map_err(|e| TestcontainersError::Other(Box::new(e)))?;

        Migrator::up(&connection, None)
            .await
            .map_err(|e| TestcontainersError::Other(Box::new(e)))?;

        Ok(Self {
            _container: container,
            connection,
        })
    }

    pub fn connection(&self) -> &DatabaseConnection {
        &self.connection
    }
}
