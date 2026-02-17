use async_trait::async_trait;
use todolist_core::Result;
use todolist_core::{
    model::{Todo, TodoId},
    persistence::{TodoRepository, UpsertResult},
};

pub struct PostgresTodoRepository {}

#[async_trait]
impl TodoRepository for PostgresTodoRepository {
    async fn upsert(&self, todo: &Todo) -> Result<UpsertResult> {
        unimplemented!()
    }
    async fn delete(&self, todo_id: &TodoId) -> Result<()> {
        unimplemented!()
    }
}
