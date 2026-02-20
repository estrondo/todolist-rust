mod entities;
mod field;
mod todo_repository;


pub use todo_repository::PostgresTodoRepository;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field<T: ToOwned>(pub T);

impl<T: Clone> Field<T> {
    fn from_owned(value: &T) -> Field<T> {
        Field(value.to_owned())
    }
}



#[derive(Debug)]
pub struct PostgresServer {
    pub url: String,
}
