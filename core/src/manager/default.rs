use async_trait::async_trait;
use log;

use crate::{
    error::ManagerError,
    manager::{ManagerResult, TodoManager},
    model::{Todo, TodoId},
    persistence::TodoRepository,
};

pub struct PersistentTodoManager<P: TodoRepository> {
    repository: P,
}

impl<R> PersistentTodoManager<R>
where
    R: TodoRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<P> TodoManager for PersistentTodoManager<P>
where
    P: TodoRepository + Send + Sync,
{
    async fn upsert<'a>(&self, todo: &'a Todo) -> ManagerResult<Todo> {
        let result = self
            .repository
            .upsert(todo)
            .await
            .inspect_err(|e| log::error!("Unable to upsert a Todo item: {}", e.to_string()))
            .map_err(|e| ManagerError::CausedByPersistence {
                message: String::from("Unable to upsert Todo item."),
                cause: e,
            })?;

        ManagerResult::Ok(result)
    }

    async fn remove<'a>(&self, _todo_id: &'a TodoId) -> ManagerResult<Option<Todo>> {
        unimplemented!()
    }
}
