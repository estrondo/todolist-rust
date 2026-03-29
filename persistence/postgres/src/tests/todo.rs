mod with_checkito {

    use std::num::NonZeroUsize;

    use checkito::{Check, FullGenerate};
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
            .check(async |todo| {
                let upserted = repository.upsert(&todo).await.unwrap();
                assert_eq!(upserted, UpsertResult::Inserted(todo))
            })
            .await;

        if let Some(fail) = result {
            panic!("{fail:?}")
        }
    }
}
