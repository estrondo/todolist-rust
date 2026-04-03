use std::num::NonZeroUsize;
use std::{fmt::Debug, future::Future};

use checkito::{Check, FullGenerate};

pub async fn async_check<B, BR, G>(block: B)
where
    B: Fn(G) -> BR,
    BR: Future<Output = ()>,
    G: FullGenerate<Item = G> + Debug,
{
    let checker = G::generator().checker().asynchronous(NonZeroUsize::new(1));
    let result = checker
        .check(|item| async {
            block(item).await;
        })
        .await;

    if let Some(fail) = result {
        panic!("{fail:?}");
    }
}
