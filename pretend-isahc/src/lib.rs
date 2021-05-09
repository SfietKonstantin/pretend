pub use isahc;

use isahc::http::Request;
use isahc::{AsyncBody, AsyncReadResponseExt, HttpClient};
use pretend::client::{async_trait, Bytes, Client as PClient, Method};
use pretend::{Error, HeaderMap, Response, Result, Url};
use std::mem;

pub struct Client {
    client: HttpClient,
}

impl Client {
    pub fn with_client(client: HttpClient) -> Self {
        Client { client }
    }

    pub fn new() -> Result<Self> {
        let client = HttpClient::new().map_err(|err| Error::Client(Box::new(err)))?;
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

        let request = request.map_err(|err| Error::Request(Box::new(err)))?;
        let response = self.client.send_async(request).await;
        let mut response = response.map_err(|err| Error::Response(Box::new(err)))?;
        // let (parts, body) = response.into_parts();

        let status = mem::take(response.status_mut());
        let headers = mem::take(response.headers_mut());
        let mut body = Vec::new();
        let result = response.copy_to(&mut body).await;
        result.map_err(|err| Error::Body(Box::new(err)))?;
        Ok(Response::new(status, headers, Bytes::from(body)))
    }
}
