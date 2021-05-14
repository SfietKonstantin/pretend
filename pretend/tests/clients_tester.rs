use pretend::client::{Bytes, Client, Method};
use pretend::http::HeaderValue;
use pretend::{HeaderMap, Response, Result, Url};
use std::collections::HashMap;
use tokio::runtime::Runtime;

pub trait TestableClient {
    fn execute(
        &self,
        method: Method,
        url: Url,
        headers: HeaderMap,
        body: Option<Bytes>,
    ) -> Result<Response<Bytes>>;
}

pub struct TokioTestableClient<C>
where
    C: Client,
{
    client: C,
    runtime: Runtime,
}

impl<C> TokioTestableClient<C>
where
    C: Client,
{
    pub fn new(client: C, runtime: Runtime) -> Self {
        TokioTestableClient { client, runtime }
    }
}

impl<C> TestableClient for TokioTestableClient<C>
where
    C: Client,
{
    fn execute(
        &self,
        method: Method,
        url: Url,
        headers: HeaderMap,
        body: Option<Bytes>,
    ) -> Result<Response<Bytes>> {
        self.runtime
            .block_on(async { self.client.execute(method, url, headers, body).await })
    }
}

pub struct ClientsTester {
    url: Url,
    clients: Vec<Box<dyn TestableClient>>,
}

impl ClientsTester {
    pub fn new(url: Url, clients: Vec<Box<dyn TestableClient>>) -> Self {
        ClientsTester { url, clients }
    }

    pub fn test(self) {
        for client in &self.clients {
            self.test_methods(Box::as_ref(client));
            self.test_headers(Box::as_ref(client));
            self.test_bodies(Box::as_ref(client));
        }
    }

    fn test_methods(&self, client: &dyn TestableClient) {
        self.test_method(client, Method::GET);
        self.test_method(client, Method::POST);
        self.test_method(client, Method::PUT);
        self.test_method(client, Method::PATCH);
        self.test_method(client, Method::DELETE);
    }

    fn test_method(&self, client: &dyn TestableClient, method: Method) {
        let method_name = method.to_string();
        let url = self.url.join("/method").unwrap();

        let response = client.execute(method, url, HeaderMap::new(), None).unwrap();

        assert_eq!(response.status().as_u16(), 200);
        let body = String::from_utf8(response.body().to_vec()).unwrap();
        assert_eq!(body, method_name);
    }

    fn test_headers(&self, client: &dyn TestableClient) {
        let url = self.url.join("/headers").unwrap();
        let mut headers = HeaderMap::new();
        headers.append("X-Test-Header-1", HeaderValue::from_static("abc"));
        headers.append("X-Test-Header-2", HeaderValue::from_static("123"));
        let response = client.execute(Method::GET, url, headers, None).unwrap();

        assert_eq!(response.status().as_u16(), 200);
        let headers: HashMap<String, String> =
            serde_json::from_slice(response.body().as_ref()).unwrap();
        assert_eq!(headers.get("x-test-header-1").unwrap(), "abc");
        assert_eq!(headers.get("x-test-header-2").unwrap(), "123");
    }

    fn test_bodies(&self, client: &dyn TestableClient) {
        let expected = "Hello World";

        let url = self.url.join("/post/string").unwrap();
        let mut headers = HeaderMap::new();
        headers.append("Content-Type", HeaderValue::from_static("text/plain"));
        let body = Some(Bytes::from(expected));
        let response = client.execute(Method::POST, url, headers, body).unwrap();

        assert_eq!(response.status().as_u16(), 200);
        let body = String::from_utf8(response.body().to_vec()).unwrap();
        assert_eq!(body, expected);
    }
}
