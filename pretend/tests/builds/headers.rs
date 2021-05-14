#![allow(unused_imports)]

use pretend::{header, pretend, request, Result};

#[pretend]
trait Test {
    #[request(method = "GET", path = "/get")]
    #[header(value = "test")]
    async fn test_1(&self) -> Result<()>;
    #[request(method = "GET", path = "/get")]
    #[header(name = "X-Test")]
    async fn test_2(&self) -> Result<()>;
    #[request(method = "GET", path = "/get")]
    #[header(name = "X-Test", value = "test", other = "something")]
    async fn test_3(&self) -> Result<()>;
}

fn main() {}
