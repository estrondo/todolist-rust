use checkito::check;
use mockall::predicate::eq;

use crate::{
    centre::permission::{DefaultPermissionCentre, PermissionCentre},
    model::permission::TodoPermission,
    persistence::MockTodoPermissionRepository,
};

#[tokio::test]
#[check(_, verbose = false)]
async fn check_upsert_inserting_success(permission: TodoPermission) {
    let mut todo_permission_repository = MockTodoPermissionRepository::default();

    todo_permission_repository
        .expect_upsert()
        .once()
        .with(eq(permission.to_owned()))
        .returning(|arg| {
            let owned = arg.to_owned();
            Ok(owned)
        });

    let result = DefaultPermissionCentre::new(todo_permission_repository)
        .upsert(&permission)
        .await
        .expect("Failed to insert todo permission");

    assert_eq!(result, permission)
}

#[tokio::test]
#[check(_, verbose = false)]
async fn check_get(permission: TodoPermission) {
    let mut todo_permission_repository = MockTodoPermissionRepository::default();

    let expected_return = permission.to_owned();
    todo_permission_repository
        .expect_get()
        .with(
            eq(permission.todo_id.to_owned()),
            eq(permission.user_id.to_owned()),
        )
        .once()
        .returning(move |_a, _b| {
            let owned = expected_return.to_owned();
            Ok(Some(owned))
        });

    let result = DefaultPermissionCentre::new(todo_permission_repository)
        .get(&permission.todo_id, &permission.user_id)
        .await
        .expect("Unable to get the todo permission")
        .expect("unexpected result");

    assert_eq!(result, permission)
}

#[tokio::test]
#[check(_, verbose = false)]
async fn check_remove(permission: TodoPermission) {
    let mut repo_mock = MockTodoPermissionRepository::default();

    let expected_return = permission.to_owned();
    repo_mock
        .expect_remove()
        .with(
            eq(permission.todo_id.to_owned()),
            eq(permission.user_id.to_owned()),
        )
        .once()
        .returning(move |_a, _b| {
            let owned = expected_return.to_owned();
            Ok(Some(owned))
        });

    let result = DefaultPermissionCentre::new(repo_mock)
        .remove(&permission)
        .await
        .expect("Unable to remove the todo permission")
        .expect("unexpected result");

    assert_eq!(result, permission)
}
