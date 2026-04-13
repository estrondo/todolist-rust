use checkito_ext::async_check;
use todolist_core::{
    generator::{DefaultTimeGenerator, TimeGenerator},
    model::todo::Todo,
    persistence::TodoRepository,
};

use crate::{PostgresTodoRepository, tests::containers::PostgresContainer};

async fn create_repository<T: TimeGenerator>(
    time_generator: T,
) -> (PostgresContainer, PostgresTodoRepository<T>) {
    let container = PostgresContainer::new().await.unwrap();
    let repository = PostgresTodoRepository::new(container.connection().clone(), time_generator);
    (container, repository)
}

#[tokio::test]
async fn check_upsert_insert() {
    let (_c, repo) = create_repository(DefaultTimeGenerator::default()).await;
    async_check(|todo| {
        let repo = repo.clone();
        async move {
            repo.upsert(&todo).await.unwrap();
            let result = repo
                .get(&todo.id)
                .await
                .expect("get todo failure!")
                .expect("todo not found!");
            assert_eq!(result, todo);
        }
    })
    .await;
}

#[tokio::test]
async fn check_upsert_update() {
    let (_c, repo) = create_repository(DefaultTimeGenerator::default()).await;

    async_check(|(original, mut updated): (Todo, Todo)| {
        let repo = repo.to_owned();
        async move {
            repo.upsert(&original).await.unwrap();

            updated.id = original.id.to_owned();
            repo.upsert(&updated).await.unwrap();

            let result = repo
                .get(&original.id)
                .await
                .expect("get todo failure!")
                .expect("todo not found!");

            assert_eq!(result, updated);
        }
    })
    .await;
}

#[tokio::test]
async fn check_remove() {
    let (_c, repo) = create_repository(DefaultTimeGenerator::default()).await;

    async_check(|todo| {
        let repo = repo.to_owned();
        async move {
            repo.upsert(&todo).await.unwrap();

            let stored = repo
                .get(&todo.id)
                .await
                .expect("get just inserted todo failure!")
                .expect("inserted todo not found!");

            assert_eq!(stored, todo);

            let removed = repo
                .remove(&stored.id)
                .await
                .expect("get just removed todo failure")
                .expect("removed todo not found");

            assert_eq!(removed, todo);

            let not_found = repo.get(&todo.id).await.expect("get removed todo failure");

            assert_eq!(not_found, None);
        }
    })
    .await;
}
