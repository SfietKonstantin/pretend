use pretend::{pretend, request, Pretend, Result, Serialize, Url};
use pretend_reqwest::Client as RClient;

// This example show how to pass URL queries to https://httpbin.org

#[derive(Clone, Serialize)]
struct Query {
    first: String,
    second: i32,
}

#[pretend]
trait HttpBin {
    #[request(method = "GET", path = "/get?first={first}&second={second}")]
    async fn get(&self, first: String, second: i32) -> Result<String>;

    #[request(method = "GET", path = "/get")]
    async fn get_query(&self, query: Query) -> Result<String>;
}

fn create_pretend() -> impl HttpBin {
    let url = Url::parse("https://httpbin.org").unwrap();
    Pretend::for_client(RClient::default()).with_url(url)
}

#[tokio::main]
async fn main() {
    let pretend = create_pretend();

    let result = pretend.get("Hello".to_string(), 123).await.unwrap();
    println!("{}", result);

    let query = Query {
        first: "Hello".to_string(),
        second: 123,
    };

    let result = pretend.get_query(query).await.unwrap();
    println!("{}", result);
}
