use std::collections::HashMap;

use checkito_ext::async_check;
use futures::StreamExt;
use todolist_core::{
    generator::{DefaultTimeGenerator, TimeGenerator},
    model::{permission::TodoPermission, todo::TodoId},
    repositories::TodoPermissionRepository,
};

use crate::{PostgresTodoPermissionRepository, tests::container::PostgresContainer};

async fn create_repository<T: TimeGenerator>(
    generator: T,
) -> (PostgresContainer, PostgresTodoPermissionRepository<T>) {
    let container = PostgresContainer::new()
        .await
        .expect("Unable to create the container");

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

#[tokio::test]
async fn check_select_by_todo_id() {
    let (_c, repository) = create_repository(DefaultTimeGenerator::default()).await;

    async_check(|(todo_id, samples): (TodoId, [TodoPermission; 5])| {
        let repository = repository.to_owned();
        async move {
            let mut hash_map = HashMap::new();
            for mut x in samples {
                x.todo_id = todo_id.to_owned();
                repository.upsert(&x).await.expect("Unable to upsert");
                hash_map.insert(x.user_id.0, x);
            }

            let mut stream = repository
                .search_permissions(&todo_id)
                .await
                .expect("Unable to search permissions");

            while let Some(stored) = stream.next().await {
                match stored {
                    Ok(todo_permission) => {
                        hash_map.get(&todo_permission.user_id.0).expect(&format!(
                            "Unable to find the todo for user {:?}",
                            &todo_permission.user_id
                        ));
                    }
                    Err(cause) => {
                        panic!("{cause:?}");
                    }
                }
            }
        }
    })
    .await;

    ()
}
