use sea_orm::DbErr;
use todolist_core::error::PersistenceError;

pub(crate) fn db_err_to_persistence_error(error: DbErr) -> PersistenceError {
    PersistenceError::UnexpectedError(error.to_string(), Some(Box::new(error)))
}
