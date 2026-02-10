use super::Result;
use async_trait::async_trait;

#[async_trait]
pub trait GeoPersistence {
    async fn search(&self) -> Result<String>;
}
