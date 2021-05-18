use pretend::{header, pretend, request, Pretend, Result, Url};
use pretend_reqwest::Client;

// This example show how to send various headers to https://httpbin.org

#[pretend]
trait HttpBin {
    #[request(method = "GET", path = "/get")]
    #[header(name = "X-Test", value = "Hello")]
    #[header(name = "X-Something-Nice", value = "Lovely")]
    async fn get(&self) -> Result<String>;
}

fn create_pretend() -> impl HttpBin {
    let url = Url::parse("https://httpbin.org").unwrap();
    Pretend::for_client(Client::default()).with_url(url)
}

#[tokio::main]
async fn main() {
    let pretend = create_pretend();

    let result = pretend.get().await.unwrap();
    println!("{}", result);
}
