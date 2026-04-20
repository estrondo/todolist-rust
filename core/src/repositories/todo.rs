use super::PersistenceResult;
use crate::model::todo::{Todo, TodoId};
use async_trait::async_trait;

#[cfg(test)]
use mockall::{automock, predicate::eq};

#[cfg_attr(test, automock)]
#[async_trait]
pub trait TodoRepository: Send + Sync {
    async fn get(&self, id: &TodoId) -> PersistenceResult<Option<Todo>>;
    async fn upsert(&self, todo: &Todo) -> PersistenceResult<Todo>;
    async fn remove(&self, todo_id: &TodoId) -> PersistenceResult<Option<Todo>>;
}

#[cfg(test)]
impl MockTodoRepository {
    pub fn once_success_upsert(&mut self, todo: &Todo) {
        let todo = todo.to_owned();
        self.expect_upsert()
            .once()
            .with(eq(todo.to_owned()))
            .returning(|a| PersistenceResult::Ok(a.to_owned()));
    }
}
