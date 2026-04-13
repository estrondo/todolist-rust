use async_trait::async_trait;

#[cfg(test)]
use mockall::automock;

use crate::{
    error::AuthError,
    model::user::{User, UserId},
    persistence::AuthRepository,
};

pub type AuthCentreResult<T> = Result<T, AuthError>;

#[async_trait]
#[cfg_attr(test, automock)]
pub trait AuthCentre: Send + Sync {
    async fn identify(&self, token: &UserId) -> AuthCentreResult<Option<User>>;
}

pub struct DefaultAuthCentre<D> {
    _data: D,
}

impl<D> DefaultAuthCentre<D> {
    pub fn new(data: D) -> Self {
        Self { _data: data }
    }
}

#[async_trait]
impl<D> AuthCentre for DefaultAuthCentre<D>
where
    D: AuthRepository,
{
    async fn identify(&self, _user_id: &UserId) -> AuthCentreResult<Option<User>> {
        todo!()
    }
}
