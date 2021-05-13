#![allow(unused_imports)]

use pretend::{pretend, request, Result};

#[pretend]
trait Test1 {}

#[pretend]
trait Test2 {
    #[request(method = "GET", path = "/get")]
    fn test_1(&self) -> Result<()>;
    #[request(method = "GET", path = "/get")]
    async fn test_2(&self) -> Result<()>;
}

#[pretend(?Send)]
trait Test3 {
    #[request(method = "GET", path = "/get")]
    fn test_1(&self) -> Result<()>;
}

fn main() {}
