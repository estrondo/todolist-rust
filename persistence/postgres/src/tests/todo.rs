mod with_checkito {

    use std::num::NonZeroUsize;

    use checkito::{Check, FullGenerate};
    use checkito_ext::ContextedChecker;
    use todolist_core::{
        model::Todo,
        persistence::{TodoRepository, UpsertResult},
    };

    use crate::{PostgresTodoRepository, tests::containers::PostgresContainer};

    #[tokio::test]
    async fn simple_todo_test() {
        let container = PostgresContainer::new().await.unwrap();
        let repository = PostgresTodoRepository::new(container.connection().clone());

        let checker = Todo::generator()
            .checker()
            .asynchronous(NonZeroUsize::new(2));

        let result = checker
            .check(|todo| async {
                println!("{todo:?}");
                let upserted = repository.upsert(&todo).await.unwrap();
                assert_eq!(upserted, UpsertResult::Inserted(todo))
            })
            .await;

        if let Some(fail) = result {
            panic!("{fail:?}")
        }
    }

    async fn create_repository() -> (PostgresContainer, PostgresTodoRepository) {
        let container = PostgresContainer::new().await.unwrap();
        let repository = PostgresTodoRepository::new(container.connection().clone());
        (container, repository)
    }

    #[tokio::test]
    async fn same_test() {
        let (_c, repository) = create_repository().await;

        ContextedChecker::new(|todo, repository: PostgresTodoRepository| async move {
            println!("{todo:?}");
            let inserted = repository.upsert(&todo).await.unwrap();
            assert_eq!(inserted, UpsertResult::Inserted(todo))
        })
        .with(repository)
        .await
    }
}
