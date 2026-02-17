use async_trait::async_trait;

use crate::Result;
use crate::model::{Todo, TodoId};

pub mod default;

#[async_trait]
pub trait TodoManager {
    async fn upsert(&self, todo: &Todo) -> Result<()>;

    async fn remove(&self, todo_id: &TodoId) -> Result<()>;
}
