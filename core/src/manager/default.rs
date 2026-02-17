use async_trait::async_trait;

use crate::{
    Result,
    manager::TodoManager,
    model::{Todo, TodoId},
    persistence::TodoRepository,
};

pub struct PersistentTodoManager<P: TodoRepository> {
    repository: P,
}

impl<P> PersistentTodoManager<P>
where
    P: TodoRepository,
{
    pub fn new(repository: P) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<P> TodoManager for PersistentTodoManager<P>
where
    P: TodoRepository + Send + Sync,
{
    async fn upsert(&self, todo: &Todo) -> Result<()> {
        unimplemented!()
    }

    async fn remove(&self, todo_id: &TodoId) -> Result<()> {
        unimplemented!()
    }
}
