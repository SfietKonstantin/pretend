pub use reqwest;

use pretend::client::{async_trait, Bytes, Client as PClient, Method};
use pretend::{Error, HeaderMap, Response as PResponse, Result, Url};
use reqwest::Client as RClient;
use std::mem;

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
    async fn execute(
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
        let response = builder.send().await;
        let mut response = response.map_err(|err| Error::Response(Box::new(err)))?;

        let status = response.status();
        let headers = mem::take(response.headers_mut());

        let bytes = response.bytes().await;
        let bytes = bytes.map_err(|err| Error::Body(Box::new(err)))?;

        Ok(PResponse::new(status, headers, bytes))
    }
}
