use config::Config;
use todolist_core::manager::geo::{GeoManager, PersistentGeoManager};

pub struct ManagerModule {
    geo_manager: Box<dyn GeoManager>
}

impl ManagerModule {
    pub fn new(_config: Config, dep: super::persistence::PersistenceModule) -> Self {
        Self {
            geo_manager: Box::new(dep.geo_persistence())
        }
    }
}