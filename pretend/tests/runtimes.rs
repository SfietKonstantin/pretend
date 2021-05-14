use std::future::Future;
use tokio::runtime::Runtime;

#[allow(unused)]
pub fn create_runtime() -> Runtime {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
}

#[allow(unused)]
pub fn block_on<F>(f: F)
where
    F: Future<Output = ()>,
{
    create_runtime().block_on(f)
}
