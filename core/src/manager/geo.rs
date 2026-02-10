use async_trait::async_trait;

use crate::persistence::GeoPersistence;

#[async_trait]
pub trait GeoManager {}

pub struct PersistentGeoManager<P: GeoPersistence> {
    persistence: P,
}

impl<P: GeoPersistence> PersistentGeoManager<P> {
    pub fn new(persistence: P) -> Self {
        Self { persistence }
    }
}

impl<P> GeoManager for PersistentGeoManager<P> where P: GeoPersistence {}
