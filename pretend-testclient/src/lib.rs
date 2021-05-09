use pretend::client::{async_trait, Bytes, Client as PClient, Method};
use pretend::{Error, HeaderMap, Response, Result, Url};
use std::io;

pub type ClientCallback =
    fn(Method, Url, HeaderMap, Option<Bytes>) -> Result<Response<Option<Vec<u8>>>>;

pub struct Client {
    callback: ClientCallback,
}

impl Client {
    pub fn new(callback: ClientCallback) -> Self {
        Client { callback }
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
        let response = (self.callback)(method, url, headers, body)?;
        response.try_map_body(map_body)
    }
}

fn map_body(body: Option<Vec<u8>>) -> Result<Bytes> {
    let body = body.ok_or_else(|| {
        let error = io::Error::new(io::ErrorKind::Other, "No body");
        Error::Body(Box::new(error))
    })?;
    Ok(Bytes::from(body))
}
