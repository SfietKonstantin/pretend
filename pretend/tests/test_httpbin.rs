use pretend::{header, pretend, request, Deserialize, Json, Pretend, Result, Serialize, Url};
use pretend_reqwest::Client as RClient;
use std::collections::HashMap;

#[derive(Deserialize)]
struct ReturnValue<T = ()> {
    args: HashMap<String, String>,
    headers: HashMap<String, String>,
    form: Option<HashMap<String, String>>,
    json: Option<T>,
}

#[derive(Serialize)]
struct TestData {
    first: &'static str,
    second: i32,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
struct TestJson {
    first: String,
    second: i32,
}

#[pretend]
trait HttpBin {
    #[request(method = "GET", path = "/get")]
    async fn get(&self) -> Result<Json<ReturnValue>>;
    #[request(method = "POST", path = "/post")]
    async fn post(&self) -> Result<Json<ReturnValue>>;
    #[request(method = "PUT", path = "/put")]
    async fn put(&self) -> Result<Json<ReturnValue>>;
    #[request(method = "PATCH", path = "/patch")]
    async fn patch(&self) -> Result<Json<ReturnValue>>;
    #[request(method = "DELETE", path = "/delete")]
    async fn delete(&self) -> Result<Json<ReturnValue>>;
    #[request(method = "GET", path = "/get")]
    async fn get_with_form(&self, query: TestData) -> Result<Json<ReturnValue>>;
    #[request(method = "GET", path = "/get")]
    #[header(name = "X-Test-Header-1", value = "abc")]
    #[header(name = "X-Test-Header-2", value = "{value}")]
    #[header(name = "X-{custom}", value = "custom")]
    async fn get_with_headers(&self, value: i32, custom: &str) -> Result<Json<ReturnValue>>;
    #[request(method = "POST", path = "/anything")]
    async fn post_with_json(&self, json: &TestJson) -> Result<Json<ReturnValue<TestJson>>>;
    #[request(method = "POST", path = "/anything")]
    async fn post_with_form(&self, form: &TestData) -> Result<Json<ReturnValue>>;
}

fn create_client() -> impl HttpBin {
    let client = RClient::default();
    let url = Url::parse("https://httpbin.org").unwrap();
    Pretend::for_client(client).with_url(url)
}

#[tokio::test]
async fn test_get() {
    let result = create_client().get().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_post() {
    let result = create_client().post().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_put() {
    let result = create_client().put().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_patch() {
    let result = create_client().patch().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete() {
    let result = create_client().delete().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_query() {
    let query = TestData {
        first: "Hello",
        second: 123,
    };
    let expected_args = [("first", "Hello"), ("second", "123")]
        .iter()
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .collect::<HashMap<_, _>>();
    let result = create_client().get_with_form(query).await.unwrap();
    assert_eq!(result.value().args, expected_args);
}

#[tokio::test]
async fn test_headers() {
    let result = create_client().get_with_headers(123, "test").await.unwrap();
    let headers = result.value().headers;
    assert_eq!(headers.get("X-Test-Header-1").unwrap(), "abc");
    assert_eq!(headers.get("X-Test-Header-2").unwrap(), "123");
    assert_eq!(headers.get("X-Test").unwrap(), "custom");
}

#[tokio::test]
async fn test_json() {
    let query = TestJson {
        first: "Hello".to_string(),
        second: 123,
    };
    let result = create_client().post_with_json(&query).await.unwrap();
    assert_eq!(result.value().json, Some(query));
}

#[tokio::test]
async fn test_form() {
    let form = TestData {
        first: "Hello",
        second: 123,
    };
    let expected_args = [("first", "Hello"), ("second", "123")]
        .iter()
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .collect::<HashMap<_, _>>();
    let result = create_client().post_with_form(&form).await.unwrap();
    assert_eq!(result.value().form, Some(expected_args));
}
