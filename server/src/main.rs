mod start_tracing;

use crate::start_tracing::start_tracing;
use config::Config;
use core::error::Error;
use log;
use std::net::SocketAddr;
use todolist_server::{
    api::v1::todo_service_server::TodoServiceServer,
    configuration::Configuration,
    module::{
        manager::CentreModule, repository::RepositoryModule, security::SecurityModule,
        service::ServiceModule,
    },
};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (configuration, mode) = Configuration::default();
    start_tracing(&configuration, &mode).await;

    log::info!("Starting Estrondo's TODOList Server");
    log::info!("Initialising in {:?} mode.", mode);

    let configuration: Configuration = Config::builder()
        .add_source(Config::try_from(&configuration)?)
        .add_source(config::Environment::with_prefix("TODOLIST").separator("_"))
        .build()?
        .try_deserialize()?;

    let repository_module = RepositoryModule::new(&configuration)
        .await
        .inspect_err(|e| {
            log::error!("Unable to create the repository module: {}.", e.to_string())
        })?;

    let security_module = SecurityModule::new(&configuration).await?;
    let centre_module = CentreModule::new(&configuration, &repository_module)?;
    let service_module = ServiceModule::new(&configuration, &centre_module)?;

    let addr: SocketAddr = format!(
        "{}:{}",
        configuration.server.address, configuration.server.port
    )
    .parse()?;

    log::info!("Starting the server at {}.", addr);
    let security_interceptor = security_module.create_auth_info_interceptor();

    Server::builder()
        .add_service(TodoServiceServer::with_interceptor(
            service_module.todo_service(),
            security_interceptor.to_owned(),
        ))
        .serve(addr)
        .await?;

    Ok(())
}
