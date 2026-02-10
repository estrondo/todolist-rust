use super::Result;
use async_trait::async_trait;

pub struct GeoPersistence {}

impl GeoPersistence {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl super::GeoPersistence for GeoPersistence {
    async fn search(&self) -> Result<String> {
        todo!()
    }
}
