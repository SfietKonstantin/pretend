//! `ureq` based `pretend` client

#![warn(missing_docs)]
#![forbid(unsafe_code)]

pub use ureq;

use pretend::client::{BlockingClient, Bytes, Method};
use pretend::http::header::HeaderName;
use pretend::http::HeaderValue;
use pretend::{Error, HeaderMap, Response as PResponse, Result, StatusCode, Url};
use std::convert::TryFrom;
use std::io::Read;
use ureq::Agent;

/// `ureq` based `pretend` client
pub struct Client {
    agent: Agent,
}

impl Client {
    /// Constructor with custom agent
    ///
    /// This constructor creates a client implementation
    /// for `pretend` wrapping the supplied `ureq` agent.
    pub fn new(agent: Agent) -> Self {
        Client { agent }
    }
}

impl BlockingClient for Client {
    fn execute(
        &self,
        method: Method,
        url: Url,
        headers: HeaderMap,
        body: Option<Bytes>,
    ) -> Result<PResponse<Bytes>> {
        let mut request = self.agent.request_url(method.as_str(), &url);

        for (name, value) in headers.iter() {
            let value = value.to_str();
            let value = value.map_err(|err| Error::Request(Box::new(err)))?;
            request = request.set(name.as_str(), value);
        }

        let response = if let Some(body) = body {
            request.send_bytes(&body)
        } else {
            request.call()
        };

        let response = response.map_err(|err| Error::Response(Box::new(err)))?;

        let status = StatusCode::from_u16(response.status());
        let status = status.map_err(|err| Error::Response(Box::new(err)))?;

        let mut headers = HeaderMap::new();
        for name in response.headers_names() {
            let values = response.all(&name);

            let name = HeaderName::try_from(&name);
            let name = name.map_err(|err| Error::Response(Box::new(err)))?;

            for value in values {
                let value = HeaderValue::try_from(value);
                let value = value.map_err(|err| Error::Response(Box::new(err)))?;

                headers.append(&name, value);
            }
        }

        let mut body = Vec::new();
        let mut reader = response.into_reader();
        reader
            .read_to_end(&mut body)
            .map_err(|err| Error::Response(Box::new(err)))?;

        Ok(PResponse::new(status, headers, Bytes::from(body)))
    }
}
