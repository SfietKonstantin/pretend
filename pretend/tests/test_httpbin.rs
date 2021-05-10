use pretend::{header, pretend, request, Deserialize, Json, Pretend, Result, Serialize, Url, UrlResolver};
use pretend_reqwest::Client as RClient;
use pretend_isahc::Client as IClient;
use std::collections::HashMap;
use pretend::client::Client;

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

struct Tester<C> where C: Client + Send + Sync {
    pretend: Pretend<C, UrlResolver>,
}

fn httpbin_url() -> Url {
    Url::parse("https://httpbin.org").unwrap()
}

impl Default for Tester<RClient> {
    fn default() -> Self {
        let client = RClient::default();
        let client = Pretend::for_client(client).with_url(httpbin_url());
        Tester {
            pretend: client
        }
    }
}

impl Default for Tester<IClient> {
    fn default() -> Self {
        let client = IClient::new().unwrap();
        let client = Pretend::for_client(client).with_url(httpbin_url());
        Tester {
            pretend: client
        }
    }
}

impl<C> Tester<C> where C: Client + Send + Sync {
    async fn test_get(self) {
        let result = self.pretend.get().await;
        assert!(result.is_ok());
    }

    async fn test_post(self) {
        let result = self.pretend.post().await;
        assert!(result.is_ok());
    }

    async fn test_put(self) {
        let result = self.pretend.put().await;
        assert!(result.is_ok());
    }

    async fn test_patch(self) {
        let result = self.pretend.patch().await;
        assert!(result.is_ok());
    }

    async fn test_delete(self) {
        let result = self.pretend.delete().await;
        assert!(result.is_ok());
    }

    async fn test_query(self) {
        let query = TestData {
            first: "Hello",
            second: 123,
        };
        let expected_args = [("first", "Hello"), ("second", "123")]
            .iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect::<HashMap<_, _>>();
        let result = self.pretend.get_with_form(query).await.unwrap();
        assert_eq!(result.value().args, expected_args);
    }

    async fn test_headers(self) {
        let result = self.pretend.get_with_headers(123, "test").await.unwrap();
        let headers = result.value().headers;
        assert_eq!(headers.get("X-Test-Header-1").unwrap(), "abc");
        assert_eq!(headers.get("X-Test-Header-2").unwrap(), "123");
        assert_eq!(headers.get("X-Test").unwrap(), "custom");
    }

    async fn test_json(self) {
        let query = TestJson {
            first: "Hello".to_string(),
            second: 123,
        };
        let result = self.pretend.post_with_json(&query).await.unwrap();
        assert_eq!(result.value().json, Some(query));
    }

    async fn test_form(self) {
        let form = TestData {
            first: "Hello",
            second: 123,
        };
        let expected_args = [("first", "Hello"), ("second", "123")]
            .iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect::<HashMap<_, _>>();
        let result = self.pretend.post_with_form(&form).await.unwrap();
        assert_eq!(result.value().form, Some(expected_args));
    }

}

macro_rules! gen_test {
    ($test:ident) => {
        #[tokio::test]
        async fn $test() {
            Tester::<RClient>::default().$test().await;
            Tester::<IClient>::default().$test().await;
        }
    }
}

gen_test!(test_get);
gen_test!(test_post);
gen_test!(test_put);
gen_test!(test_patch);
gen_test!(test_delete);
gen_test!(test_query);
gen_test!(test_headers);
gen_test!(test_json);
gen_test!(test_form);
