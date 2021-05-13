#![allow(unused_imports)]

use pretend::{pretend, request, Result};

#[pretend]
trait Test {
    #[request(method = "GET", path = "/get")]
    async fn test_1() -> Result<()>;
    #[request(method = "GET", path = "/get")]
    async fn test_2(self) -> Result<()>;
    #[request(method = "GET", path = "/get")]
    async fn test_3(&mut self) -> Result<()>;
    #[request(method = "GET", path = "/get")]
    async fn test_4(input: i32) -> Result<()>;
}

fn main() {}
