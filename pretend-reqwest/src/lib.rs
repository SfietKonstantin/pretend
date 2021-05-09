pub use reqwest;

use pretend::client::{
    Client as PClient, RequestBuilder as PRequestBuilder, ResponseBody as PResponseBody,
};
use pretend::{
    async_trait, DeserializeOwned, Error, HeaderMap, Method, Response as PResponse, Result,
    Serialize, Url,
};
use reqwest::{Client as RClient, RequestBuilder as RRequestBuilder, Response as RResponse};

#[derive(Default)]
pub struct Client {
    client: RClient,
}

impl Client {
    pub fn new(client: RClient) -> Self {
        Client { client }
    }
}

#[async_trait]
impl PClient for Client {
    type Request = Request;
    type RequestBuilder = RequestBuilder;
    type ResponseBody = ResponseBody;

    fn request_builder(&self, method: Method, url: Url) -> Result<Self::RequestBuilder> {
        let builder = self.client.request(method.clone(), url.clone());
        Ok(RequestBuilder {
            request: Request {
                url,
                method,
                builder,
            },
        })
    }

    async fn execute(&self, request: Self::Request) -> Result<PResponse<Self::ResponseBody>> {
        let builder = request.builder;
        let method = request.method;
        let url = request.url;

        let response = builder.send().await;
        let response = response.map_err(|err| Error::Response {
            method,
            url,
            source: Box::new(err),
        })?;

        Ok(PResponse::new(
            response.status(),
            response.headers().clone(),
            ResponseBody { response },
        ))
    }
}

pub struct Request {
    method: Method,
    url: Url,
    builder: RRequestBuilder,
}

pub struct RequestBuilder {
    request: Request,
}

impl PRequestBuilder for RequestBuilder {
    type Request = Request;

    fn headers(mut self, headers: HeaderMap) -> Result<Self> {
        self.request.builder = self.request.builder.headers(headers);
        Ok(self)
    }

    fn body(mut self, body: Vec<u8>) -> Result<Self> {
        self.request.builder = self.request.builder.body(body);
        Ok(self)
    }

    fn query<T>(mut self, query: &T) -> Result<Self>
    where
        T: Serialize,
    {
        self.request.builder = self.request.builder.query(query);
        Ok(self)
    }

    fn form<T>(mut self, form: &T) -> Result<Self>
    where
        T: Serialize,
    {
        self.request.builder = self.request.builder.form(form);
        Ok(self)
    }

    fn json<T>(mut self, json: &T) -> Result<Self>
    where
        T: Serialize,
    {
        self.request.builder = self.request.builder.json(json);
        Ok(self)
    }

    fn build(self) -> Self::Request {
        self.request
    }
}

pub struct ResponseBody {
    response: RResponse,
}

#[async_trait]
impl PResponseBody for ResponseBody {
    async fn text(self) -> Result<String> {
        body_response(self.response.text().await)
    }

    async fn bytes(self) -> Result<Vec<u8>> {
        body_response(self.response.bytes().await.map(|bytes| bytes.to_vec()))
    }

    async fn json<T>(self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        body_response(self.response.json().await)
    }
}

fn body_response<T>(response: reqwest::Result<T>) -> Result<T> {
    response.map_err(|err| Error::Body(Box::new(err)))
}
