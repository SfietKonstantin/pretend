mod runtimes;
mod server;

use pretend::{header, pretend, request, Json, Pretend, Result, Url};
use pretend_reqwest::Client;
use std::collections::HashMap;

#[pretend]
trait TestApi {
    #[request(method = "GET", path = "/method")]
    async fn get(&self) -> Result<String>;
    #[request(method = "POST", path = "/method")]
    async fn post(&self) -> Result<String>;
    #[request(method = "PUT", path = "/method")]
    async fn put(&self) -> Result<String>;
    #[request(method = "PATCH", path = "/method")]
    async fn patch(&self) -> Result<String>;
    #[request(method = "DELETE", path = "/method")]
    async fn delete(&self) -> Result<String>;
    #[request(method = "GET", path = "/query")]
    async fn query(&self, query: &server::TestData) -> Result<Json<HashMap<String, String>>>;
    #[request(method = "GET", path = "/headers")]
    #[header(name = "X-Test-Header-1", value = "abc")]
    #[header(name = "X-Test-Header-2", value = "{value}")]
    #[header(name = "X-{custom}", value = "custom")]
    async fn headers(&self, value: i32, custom: &str) -> Result<Json<HashMap<String, String>>>;
    #[request(method = "POST", path = "/post/string")]
    #[header(name = "Content-Type", value = "text/plain")]
    async fn post_string(&self, body: &'static str) -> Result<String>;
    #[request(method = "POST", path = "/post/json")]
    async fn post_json(&self, json: &server::TestData) -> Result<Json<server::TestData>>;
    #[request(method = "POST", path = "/post/form")]
    async fn post_form(&self, form: &server::TestData) -> Result<Json<server::TestData>>;
}

fn new_pretend() -> impl TestApi {
    let url = Url::parse(server::URL).unwrap();
    let client = Client::default();
    Pretend::for_client(client).with_url(url)
}

#[test]
fn test_pretend() {
    server::test(|| {
        runtimes::block_on(async {
            test_get().await;
            test_post().await;
            test_put().await;
            test_patch().await;
            test_delete().await;

            test_query().await;
            test_headers().await;
            test_post_string().await;
            test_post_json().await;
            test_post_form().await;
        })
    });
}

async fn test_get() {
    let result = new_pretend().get().await;
    assert!(result.is_ok());
}

async fn test_post() {
    let result = new_pretend().post().await;
    assert!(result.is_ok());
}

async fn test_put() {
    let result = new_pretend().put().await;
    assert!(result.is_ok());
}

async fn test_patch() {
    let result = new_pretend().patch().await;
    assert!(result.is_ok());
}

async fn test_delete() {
    let result = new_pretend().delete().await;
    assert!(result.is_ok());
}

async fn test_query() {
    let query = server::TestData {
        first: "Hello".to_string(),
        second: 123,
    };

    let expected_args = [("first", "Hello"), ("second", "123")]
        .iter()
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .collect::<HashMap<_, _>>();

    let result = new_pretend().query(&query).await.unwrap();
    assert_eq!(result.value(), expected_args);
}

async fn test_headers() {
    let result = new_pretend().headers(123, "test").await.unwrap();
    let headers = result.value();
    assert_eq!(headers.get("x-test-header-1").unwrap(), "abc");
    assert_eq!(headers.get("x-test-header-2").unwrap(), "123");
    assert_eq!(headers.get("x-test").unwrap(), "custom");
}

async fn test_post_string() {
    let expected = "Hello";
    let result = new_pretend().post_string(expected).await.unwrap();
    assert_eq!(expected, result);
}

async fn test_post_json() {
    let json = server::TestData {
        first: "Hello".to_string(),
        second: 123,
    };
    let result = new_pretend().post_json(&json).await.unwrap();
    assert_eq!(result.value(), json);
}

async fn test_post_form() {
    let json = server::TestData {
        first: "Hello".to_string(),
        second: 123,
    };
    let result = new_pretend().post_form(&json).await.unwrap();
    assert_eq!(result.value(), json);
}
