use crate::error::PersistenceError;

pub type PersistenceResult<T> = Result<T, PersistenceError>;

pub mod auth;
pub mod permission;
pub mod todo;
