use pretend::{pretend, Pretend, Result, Url};
use pretend_reqwest::BlockingClient;

// This example show how the use of a blocking client

#[pretend]
trait HttpBin {
    #[request(method = "GET", path = "/get")]
    fn get(&self) -> Result<String>;
}

fn create_pretend() -> impl HttpBin {
    let url = Url::parse("https://httpbin.org").unwrap();
    Pretend::for_client(BlockingClient::default()).with_url(url)
}

fn main() {
    let pretend = create_pretend();

    let get = pretend.get().unwrap();
    println!("{}", get);
}
