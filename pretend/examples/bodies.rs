use pretend::{pretend, Pretend, Result, Url};
use pretend_reqwest::Client;
use serde::Serialize;

// This example show how to send various body types to https://httpbin.org

#[derive(Clone, Serialize)]
struct Data {
    first: String,
    second: i32,
}

#[pretend]
trait HttpBin {
    #[request(method = "POST", path = "/anything")]
    #[header(name = "Content-Type", value = "text/plain")]
    async fn post_string_ref(&self, body: &'static str) -> Result<String>;

    #[request(method = "POST", path = "/anything")]
    #[header(name = "Content-Type", value = "text/plain")]
    async fn post_string(&self, body: String) -> Result<String>;

    #[request(method = "POST", path = "/anything")]
    #[header(name = "Content-Type", value = "application/octet-stream")]
    async fn post_bytes_ref(&self, body: &'static [u8]) -> Result<String>;

    #[request(method = "POST", path = "/anything")]
    #[header(name = "Content-Type", value = "application/octet-stream")]
    async fn post_bytes(&self, body: Vec<u8>) -> Result<String>;

    #[request(method = "POST", path = "/anything")]
    async fn post_form_ref(&self, form: &Data) -> Result<String>;

    #[request(method = "POST", path = "/anything")]
    async fn post_form(&self, form: Data) -> Result<String>;

    #[request(method = "POST", path = "/anything")]
    async fn post_json_ref(&self, json: &Data) -> Result<String>;

    #[request(method = "POST", path = "/anything")]
    async fn post_json(&self, json: Data) -> Result<String>;
}

fn create_pretend() -> impl HttpBin {
    let url = Url::parse("https://httpbin.org").unwrap();
    Pretend::for_client(Client::default()).with_url(url)
}

#[tokio::main]
async fn main() {
    let pretend = create_pretend();

    let result = pretend.post_string_ref("Hello").await.unwrap();
    println!("{}", result);

    let result = pretend.post_string("Hello".to_string()).await.unwrap();
    println!("{}", result);

    let result = pretend.post_bytes_ref(&[1, 2, 3]).await.unwrap();
    println!("{}", result);

    let result = pretend.post_bytes(vec![1, 2, 3]).await.unwrap();
    println!("{}", result);

    let data = Data {
        first: "Hello".to_string(),
        second: 123,
    };

    let result = pretend.post_form_ref(&data).await.unwrap();
    println!("{}", result);

    let result = pretend.post_form(data.clone()).await.unwrap();
    println!("{}", result);

    let result = pretend.post_json_ref(&data).await.unwrap();
    println!("{}", result);

    let result = pretend.post_json(data.clone()).await.unwrap();
    println!("{}", result);
}
