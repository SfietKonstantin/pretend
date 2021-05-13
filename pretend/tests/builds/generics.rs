#![allow(unused_imports)]

use pretend::{pretend, request, Result};

#[pretend]
trait Test {
    #[request(method = "GET", path = "/get")]
    async fn test_1<T>(&self) -> Result<()>;
    #[request(method = "GET", path = "/get")]
    async fn test_2(&self) -> Result<()>
    where
        Self: Sized;
}

fn main() {}
