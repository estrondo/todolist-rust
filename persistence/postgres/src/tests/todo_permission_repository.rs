use checkito_ext::async_check;
use todolist_core::{
    generator::{DefaultTimeGenerator, TimeGenerator},
    model::permission::TodoPermission,
    persistence::TodoPermissionRepository,
};

use crate::{PostgresTodoPermissionRepository, tests::containers::PostgresContainer};

async fn create_repository<T: TimeGenerator>(
    generator: T,
) -> (PostgresContainer, PostgresTodoPermissionRepository<T>) {
    let container = PostgresContainer::new().await.unwrap();
    let permission_repository =
        PostgresTodoPermissionRepository::new(container.connection().clone(), generator.to_owned());

    (container, permission_repository)
}

#[tokio::test]
async fn check_upsert_insert() {
    let (_c, repo) = create_repository(DefaultTimeGenerator::default()).await;
    async_check(|todo_permission| {
        let repo = repo.to_owned();

        async move {
            repo.upsert(&todo_permission)
                .await
                .expect("Unable to upsert the todo permission");
            let result = repo
                .get(&todo_permission.todo_id, &todo_permission.user_id)
                .await
                .expect("get todo permission failure")
                .expect("todo permission not found");
            assert_eq!(result, todo_permission);
        }
    })
    .await;
}

#[tokio::test]
async fn check_upsert_update() {
    let (_c, repo) = create_repository(DefaultTimeGenerator::default()).await;
    async_check(
        |(original, mut to_updated): (TodoPermission, TodoPermission)| {
            let repo = repo.to_owned();

            async move {
                let inserted = repo
                    .upsert(&original)
                    .await
                    .expect("Unable to upsert the todo permission");

                assert_eq!(inserted, original);

                let stored = repo
                    .get(&original.todo_id, &original.user_id)
                    .await
                    .expect("get todo permission failure")
                    .expect("todo permission not found");

                assert_eq!(stored, original);

                to_updated.todo_id = original.todo_id.to_owned();
                to_updated.user_id = original.user_id.to_owned();

                let updated = repo.upsert(&to_updated).await.expect("Update failed");
                assert_eq!(updated, to_updated);

                let stored = repo
                    .get(&original.todo_id, &original.user_id)
                    .await
                    .expect("failed to load the stored")
                    .expect("todo permission not found");

                assert_eq!(stored, to_updated);
            }
        },
    )
    .await;
}

#[tokio::test]
async fn check_remove() {
    let (_c, repo) = create_repository(DefaultTimeGenerator::default()).await;
    async_check(|todo_permission| {
        let repo = repo.to_owned();

        async move {
            repo.upsert(&todo_permission)
                .await
                .expect("Unable to upsert the todo permission");

            let result = repo
                .get(&todo_permission.todo_id, &todo_permission.user_id)
                .await
                .expect("get todo permission failure")
                .expect("todo permission not found");

            assert_eq!(result, todo_permission);

            let result = repo
                .remove(&todo_permission.todo_id, &todo_permission.user_id)
                .await
                .expect("Unable to remove the todo permission")
                .expect("It was expected to return the removed todo permission");

            assert_eq!(result, todo_permission);

            let result = repo
                .remove(&todo_permission.todo_id, &todo_permission.user_id)
                .await
                .expect("Unexpected failure");

            assert_eq!(result, None);
        }
    })
    .await;
}
