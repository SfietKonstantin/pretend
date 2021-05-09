//! `pretend` HTTP client
//!
//! # Examples
//!
//! ```rust
//! use pretend::{pretend, request, Pretend, Result, Url, UrlResolver};
//! # /*
//! use pretend_reqwest::Client;
//! # */
//! # use pretend_testclient::Client;
//! # use pretend::{HeaderMap, Response, StatusCode};
//!
//! #[pretend]
//! trait RestApi {
//!     #[request(method = "GET", path = "/get/{value}")]
//!     async fn api_get(&self, value: &str) -> Result<String>;
//! }
//!
//! #[tokio::main]
//! async fn main() {
//! # /*
//!     let client = Client::default();
//! # */
//! #    let client = Client::new(|_, _, _, _| {
//! #        let body = "Hello world";
//! #        let body = Some(body.as_bytes().to_vec());
//! #        Ok(Response::new(StatusCode::OK, HeaderMap::new(), body))
//! #    });
//!     let url = Url::parse("http://localhost").unwrap();
//!     let pretend = Pretend::for_client(client).with_url(url);
//!     let response = pretend.api_get("some-value").await.unwrap();
//!     assert_eq!(response, "Hello world");
//! }
//!
//! ```

// #[warn(missing_docs)]

pub mod client;
pub mod internal;
pub mod thirdparty;

mod errors;

pub use self::errors::{Error, Result};
pub use pretend_codegen::{header, pretend, request};
pub use thirdparty::async_trait::async_trait;
pub use thirdparty::http::{HeaderMap, Method, StatusCode};
pub use thirdparty::serde::de::DeserializeOwned;
pub use thirdparty::serde::{Deserialize, Serialize};
pub use thirdparty::url::{ParseError, Url};

use crate::client::Client;
use std::result;

pub struct Response<T> {
    status: StatusCode,
    headers: HeaderMap,
    body: T,
}

impl<T> Response<T> {
    pub fn new(status: StatusCode, headers: HeaderMap, body: T) -> Self {
        Response {
            status,
            headers,
            body,
        }
    }

    pub fn status(&self) -> &StatusCode {
        &self.status
    }

    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    pub fn body(&self) -> &T {
        &self.body
    }
}

pub trait ResolveUrl {
    fn resolve_url(&self, path: &str) -> result::Result<Url, ParseError>;
}

pub struct UrlResolver {
    base: Url,
}

impl UrlResolver {
    pub fn new(base: Url) -> Self {
        UrlResolver { base }
    }
}

impl ResolveUrl for UrlResolver {
    fn resolve_url(&self, path: &str) -> result::Result<Url, ParseError> {
        self.base.join(path)
    }
}

pub struct InvalidUrlResolver;

impl ResolveUrl for InvalidUrlResolver {
    fn resolve_url(&self, _: &str) -> result::Result<Url, ParseError> {
        Err(ParseError::EmptyHost)
    }
}

pub struct Pretend<C, R>
where
    C: Client + Send + Sync,
    R: ResolveUrl + Send + Sync,
{
    client: C,
    resolver: R,
}

impl<C, R> Pretend<C, R>
where
    C: Client + Send + Sync,
    R: ResolveUrl + Send + Sync,
{
    pub fn new(client: C, resolver: R) -> Pretend<C, R> {
        Pretend { client, resolver }
    }

    pub fn with_url(self, url: Url) -> Pretend<C, UrlResolver> {
        self.with_url_resolver(UrlResolver::new(url))
    }

    pub fn with_url_resolver<RR>(self, resolver: RR) -> Pretend<C, RR>
    where
        RR: ResolveUrl + Send + Sync,
    {
        Pretend {
            client: self.client,
            resolver,
        }
    }
}

impl<C> Pretend<C, InvalidUrlResolver>
where
    C: Client + Send + Sync,
{
    pub fn for_client(client: C) -> Pretend<C, InvalidUrlResolver> {
        Pretend {
            client,
            resolver: InvalidUrlResolver,
        }
    }
}

pub struct Json<T>
where
    T: DeserializeOwned,
{
    value: T,
}

impl<T> Json<T>
where
    T: DeserializeOwned,
{
    pub fn value(self) -> T {
        self.value
    }
}

pub enum JsonResult<T, E>
where
    T: DeserializeOwned,
    E: DeserializeOwned,
{
    Ok(T),
    Err(E),
}
