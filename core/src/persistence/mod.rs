use async_trait::async_trait;

use crate::Result;
use crate::model::{Todo, TodoId};

pub enum UpsertResult {
    Updated,
    Inserted,
}

#[async_trait]
pub trait TodoRepository {
    async fn upsert(&self, todo: &Todo) -> Result<UpsertResult>;
    async fn delete(&self, todo_id: &TodoId) -> Result<()>;
}
