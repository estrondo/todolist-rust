use std::marker::PhantomData;
use std::num::NonZeroUsize;
use std::{fmt::Debug, future::Future};

use checkito::{Check, FullGenerate};

pub struct ContextedChecker<A>(PhantomData<A>);

impl<A> ContextedChecker<A>
where
    A: FullGenerate<Item = A> + Debug,
{
    pub fn new() -> Self {
        Self(PhantomData)
    }

    pub async fn with<'a, C, B, F>(&self, context: &'a C, block: B)
    where
        B: Fn(A, &'a C) -> F,
        F: Future<Output = ()>,
    {
        let checker = A::generator().checker().asynchronous(NonZeroUsize::new(1));
        let result = checker
            .check(|item| async { block(item, context).await })
            .await;

        if let Some(fail) = result {
            panic!("{fail:?}");
        }
    }
}
