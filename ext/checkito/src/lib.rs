use std::num::NonZeroUsize;
use std::{fmt::Debug, future::Future};

use checkito::{Check, FullGenerate};

pub struct ContextedChecker<A>(pub(crate) A);

impl<A> ContextedChecker<A> {
    pub fn new(a: A) -> Self {
        Self(a)
    }
}

impl<A> ContextedChecker<A> {
    pub async fn with<G, FR, C>(self, context: C)
    where
        A: Fn(G, C) -> FR,
        G: FullGenerate<Item = G> + Debug,
        FR: Future<Output = ()>,
        C: Clone,
    {
        let checker = G::generator().checker().asynchronous(NonZeroUsize::new(2));
        let result = checker
            .check(|item| async {
                self.0(item, context.clone()).await;
            })
            .await;

        if let Some(fail) = result {
            panic!("{fail:?}");
        }
    }
}
