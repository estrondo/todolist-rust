use async_trait::async_trait;

use super::super::Result;
use crate::{model::geo::GeoItem, persistence::GeoPersistence};

#[async_trait]
pub trait GeoManager {
    async fn add_item(&mut self, geo_item: &GeoItem) -> Result<()>;
}

pub struct PersistentGeoManager<P: GeoPersistence> {
    persistence: P,
}

impl<P: GeoPersistence + Send> PersistentGeoManager<P> {
    pub fn new(persistence: P) -> Self {
        Self { persistence }
    }
}

#[async_trait]
impl<P> GeoManager for PersistentGeoManager<P>
where
    P: GeoPersistence + Send,
{
    async fn add_item(&mut self, item: &GeoItem) -> Result<()> {
        unimplemented!()
    }
}
