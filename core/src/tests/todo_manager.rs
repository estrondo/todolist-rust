use checkito::check;
use mockall::predicate::*;

use crate::{
    manager::{DefaultTodoManager, TodoManager}, model::todo::Todo, persistence::MockTodoRepository
};

#[tokio::test]
#[check(_, verbose = false)]
async fn check_insert(todo: Todo) {
    let mut mock = MockTodoRepository::default();
    mock.expect_upsert()
        .with(eq(todo.to_owned()))
        .times(1)
        .returning(|arg| {
            let arg = arg.to_owned();
            Box::pin(async { Ok(arg) })
        });

    let manager = DefaultTodoManager::new(mock);
    let result = manager.upsert(&todo).await.unwrap();
    assert_eq!(result, todo);
}
