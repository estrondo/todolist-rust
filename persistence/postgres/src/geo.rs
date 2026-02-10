use async_trait::async_trait;
use todolist_core::Result;
use todolist_core::persistence::GeoPersistence;

use crate::PostgresServer;

pub struct PostgresGeoPersistence {}

impl PostgresGeoPersistence {
    pub fn new(_server: PostgresServer) -> Self {
        Self {}
    }
}

#[async_trait]
impl GeoPersistence for PostgresGeoPersistence {
    async fn search(&self) -> Result<String> {
        unimplemented!()
    }
}
