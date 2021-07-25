//! `awc` based `pretend` client

#![warn(missing_docs)]
#![forbid(unsafe_code)]

pub use awc;

use awc::http::{HeaderName, HeaderValue};
use awc::Client as AClient;
use pretend::client::{async_trait, Bytes, Method};
use pretend::http::header::{HeaderName as PHeaderName, HeaderValue as PHeaderValue};
use pretend::local::client::Client as PLClient;
use pretend::local::{Error, Result};
use pretend::{HeaderMap, Response, Url};

/// `awc` based `pretend` client
#[derive(Clone, Default)]
pub struct Client {
    client: AClient,
}

impl Client {
    /// Constructor with custom client
    ///
    /// This constructor creates a client implementation
    /// for `pretend` wrapping the supplied `awc` client.
    pub fn new(client: AClient) -> Self {
        Client { client }
    }
}

#[async_trait(?Send)]
impl PLClient for Client {
    async fn execute(
        &self,
        method: Method,
        url: Url,
        headers: HeaderMap,
        body: Option<Bytes>,
    ) -> Result<Response<Bytes>> {
        let mut request = self.client.request(method, url.as_str());
        for (name, value) in headers.iter() {
            request = request.set_header(name, value.as_bytes());
        }

        let future = if let Some(body) = body {
            request.send_body(body.to_vec())
        } else {
            request.send()
        };

        let mut response = future.await.map_err(|err| Error::Response(Box::new(err)))?;
        let status = response.status();
        let headers = response.headers();
        let headers = headers.iter().map(create_header).collect::<HeaderMap>();
        let future = response.body();
        let result = future.await.map_err(|err| Error::Body(Box::new(err)))?;
        Ok(Response::new(status, headers, Bytes::from(result.to_vec())))
    }
}

fn create_header((name, value): (&HeaderName, &HeaderValue)) -> (PHeaderName, PHeaderValue) {
    (PHeaderName::from(name), PHeaderValue::from(value))
}
