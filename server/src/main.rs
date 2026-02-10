use config::Config;
use core::error::Error;
use log;
use simple_logger::SimpleLogger;
use std::net::SocketAddr;
use todolist_core::manager::geo::PersistentGeoManager;
use todolist_persistence_postgres::geo::PostgresGeoPersistence;
use todolist_server::{
    api::v1::geo_service_server::GeoServiceServer,
    config::{Configuration, Mode},
    modules::persistence::PersistenceModule,
    services::geo::GeoServiceImpl,
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
            Mode::Dev() => "Well, it's starting in development environment, let's make it happen!",
            Mode::Stg() => "Are you ready to test? I am starting for tests purposes!",
            Mode::Prd() => "Okay, our mission is to help the people to have their lives organised!",
        }
    );

    let configuration: Configuration = Config::builder()
        .add_source(Config::try_from(&configuration)?)
        .add_source(config::Environment::with_prefix("TODOLIST").separator("_"))
        .build()?
        .try_deserialize()?;

    log::debug!("Preparing all services.");

    let persistence_module = PersistenceModule::new(&configuration);
    let geo_manager = PersistentGeoManager::new(geo_persistence);
    let geo_service = GeoServiceImpl::new(geo_manager);

    let addr: SocketAddr = format!(
        "{}:{}",
        configuration.server.address, configuration.server.port
    )
    .parse()?;

    log::info!("Starting the server at {}", addr);

    Server::builder()
        .add_service(GeoServiceServer::new(geo_service))
        .serve(addr)
        .await?;

    Ok(())
}
