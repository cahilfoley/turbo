#![feature(arbitrary_self_types)]

use turbo_tasks::Vc;
use turbo_tasks_testing::{register, run};

register!();

#[turbo_tasks::value]
struct Wrapper(u32);

#[turbo_tasks::value(transparent)]
struct TransparentWrapper(u32);

#[tokio::test]
async fn store_and_read() {
    run! {
        let a: Vc<u32> = Vc::local_cell(42);
        assert_eq!(*a.await.unwrap(), 42);

        let b = Wrapper(42).local_cell();
        assert_eq!((*b.await.unwrap()).0, 42);

        let c = TransparentWrapper(42).local_cell();
        assert_eq!(*c.await.unwrap(), 42);
    }
}
