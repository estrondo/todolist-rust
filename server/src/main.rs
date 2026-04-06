use config::Config;
use core::error::Error;
use log;
use simple_logger::SimpleLogger;
use std::net::SocketAddr;
use todolist_server::{
    api::v1::todo_service_server::TodoServiceServer,
    configuration::{Configuration, Mode},
    module::{manager::CentreModule, repository::RepositoryModule, service::ServiceModule},
};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new().init()?;

    log::info!("Preparing to initialise the wonderful TODOList Server.");

    let (configuration, mode) = Configuration::default();

    log::info!(
        "{}",
        match mode {
            Mode::Dev => "Well, it's starting in development environment, let's make it happen!",
            Mode::Stg => "Are you ready to test? I am starting for tests purposes!",
            Mode::Prd => "Okay, our mission is to help the people to have their lives organised!",
        }
    );

    let configuration: Configuration = Config::builder()
        .add_source(Config::try_from(&configuration)?)
        .add_source(config::Environment::with_prefix("TODOLIST").separator("_"))
        .build()?
        .try_deserialize()?;

    log::info!("Preparing the server...");

    let repository_module = RepositoryModule::new(&configuration)
        .await
        .inspect_err(|e| {
            log::error!("Unable to create the repository module: {}.", e.to_string())
        })?;

    let manager_module = CentreModule::new(&configuration, &repository_module)?;
    let service_module = ServiceModule::new(&configuration, &manager_module)?;

    let addr: SocketAddr = format!(
        "{}:{}",
        configuration.server.address, configuration.server.port
    )
    .parse()?;

    log::info!("Starting the server at {}.", addr);

    Server::builder()
        .add_service(TodoServiceServer::new(service_module.todo_service()))
        .serve(addr)
        .await?;

    Ok(())
}
