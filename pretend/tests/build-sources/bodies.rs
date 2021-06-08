#![allow(unused_imports)]

use pretend::{pretend, Result};

#[pretend]
trait Test {
    #[request(method = "GET", path = "/get")]
    async fn test_1(&self, body: String, json: String) -> Result<()>;
}

fn main() {}
