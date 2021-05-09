use pretend::{pretend, request, Pretend, Result, Url};
use pretend_reqwest::Client as RClient;

// This example defines a simple pretend-based trait
// that returns the result of a call to https://httpbin.org/get

#[pretend]
trait JsonApi {
    #[request(method = "GET", path = "/get")]
    async fn get(&self) -> Result<String>;
}

#[tokio::main]
async fn main() {
    let url = Url::parse("https://httpbin.org").unwrap();
    let pretend = Pretend::for_client(RClient::default()).with_url(url);
    let result = pretend.get().await.unwrap();
    println!("{}", result);
}
