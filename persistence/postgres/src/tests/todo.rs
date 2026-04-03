mod with_checkito {

    use checkito_ext::async_check;
    use todolist_core::persistence::{TodoRepository, UpsertResult};

    use crate::{PostgresTodoRepository, tests::containers::PostgresContainer};

    async fn create_repository() -> (PostgresContainer, PostgresTodoRepository) {
        let container = PostgresContainer::new().await.unwrap();
        let repository = PostgresTodoRepository::new(container.connection().clone());
        (container, repository)
    }

    #[tokio::test]
    async fn check_todo_insert() {
        let tuple = create_repository().await;
        async_check(|todo| async {
            let inserted = tuple.1.upsert(&todo).await.unwrap();
            assert_eq!(inserted, UpsertResult::Inserted(todo));
        })
        .await;
    }
}
