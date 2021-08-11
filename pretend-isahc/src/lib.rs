//! `isahc` based `pretend` client

#![warn(missing_docs)]
#![forbid(unsafe_code)]

pub use isahc;

use isahc::http::Request;
use isahc::{AsyncBody, AsyncReadResponseExt, HttpClient};
use pretend::client::{async_trait, Bytes, Client as PClient, Method};
use pretend::{Error, HeaderMap, Response, Result, Url};
use std::mem;

/// `ishac` based `pretend` client
#[derive(Clone, Debug)]
pub struct Client {
    client: HttpClient,
}

impl Client {
    /// Constructor with custom client
    ///
    /// This constructor creates a client implementation
    /// for `pretend` wrapping the supplied `isahc` client.
    pub fn with_client(client: HttpClient) -> Self {
        Client { client }
    }

    /// Constructor
    ///
    /// This constructor creates a client implementation
    /// for `pretend` using a default `isahc` client.
    pub fn new() -> Result<Self> {
        let client = HttpClient::new().map_err(Error::client)?;
        Ok(Client { client })
    }
}

#[async_trait]
impl PClient for Client {
    async fn execute(
        &self,
        method: Method,
        url: Url,
        headers: HeaderMap,
        body: Option<Bytes>,
    ) -> Result<Response<Bytes>> {
        let mut builder = Request::builder().method(method).uri(url.as_str());

        for (name, value) in headers.iter() {
            builder = builder.header(name, value);
        }

        let request = if let Some(body) = body {
            builder.body(AsyncBody::from_bytes_static(body))
        } else {
            builder.body(AsyncBody::empty())
        };

        let request = request.map_err(Error::request)?;
        let response = self.client.send_async(request).await;
        let mut response = response.map_err(Error::response)?;

        let status = mem::take(response.status_mut());
        let headers = mem::take(response.headers_mut());
        let mut body = Vec::new();
        let result = response.copy_to(&mut body).await;
        result.map_err(Error::body)?;
        Ok(Response::new(status, headers, Bytes::from(body)))
    }
}
