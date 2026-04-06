use std::{
    error::Error,
    fmt::{Debug, Display},
};

use crate::error::PersistenceError;

pub mod auth;
pub mod permission;
pub mod todo;

pub type CentreResult<T> = Result<T, CentreError>;

#[derive(Debug)]
pub enum CentreError {
    Unexpected(String, Option<Box<dyn Error + Send + Sync>>),
    Unauthorized(String, Option<Box<dyn Error + Send + Sync>>),
}

impl From<PersistenceError> for CentreError {
    fn from(value: PersistenceError) -> Self {
        CentreError::Unexpected(
            "Unexpected persistence error.".to_owned(),
            Some(Box::new(value)),
        )
    }
}

impl Display for CentreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CentreError::Unexpected(mgs, error) => {
                f.write_str(mgs)?;
                f.write_str(" -> ")?;
                error.fmt(f)
            }
            CentreError::Unauthorized(msg, error) => {
                f.write_str(msg)?;
                f.write_str(" -> ")?;
                error.fmt(f)
            }
        }
    }
}

impl Error for CentreError {}
