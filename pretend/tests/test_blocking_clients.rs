mod server;

use pretend::{header, pretend, request, Json, Pretend, Result, Url};
use pretend_reqwest::BlockingClient as RClient;
use std::collections::HashMap;

#[pretend(blocking)]
trait TestApi {
    #[request(method = "GET", path = "/get")]
    fn get(&self) -> Result<String>;
    #[request(method = "POST", path = "/post")]
    fn post(&self) -> Result<String>;
    #[request(method = "PUT", path = "/put")]
    fn put(&self) -> Result<String>;
    #[request(method = "PATCH", path = "/patch")]
    fn patch(&self) -> Result<String>;
    #[request(method = "DELETE", path = "/delete")]
    fn delete(&self) -> Result<String>;
    #[request(method = "GET", path = "/query")]
    fn query(&self, query: &server::TestData) -> Result<Json<HashMap<String, String>>>;
    #[request(method = "GET", path = "/headers")]
    #[header(name = "X-Test-Header-1", value = "abc")]
    #[header(name = "X-Test-Header-2", value = "{value}")]
    #[header(name = "X-{custom}", value = "custom")]
    fn headers(&self, value: i32, custom: &str) -> Result<Json<HashMap<String, String>>>;
    #[request(method = "POST", path = "/post/string")]
    #[header(name = "Content-Type", value = "text/plain")]
    fn post_string(&self, body: &'static str) -> Result<String>;
    #[request(method = "POST", path = "/post/json")]
    fn post_json(&self, json: &server::TestData) -> Result<Json<server::TestData>>;
    #[request(method = "POST", path = "/post/form")]
    fn post_form(&self, form: &server::TestData) -> Result<Json<server::TestData>>;
}

fn execute_test<F>(check: F)
where
    F: Fn(Box<dyn TestApi>),
{
    let url = Url::parse(server::URL).unwrap();

    let client = RClient::default();
    let client = Pretend::for_client(client).with_url(url.clone());

    check(Box::new(client));
}

#[test]
fn test_clients() {
    server::test_sync(|| {
        test_get();
        test_post();
        test_put();
        test_patch();
        test_delete();

        test_query();
        test_headers();
        test_post_string();
        test_post_json();
        test_post_form();
    });
}

fn test_get() {
    execute_test(|api| {
        let result = api.get();
        assert!(result.is_ok());
    })
}

fn test_post() {
    execute_test(|api| {
        let result = api.post();
        assert!(result.is_ok());
    })
}

fn test_put() {
    execute_test(|api| {
        let result = api.put();
        assert!(result.is_ok());
    })
}

fn test_patch() {
    execute_test(|api| {
        let result = api.patch();
        assert!(result.is_ok());
    })
}

fn test_delete() {
    execute_test(|api| {
        let result = api.delete();
        assert!(result.is_ok());
    })
}

fn test_query() {
    execute_test(|api| {
        let query = server::TestData {
            first: "Hello".to_string(),
            second: 123,
        };

        let expected_args = [("first", "Hello"), ("second", "123")]
            .iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect::<HashMap<_, _>>();
        let result = api.query(&query).unwrap();
        assert_eq!(result.value(), expected_args);
    })
}

fn test_headers() {
    execute_test(|api| {
        let result = api.headers(123, "test").unwrap();
        let headers = result.value();
        assert_eq!(headers.get("x-test-header-1").unwrap(), "abc");
        assert_eq!(headers.get("x-test-header-2").unwrap(), "123");
        assert_eq!(headers.get("x-test").unwrap(), "custom");
    })
}

fn test_post_string() {
    execute_test(|api| {
        let expected = "Hello";
        let result = api.post_string(expected).unwrap();
        assert_eq!(expected, result);
    })
}

fn test_post_json() {
    execute_test(|api| {
        let json = server::TestData {
            first: "Hello".to_string(),
            second: 123,
        };
        let result = api.post_json(&json).unwrap();
        assert_eq!(result.value(), json);
    })
}

fn test_post_form() {
    execute_test(|api| {
        let json = server::TestData {
            first: "Hello".to_string(),
            second: 123,
        };
        let result = api.post_form(&json).unwrap();
        assert_eq!(result.value(), json);
    })
}
