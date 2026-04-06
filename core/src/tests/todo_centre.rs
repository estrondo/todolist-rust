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
        user::{User, UserId},
    },
    persistence::{MockTodoRepository, PersistenceResult},
    tests::assert_with_debug,
};

#[tokio::test]
#[check(_, _, verbose = false)]
async fn check_upsert_success(todo: Todo, user: User) {
    let mut mock_permission = MockPermissionCentre::new();
    let mut mock_repository = MockTodoRepository::new();

    mock_repository
        .expect_upsert()
        .with(eq(todo.to_owned()))
        .times(1)
        .returning(|todo| {
            let cloned = todo.to_owned();
            Box::pin(async { PersistenceResult::Ok(cloned) })
        });

    let expected_permission = TodoPermission::new_owner(todo.id.to_owned(), user.id.to_owned());
    mock_permission
        .expect_insert_todo_permission()
        .times(1)
        .with(eq(expected_permission))
        .returning(|todo_permission| {
            let cloned = todo_permission.to_owned();
            Box::pin(async { CentreResult::Ok(cloned) })
        });

    let result = DefaultTodoCentre::new(mock_repository, Arc::new(mock_permission))
        .upsert(&todo, &user.id)
        .await
        .unwrap();

    assert_eq!(result, todo)
}

#[tokio::test]
#[check(_, _, verbose = false)]
async fn check_upsert_permission_failure(todo: Todo, user: User) {
    let mut mock_permission = MockPermissionCentre::new();
    let mut mock_repository = MockTodoRepository::new();

    mock_repository
        .expect_upsert()
        .with(eq(todo.to_owned()))
        .times(1)
        .returning(|todo| {
            let cloned = todo.to_owned();
            Box::pin(async { PersistenceResult::Ok(cloned) })
        });

    let owned = todo.to_owned();
    mock_repository
        .expect_remove()
        .with(eq(todo.id.to_owned()))
        .times(1)
        .returning(move |_| {
            let owned = owned.to_owned();
            Box::pin(async { PersistenceResult::Ok(Some(owned)) })
        });

    let expected_permission = TodoPermission::new_owner(todo.id.to_owned(), user.id.to_owned());
    mock_permission
        .expect_insert_todo_permission()
        .times(1)
        .with(eq(expected_permission))
        .returning(|_| {
            Box::pin(async {
                CentreResult::Err(crate::centre::CentreError::Unexpected(
                    "An error!".into(),
                    None,
                ))
            })
        });

    let result: CentreError = DefaultTodoCentre::new(mock_repository, Arc::new(mock_permission))
        .upsert(&todo, &user.id)
        .await
        .unwrap_err();

    assert_with_debug(
        result,
        CentreError::Unexpected(
            "Unable to insert todo permission".into(),
            Some(Box::new(CentreError::Unexpected("An error!".into(), None))),
        ),
    )
}

#[tokio::test]
#[check(_, _, verbose = false)]
async fn check_upsert_todo_with_todo_repo_failure(todo: Todo, user: User) {
    let mut mock_permission = MockPermissionCentre::new();
    let mut mock_repository = MockTodoRepository::new();

    mock_repository
        .expect_upsert()
        .with(eq(todo.to_owned()))
        .times(1)
        .returning(|_| {
            Box::pin(async {
                PersistenceResult::Err(PersistenceError::UnexpectedError(
                    "Unable to upsert the todo item!".into(),
                    None,
                ))
            })
        });

    mock_permission.expect_insert_todo_permission().never();

    let result = DefaultTodoCentre::new(mock_repository, Arc::new(mock_permission))
        .upsert(&todo, &user.id)
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
async fn check_remove_todo_with_role(todo: Todo, user_id: UserId, role: TodoPermissionRole) {
    let mut mock_todo = MockTodoRepository::new();
    let mut mock_permission = MockPermissionCentre::new();

    let expected_found_permission =
        TodoPermission::new(todo.id.to_owned(), user_id.to_owned(), role.to_owned());

    let expected_remove_permission = expected_found_permission.to_owned();

    mock_permission
        .expect_get_todo_permission()
        .with(eq(todo.id.to_owned()), eq(user_id.to_owned()))
        .times(1)
        .returning(move |_a, _b| {
            let cloned = expected_found_permission.to_owned();
            Box::pin(async { CentreResult::Ok(Some(cloned)) })
        });

    match &role {
        TodoPermissionRole::Owner => {
            let expected_todo = todo.to_owned();
            mock_todo
                .expect_remove()
                .with(eq(expected_todo.id.to_owned()))
                .once()
                .returning(move |_| {
                    let cloned = expected_todo.to_owned();
                    Box::pin(async { PersistenceResult::Ok(Some(cloned)) })
                });

            let mock_permission_returns = expected_remove_permission.to_owned();

            mock_permission
                .expect_remove_todo_permission()
                .with(eq(expected_remove_permission.to_owned()))
                .once()
                .returning(move |_| {
                    let returns = mock_permission_returns.to_owned();
                    Box::pin(async { CentreResult::Ok(Some(returns)) })
                });

            let result = DefaultTodoCentre::new(mock_todo, Arc::new(mock_permission))
                .remove(&todo.id, &user_id)
                .await
                .unwrap()
                .unwrap();

            assert_eq!(result, todo);
        }
        _ => {
            mock_todo.expect_remove().never();

            let result = DefaultTodoCentre::new(mock_todo, Arc::new(mock_permission))
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
