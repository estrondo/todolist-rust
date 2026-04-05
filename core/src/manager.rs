use async_trait::async_trait;

use crate::Result;
use crate::error::ManagerError;
use crate::model::todo::{Todo, TodoId};
use crate::persistence::TodoRepository;

pub type ManagerResult<T> = Result<T, ManagerError>;

#[async_trait]
pub trait TodoManager: Send + Sync {
    async fn upsert<'a>(&self, todo: &'a Todo) -> ManagerResult<Todo>;

    async fn remove<'a>(&self, todo_id: &'a TodoId) -> ManagerResult<Option<Todo>>;
}

pub struct DefaultTodoManager<D> {
    data: D,
}

impl<D> DefaultTodoManager<D> {
    pub fn new(data: D) -> Self {
        Self { data }
    }
}

#[async_trait]
impl<D> TodoManager for DefaultTodoManager<D>
where
    D: TodoRepository,
{
    async fn upsert<'a>(&self, todo: &'a Todo) -> ManagerResult<Todo> {
        let upserted = self
            .data
            .upsert(todo)
            .await
            .inspect_err(|e| log::error!("Unable to update the todo item: {}", e))
            .map_err(|e| ManagerError::PersistenceError {
                message: "Unable to update the Todo item.".to_owned(),
                cause: e,
            })?;

        ManagerResult::Ok(upserted)
    }

    async fn remove<'a>(&self, _todo_id: &'a TodoId) -> ManagerResult<Option<Todo>> {
        unimplemented!()
    }
}
