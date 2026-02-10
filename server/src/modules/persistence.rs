use todolist_core::persistence::GeoPersistence;
use todolist_persistence_postgres::{PostgresServer, geo::PostgresGeoPersistence};

use crate::config::Configuration;

pub struct PersistenceModule<'c> {
    configuration: &'c Configuration,
}

impl<'c> PersistenceModule<'c> {
    pub fn new(configuration: &'c Configuration) -> Self {
        Self { configuration }
    }

    pub fn geo_persistence(&self) -> impl GeoPersistence {
        PostgresGeoPersistence::new(PostgresServer {
            url: self.configuration.geo_persistence.url.clone(),
        })
    }
}
