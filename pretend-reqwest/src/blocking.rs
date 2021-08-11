use pretend::client::{BlockingClient as PBlockingClient, Bytes, Method};
use pretend::{Error, HeaderMap, Response as PResponse, Result, Url};
use reqwest::blocking::Client;
use std::mem;

/// `reqwest` based `pretend` blocking client
#[derive(Clone, Debug, Default)]
pub struct BlockingClient {
    client: Client,
}

impl BlockingClient {
    /// Constructor with custom client
    ///
    /// This constructor creates a client implementation
    /// for `pretend` wrapping the supplied `reqwest` client.
    pub fn new(client: Client) -> Self {
        BlockingClient { client }
    }
}

impl PBlockingClient for BlockingClient {
    fn execute(
        &self,
        method: Method,
        url: Url,
        headers: HeaderMap,
        body: Option<Bytes>,
    ) -> Result<PResponse<Bytes>> {
        let mut builder = self.client.request(method, url).headers(headers);
        if let Some(body) = body {
            builder = builder.body(body);
        }
        let response = builder.send();
        let mut response = response.map_err(Error::response)?;

        let status = response.status();
        let headers = mem::take(response.headers_mut());

        let bytes = response.bytes();
        let bytes = bytes.map_err(Error::body)?;

        Ok(PResponse::new(status, headers, bytes))
    }
}
