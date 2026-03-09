use arbitrary::{Arbitrary, Unstructured};
use rand::{TryRng, rngs::SmallRng};
use rstest::*;
use todolist_core::persistence::TodoRepository;
use todolist_core::{model::Todo, persistence::UpsertResult};

use crate::{PostgresTodoRepository, tests::containers::PostgresContainer};

type DNA = [u8; 512];

#[fixture]
fn random() -> DNA {
    let mut u: DNA = [0u8; 512];
    let mut x: SmallRng = rand::make_rng();
    x.try_fill_bytes(&mut u).unwrap();
    u
}

#[fixture]
pub async fn postgres_container() -> PostgresContainer {
    PostgresContainer::new().await.unwrap()
}

#[fixture]
pub fn todo_item(random: DNA) -> Todo {
    let mut unstructed = Unstructured::new(&random);
    Todo::arbitrary(&mut unstructed).unwrap()
}

#[rstest]
#[tokio::test]
async fn concat_test(
    #[future(awt)] postgres_container: PostgresContainer,
    #[values(
        todo_item(random()),
        todo_item(random()),
        todo_item(random()),
        todo_item(random()),
        todo_item(random())
    )]
    todo_item: Todo,
) {
    let repository = PostgresTodoRepository::new(postgres_container.connection().clone());

    let upserted = repository.upsert(&todo_item).await.unwrap();
    println!("{upserted:?}");
    assert_eq!(upserted, UpsertResult::Inserted(todo_item))
}
