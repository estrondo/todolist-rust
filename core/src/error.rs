use std::{
    convert::Infallible,
    error::Error,
    fmt::{Debug, Display},
};

#[derive(Debug)]
pub struct ConvertError {
    message: String,
}

impl ConvertError {
    pub fn message(&self) -> String {
        self.message.clone()
    }
}

impl From<String> for ConvertError {
    fn from(message: String) -> Self {
        Self { message }
    }
}

impl From<&str> for ConvertError {
    fn from(value: &str) -> Self {
        Self {
            message: String::from(value),
        }
    }
}

impl From<uuid::Error> for ConvertError {
    fn from(value: uuid::Error) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}

impl From<Infallible> for ConvertError {
    fn from(_: Infallible) -> Self {
        ConvertError {
            message: String::from("Unexpected error!"),
        }
    }
}

#[derive(Debug)]
pub enum ManagerError {
    Internal {
        message: String,
    },
    UnexpectedError {
        message: String,
        cause: Box<dyn Error>,
    },
    PersistenceError {
        message: String,
        cause: PersistenceError,
    },
}

impl ManagerError {
    pub fn message(&self) -> String {
        match self {
            ManagerError::Internal { message } => message.to_owned(),
            ManagerError::UnexpectedError { message, cause: _ } => message.to_owned(),
            ManagerError::PersistenceError { message, cause: _ } => message.to_owned(),
        }
    }
}

impl From<Box<dyn Error>> for ManagerError {
    fn from(value: Box<dyn Error>) -> Self {
        ManagerError::UnexpectedError {
            message: value.to_string(),
            cause: value,
        }
    }
}

#[derive(Debug)]
pub enum PersistenceError {
    InvalidState {
        message: String,
    },
    UnexpectedError {
        message: String,
        cause: Option<Box<dyn Debug>>,
    },
    UnexpectedModelState {
        message: String,
        cause: Option<Box<dyn Debug>>,
    },
}

impl PersistenceError {
    pub fn invalid_state<T: Debug>(
        message: String,
        cause: Option<Box<dyn Debug>>,
    ) -> PersistenceError {
        PersistenceError::UnexpectedModelState { message, cause }
    }
}

impl Display for PersistenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PersistenceError::InvalidState { message } => {
                f.write_str(&format!("InvalidState: {message}"))
            }
            PersistenceError::UnexpectedError { message, cause } => {
                f.write_str(&format!("UnexpectedError: {message}"))?;
                cause.fmt(f)
            }
            PersistenceError::UnexpectedModelState { message, cause } => {
                f.write_str(&format!("UnexpectedModelState: {message}"))?;
                cause.fmt(f)
            }
        }
    }
}
