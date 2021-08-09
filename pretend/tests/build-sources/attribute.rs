#![allow(unused_imports)]

use pretend::{pretend, request, Result};

#[pretend(local)]
trait Test1 {
    #[request(method = "GET", path = "/get")]
    async fn test_1(&self) -> Result<()>;
}

#[pretend(blocking)]
trait Test2 {
    #[request(method = "GET", path = "/get")]
    fn test_1(&self) -> Result<()>;
}

fn main() {}
