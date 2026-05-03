use std::{
    convert::Infallible,
    error::Error,
    fmt::{Debug, Display},
};

#[derive(Debug)]
pub struct ConvertError(pub String, pub Option<Box<dyn Error>>);

impl From<String> for ConvertError {
    fn from(message: String) -> Self {
        Self(message, None)
    }
}

impl From<&str> for ConvertError {
    fn from(value: &str) -> Self {
        Self(value.into(), None)
    }
}

impl Display for ConvertError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)?;
        match &self.1 {
            Some(cause) => {
                f.write_str(" -> ")?;
                Display::fmt(cause, f)
            }
            None => Ok(()),
        }
    }
}

impl Error for ConvertError {}

impl From<uuid::Error> for ConvertError {
    fn from(value: uuid::Error) -> Self {
        Self("UUID Error".into(), Some(Box::new(value)))
    }
}

impl From<Infallible> for ConvertError {
    fn from(_: Infallible) -> Self {
        ConvertError(String::from("Unexpected error!"), None)
    }
}

#[derive(Debug)]
pub enum PersistenceError {
    InvalidState(String),
    UnexpectedError(String, Option<Box<dyn Debug + Sync + Send>>),
    UnexpectedModelState(String, Option<Box<dyn Debug + Sync + Send>>),
}

impl PersistenceError {
    pub fn invalid_state<T: Debug>(
        message: String,
        cause: Option<Box<dyn Debug + Sync + Send>>,
    ) -> PersistenceError {
        PersistenceError::UnexpectedModelState(message, cause)
    }
}

impl Display for PersistenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PersistenceError::InvalidState(message) => {
                f.write_str(&format!("InvalidState: {message}"))
            }
            PersistenceError::UnexpectedError(message, cause) => {
                f.write_str(&format!("UnexpectedError: {message}"))?;
                cause.fmt(f)
            }
            PersistenceError::UnexpectedModelState(message, cause) => {
                f.write_str(&format!("UnexpectedModelState: {message}"))?;
                cause.fmt(f)
            }
        }
    }
}

impl Error for PersistenceError {}

#[derive(Debug)]
pub enum AuthError {}

impl Display for AuthError {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
