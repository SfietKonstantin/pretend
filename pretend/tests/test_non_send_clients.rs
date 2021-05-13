mod server;

use pretend::{header, pretend, request, Json, Pretend, Result, Url};
use pretend_isahc::Client as IClient;
use pretend_reqwest::Client as RClient;
use std::collections::HashMap;
use std::future::Future;

#[pretend(non_send)]
trait TestApi {
    #[request(method = "GET", path = "/get")]
    async fn get(&self) -> Result<String>;
    #[request(method = "POST", path = "/post")]
    async fn post(&self) -> Result<String>;
    #[request(method = "PUT", path = "/put")]
    async fn put(&self) -> Result<String>;
    #[request(method = "PATCH", path = "/patch")]
    async fn patch(&self) -> Result<String>;
    #[request(method = "DELETE", path = "/delete")]
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

async fn execute_test<F, O>(check: F)
where
    F: Fn(Box<dyn TestApi>) -> O + 'static,
    O: Future<Output = ()> + 'static,
{
    let url = Url::parse(server::URL).unwrap();

    let client = RClient::default();
    let client = Pretend::for_client(client).with_url(url.clone());

    check(Box::new(client)).await;

    let client = IClient::new().unwrap();
    let client = Pretend::for_client(client).with_url(url.clone());

    check(Box::new(client)).await;
}

#[test]
fn test_clients() {
    server::test(async {
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
    });
}

async fn test_get() {
    execute_test(|api| async move {
        let result = api.get().await;
        assert!(result.is_ok());
    })
    .await;
}

async fn test_post() {
    execute_test(|api| async move {
        let result = api.post().await;
        assert!(result.is_ok());
    })
    .await;
}

async fn test_put() {
    execute_test(|api| async move {
        let result = api.put().await;
        assert!(result.is_ok());
    })
    .await;
}

async fn test_patch() {
    execute_test(|api| async move {
        let result = api.patch().await;
        assert!(result.is_ok());
    })
    .await;
}

async fn test_delete() {
    execute_test(|api| async move {
        let result = api.delete().await;
        assert!(result.is_ok());
    })
    .await;
}

async fn test_query() {
    execute_test(|api| async move {
        let query = server::TestData {
            first: "Hello".to_string(),
            second: 123,
        };

        let expected_args = [("first", "Hello"), ("second", "123")]
            .iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect::<HashMap<_, _>>();
        let result = api.query(&query).await.unwrap();
        assert_eq!(result.value(), expected_args);
    })
    .await;
}

async fn test_headers() {
    execute_test(|api| async move {
        let result = api.headers(123, "test").await.unwrap();
        let headers = result.value();
        assert_eq!(headers.get("x-test-header-1").unwrap(), "abc");
        assert_eq!(headers.get("x-test-header-2").unwrap(), "123");
        assert_eq!(headers.get("x-test").unwrap(), "custom");
    })
    .await;
}

async fn test_post_string() {
    execute_test(|api| async move {
        let expected = "Hello";
        let result = api.post_string(expected).await.unwrap();
        assert_eq!(expected, result);
    })
    .await;
}

async fn test_post_json() {
    execute_test(|api| async move {
        let json = server::TestData {
            first: "Hello".to_string(),
            second: 123,
        };
        let result = api.post_json(&json).await.unwrap();
        assert_eq!(result.value(), json);
    })
    .await;
}

async fn test_post_form() {
    execute_test(|api| async move {
        let json = server::TestData {
            first: "Hello".to_string(),
            second: 123,
        };
        let result = api.post_form(&json).await.unwrap();
        assert_eq!(result.value(), json);
    })
    .await;
}
