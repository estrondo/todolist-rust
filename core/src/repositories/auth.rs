use crate::{
    model::user::{User, UserId},
    repositories::PersistenceResult,
};
use async_trait::async_trait;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn search(&self, id: &UserId) -> PersistenceResult<Option<User>>;
}
