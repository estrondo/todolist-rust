use sea_orm::DbErr;
use todolist_core::error::PersistenceError;

pub(crate) fn db_err_to_persistence_error(error: DbErr) -> PersistenceError {
    PersistenceError::UnexpectedError {
        message: error.to_string(),
        cause: Some(Box::new(error)),
    }
}
