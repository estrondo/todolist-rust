use std::sync::Arc;

use checkito::check;
use mockall::predicate::*;

use crate::{
    centre::{
        CentreError, CentreResult,
        permission::MockPermissionCentre,
        todo::{DefaultTodoCentre, TodoCentre},
    },
    error::PersistenceError,
    model::{
        permission::{TodoPermission, TodoPermissionRole},
        todo::Todo,
        user::UserId,
    },
    repositories::{MockTodoRepository, PersistenceResult},
    tests::assert_with_debug,
};

#[tokio::test]
#[check(_, _, verbose = false)]
async fn check_upsert_owner_success(todo: Todo, user_id: UserId) {
    let mut permission_centre = MockPermissionCentre::new();
    let mut todo_repository = MockTodoRepository::new();

    todo_repository.once_success_upsert(&todo);
    permission_centre.once_success_get(&todo, &user_id, TodoPermissionRole::Owner);

    let result = DefaultTodoCentre::new(todo_repository, Arc::new(permission_centre))
        .upsert(&todo, &user_id)
        .await
        .unwrap();

    assert_eq!(result, todo)
}

#[tokio::test]
#[check(_, _, verbose = false)]
async fn check_upsert_with_get_permission_failure(todo: Todo, user_id: UserId) {
    let mut permission_centre = MockPermissionCentre::new();
    let mut todo_repository = MockTodoRepository::new();

    permission_centre
        .expect_get()
        .once()
        .with(eq(todo.id.to_owned()), eq(user_id.to_owned()))
        .returning(|_a, _b| Err(CentreError::Unexpected("You shall not pass!".into(), None)));

    todo_repository
        .expect_remove()
        .with(eq(todo.id.to_owned()))
        .never();

    let result: CentreError = DefaultTodoCentre::new(todo_repository, Arc::new(permission_centre))
        .upsert(&todo, &user_id)
        .await
        .unwrap_err();

    assert_with_debug(
        result,
        CentreError::Unexpected(
            "Unable to get permission".into(),
            Some(Box::new(CentreError::Unexpected(
                "You shall not pass!".into(),
                None,
            ))),
        ),
    )
}

#[tokio::test]
#[check(_, _, verbose = false)]
async fn check_upsert_with_repo_failure(todo: Todo, user_id: UserId) {
    let mut permission_centre = MockPermissionCentre::new();
    let mut todo_repository = MockTodoRepository::new();

    permission_centre.once_success_get(&todo, &user_id, TodoPermissionRole::Owner);

    todo_repository
        .expect_upsert()
        .with(eq(todo.to_owned()))
        .once()
        .returning(|_| {
            PersistenceResult::Err(PersistenceError::UnexpectedError(
                "Unable to upsert the todo item!".into(),
                None,
            ))
        });

    permission_centre.expect_upsert().never();

    let result = DefaultTodoCentre::new(todo_repository, Arc::new(permission_centre))
        .upsert(&todo, &user_id)
        .await
        .unwrap_err();

    assert_with_debug(
        result,
        CentreError::Unexpected(
            "Unexpected persistence error.".into(),
            Some(Box::new(PersistenceError::UnexpectedError(
                "Unable to upsert the todo item!".into(),
                None,
            ))),
        ),
    )
}

#[tokio::test]
#[check(_, _, _, verbose = false)]
async fn check_remove(todo: Todo, user_id: UserId, role: TodoPermissionRole) {
    let mut todo_repository = MockTodoRepository::new();
    let mut permission_centre = MockPermissionCentre::new();

    let expected_found_permission =
        TodoPermission::new(todo.id.to_owned(), user_id.to_owned(), role.to_owned());

    let expected_remove_permission = expected_found_permission.to_owned();

    permission_centre
        .expect_get()
        .with(eq(todo.id.to_owned()), eq(user_id.to_owned()))
        .once()
        .returning(move |_a, _b| {
            let cloned = expected_found_permission.to_owned();
            CentreResult::Ok(Some(cloned))
        });

    match &role {
        TodoPermissionRole::Owner => {
            let expected_todo = todo.to_owned();
            todo_repository
                .expect_remove()
                .with(eq(expected_todo.id.to_owned()))
                .once()
                .returning(move |_| {
                    let cloned = expected_todo.to_owned();
                    PersistenceResult::Ok(Some(cloned))
                });

            let mock_permission_returns = expected_remove_permission.to_owned();

            permission_centre
                .expect_remove()
                .with(eq(expected_remove_permission.to_owned()))
                .once()
                .returning(move |_| {
                    let returns = mock_permission_returns.to_owned();
                    CentreResult::Ok(Some(returns))
                });

            let result = DefaultTodoCentre::new(todo_repository, Arc::new(permission_centre))
                .remove(&todo.id, &user_id)
                .await
                .unwrap()
                .unwrap();

            assert_eq!(result, todo);
        }
        _ => {
            todo_repository.expect_remove().never();

            let result = DefaultTodoCentre::new(todo_repository, Arc::new(permission_centre))
                .remove(&todo.id, &user_id)
                .await
                .unwrap_err();

            assert_with_debug(
                result,
                CentreError::Unauthorized("You are not the owner!".into(), None),
            );
        }
    }
}
