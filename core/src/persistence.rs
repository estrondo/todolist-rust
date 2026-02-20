use async_trait::async_trait;

use crate::Result;
use crate::error::PersistenceError;
use crate::model::{Todo, TodoId};

pub enum UpsertResult<T> {
    Updated(T),
    Inserted(T),
}

impl<T> UpsertResult<T> {
    pub fn into_value(self) -> T {
        match self {
            UpsertResult::Updated(value) => value,
            UpsertResult::Inserted(value) => value,
        }
    }

    pub fn inspect_insert<F>(self, f: F) -> Self
    where
        F: FnOnce(&T),
    {
        if let UpsertResult::Inserted(e) = &self {
            f(e)
        }

        self
    }

    pub fn inspect_update<F>(self, f: F) -> Self
    where
        F: FnOnce(&T),
    {
        if let UpsertResult::Updated(e) = &self {
            f(e)
        }

        self
    }
}

#[async_trait]
pub trait TodoRepository {
    async fn upsert(&self, todo: &Todo) -> Result<UpsertResult<Todo>, PersistenceError>;
    async fn delete(&self, todo_id: &TodoId) -> Result<Option<Todo>, PersistenceError>;
}
