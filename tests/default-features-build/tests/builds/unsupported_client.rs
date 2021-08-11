use pretend::client::{BlockingClient, Bytes, Method};
use pretend::{Error, HeaderMap, Response, Result, Url};
use std::rc::Rc;
use thiserror::Error;

#[derive(Default, Debug, Error)]
#[error("Test error")]
struct TestError {
    data: Rc<i32>,
}

struct TestClient;

impl BlockingClient for TestClient {
    fn execute(
        &self,
        _: Method,
        _: Url,
        _: HeaderMap,
        _: Option<Bytes>,
    ) -> Result<Response<Bytes>> {
        Err(Error::response(TestError::default()))
    }
}

fn main() {}