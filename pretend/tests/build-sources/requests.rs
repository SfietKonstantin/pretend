#![allow(unused_imports)]

use pretend::{pretend, Result};

#[pretend]
trait Test {
    async fn test_1(&self) -> Result<()>;
    #[request(method = "GET", path = "/get")]
    #[request(method = "GET", path = "/get")]
    async fn test_2(&self) -> Result<()>;
    #[request(method = "GET")]
    async fn test_3(&self) -> Result<()>;
    #[request(path = "/get")]
    async fn test_4(&self) -> Result<()>;
    #[request(method = "GET", path = "/get", other = "something")]
    async fn test_5(&self) -> Result<()>;
}

fn main() {}
