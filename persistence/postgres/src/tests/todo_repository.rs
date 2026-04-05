use checkito_ext::async_check;
use todolist_core::{model::todo::Todo, persistence::TodoRepository};

use crate::{PostgresTodoRepository, tests::containers::PostgresContainer};

async fn create_repository() -> (PostgresContainer, PostgresTodoRepository) {
    let container = PostgresContainer::new().await.unwrap();
    let repository = PostgresTodoRepository::new(container.connection().clone());
    (container, repository)
}

#[tokio::test]
async fn check_upsert_insert() {
    let (_c, repo) = create_repository().await;
    async_check(|todo| {
        let repo = repo.clone();
        async move {
            repo.upsert(&todo).await.unwrap();
            let result = repo
                .get(&todo.id)
                .await
                .expect("Failed get Todo")
                .expect("Todo not found!");
            assert_eq!(result, todo);
        }
    })
    .await;
}

#[tokio::test]
async fn check_upsert_update() {
    let (_c, repo) = create_repository().await;

    async_check(|(original, mut updated): (Todo, Todo)| {
        let repo = repo.to_owned();
        async move {
            repo.upsert(&original).await.unwrap();

            updated.id = original.id.to_owned();
            repo.upsert(&updated).await.unwrap();

            let result = repo
                .get(&original.id)
                .await
                .expect("Failed get Todo")
                .expect("Todo not found!");

            assert_eq!(result, updated);
        }
    })
    .await;
}
