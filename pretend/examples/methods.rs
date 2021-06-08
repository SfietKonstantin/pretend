use pretend::{pretend, Pretend, Result, Url};
use pretend_reqwest::Client;

// This example show how to send HTTP requests to https://httpbin.org
// using different methods

#[pretend]
trait HttpBin {
    #[request(method = "GET", path = "/get")]
    async fn get(&self) -> Result<String>;

    #[request(method = "POST", path = "/post")]
    async fn post(&self) -> Result<String>;

    #[request(method = "PUT", path = "/put")]
    async fn put(&self) -> Result<String>;

    #[request(method = "DELETE", path = "/delete")]
    async fn delete(&self) -> Result<String>;
}

fn create_pretend() -> impl HttpBin {
    let url = Url::parse("https://httpbin.org").unwrap();
    Pretend::for_client(Client::default()).with_url(url)
}

#[tokio::main]
async fn main() {
    let pretend = create_pretend();

    let get = pretend.get().await.unwrap();
    println!("{}", get);

    let post = pretend.post().await.unwrap();
    println!("{}", post);

    let put = pretend.post().await.unwrap();
    println!("{}", put);

    let delete = pretend.post().await.unwrap();
    println!("{}", delete);
}
