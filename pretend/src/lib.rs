//! `pretend` HTTP client
//!
//! `pretend` is a modular, [Feign]-inspired, HTTP client based on macros. It's goal is to decouple
//! the definition of a REST API from it's implementation.
//!
//! Some features:
//! - Declarative
//! - Asynchronous-first implementations
//! - HTTP client agnostic
//! - JSON support thanks to serde
//!
//! [Feign]: https://github.com/OpenFeign/feign
//!
//! # Getting started
//!
//! A REST API is described by annotating a trait:
//!
//! ```rust
//! use pretend::{pretend, request, Result};
//!
//! #[pretend]
//! trait HttpBin {
//!     #[request(method = "POST", path = "/anything")]
//!     async fn post_anything(&self, body: &'static str) -> Result<String>;
//! }
//! ```
//!
//! Under the hood, `pretend` will implement this trait for `Pretend`. An instance of this
//! struct can be constructed by passing a client implementation, and the REST API's base url. In
//! the following example, we are using the [`reqwest`] based client.
//!
//! [`reqwest`]: https://crates.io/crates/pretend-reqwest
//!
//! ```rust
//! use pretend::{Pretend, Url};
//! use pretend_reqwest::Client;
//! # use pretend::{pretend, request, Result};
//! # #[pretend]
//! # trait HttpBin {
//! #     #[request(method = "POST", path = "/anything")]
//! #     async fn post_anything(&self, body: &'static str) -> Result<String>;
//! # }
//!
//! # #[tokio::main]
//! # async fn main() {
//! let client = Client::default();
//! let url = Url::parse("https://httpbin.org").unwrap();
//! let pretend = Pretend::for_client(client).with_url(url);
//! let response = pretend.post_anything("hello").await.unwrap();
//! assert!(response.contains("hello"));
//! # }
//! ```
//!
//! # Sending headers, query parameters and bodies
//!
//! Headers are provided as attributes using `header`.
//!
//! ```rust
//! use pretend::{header, pretend, request, Result};
//!
//! #[pretend]
//! trait HttpBin {
//!     #[request(method = "GET", path = "/get")]
//!     #[header(name = "X-Test-Header-1", value = "abc")]
//!     #[header(name = "X-Test-Header-2", value = "other")]
//!     async fn get_with_headers(&self, value: i32, custom: &str) -> Result<()>;
//! }
//! ```
//!
//! Query parameters and bodies are provided as method parameters. Body type is guessed based on
//! the parameter name:
//!
//! - Parameter `body` will be sent as raw bytes.
//! - Parameter `form` will be serialized as form-encoded using `serde`.
//! - Parameter `json` will be serialized as JSON using `serde`.
//!
//! Query parameter is passed with the `query` parameter. It is also serialized using `serde`.
//!
//! ```rust
//! use pretend::{pretend, request, Json, Result, Serialize};
//!
//! #[derive(Serialize)]
//! struct Data {
//!     value: i32,
//! }
//!
//! #[pretend]
//! trait HttpBin {
//!     #[request(method = "POST", path = "/anything")]
//!     async fn post_bytes(&self, body: Vec<u8>) -> Result<()>;
//!
//!     #[request(method = "POST", path = "/anything")]
//!     async fn post_string(&self, body: &'static str) -> Result<()>;
//!
//!     #[request(method = "POST", path = "/anything")]
//!     async fn post_with_query_params(&self, query: &Data) -> Result<()>;
//!
//!     #[request(method = "POST", path = "/anything")]
//!     async fn post_json(&self, json: &Data) -> Result<()>;
//! }
//! ```
//!
//! # Handling responses
//!
//! `pretend` support a wide range of response types, based on the return type of the method.
//! The body can be returned as a `Vec<u8>`, a string or as JSON by using the [`Json`] wrapper
//! type. The unit type `()` can also be used if the body should be discarded.
//!
//! [`JsonResult`] is also offered as a convenience type. It will deserialize into a value type
//! or an error type depending on the HTTP status code.
//!
//! When retrieving body alone, an HTTP error will cause the method to return an error. It is
//! possible to prevent the method to fail and access the HTTP status code by wrapping these
//! types inside a [`Response`]. This also allows accessing response headers.
//!
//! ```rust
//! use pretend::{pretend, request, Deserialize, Json, JsonResult, Response, Result};
//!
//! #[derive(Deserialize)]
//! struct Data {
//!     value: i32,
//! }
//!
//! #[derive(Deserialize)]
//! struct Error {
//!     error: String,
//! }
//!
//! #[pretend]
//! trait HttpBin {
//!     #[request(method = "POST", path = "/anything")]
//!     async fn read_bytes(&self) -> Result<Vec<u8>>;
//!
//!     #[request(method = "POST", path = "/anything")]
//!     async fn read_string(&self) -> Result<String>;
//!
//!     #[request(method = "POST", path = "/anything")]
//!     async fn read_json(&self) -> Result<Json<Data>>;
//!
//!     #[request(method = "POST", path = "/anything")]
//!     async fn read_json_result(&self) -> Result<JsonResult<Data, Error>>;
//!
//!     #[request(method = "POST", path = "/anything")]
//!     async fn read_status(&self) -> Result<Response<()>>;
//! }
//! ```
//!
//! # Templating
//!
//! Request paths and headers support templating. A value between braces will be replaced by
//! a parameter with the same name. The replacement is done with `format!`, meaning that
//! any type that implement `Display` is supported.
//!
//! ```rust
//! use pretend::{header, pretend, request, Deserialize, Json, Pretend, Result};
//! use pretend_reqwest::Client;
//! use std::collections::HashMap;
//! # use pretend::Url;
//!
//! #[derive(Deserialize)]
//! struct Data {
//!     url: String,
//!     headers: HashMap<String, String>,
//! }
//!
//! #[pretend]
//! trait HttpBin {
//!     #[request(method = "POST", path = "/{path}")]
//!     #[header(name = "X-{header}", value = "{value}$")]
//!     async fn read(&self, path: &str, header: &str, value: i32) -> Result<Json<Data>>;
//! }
//!
//! # #[tokio::main]
//! # async fn main() {
//! let client = Client::default();
//! let url = Url::parse("https://httpbin.org").unwrap();
//! let pretend = Pretend::for_client(client).with_url(url);
//! let response = pretend.read("anything", "My-Header", 123).await.unwrap();
//! let data = response.value();
//! assert_eq!(data.url, "https://httpbin.org/anything");
//! assert_eq!(*data.headers.get("X-My-Header").unwrap(), "123$".to_string());
//! # }
//! ```
//!
//! # Available clients
//!
//! `pretend` can be used with
//!
//! - [`reqwest`](https://crates.io/crates/pretend-reqwest)
//! - [`isahc`](https://crates.io/crates/pretend-isahc)
//!
//! # Implementing a `pretend` HTTP client
//!
//! `pretend` clients wraps HTTP clients from other crates. They allow [`Pretend`] to execute
//! HTTP requests. See the [client] module level documentation for more information about
//! how to implement a client.
//!
//! # URL resolvers
//!
//! `pretend` uses URL resolvers to resolve a full URL from the path in `request`. By default
//! the URL resolver will simply append the path to a base URL. More advanced resolvers can
//! be implemented with the [resolver] module.
//!
//! # Examples
//!
//! Please refer to the examples folder for more examples. There are not many examples yet, but
//! we are working on it !
//!
//! # The future
//!
//! Here is a quick roadmap
//!
//! - Support more clients (awc)
//! - Write more examples
//! - Introduce more attributes to mark method parameters (body, json, params)
//! - Better error reporting
//! - Introduce interceptors

#![warn(missing_docs)]
#![deny(unsafe_code)]

pub mod client;
pub mod internal;
pub mod resolver;

mod errors;

pub use self::errors::{Error, Result};
pub use http;
pub use http::{HeaderMap, StatusCode};
pub use pretend_codegen::{header, pretend, request};
pub use serde;
pub use serde::{Deserialize, Serialize};
pub use url;
pub use url::Url;

use crate::client::Client;
use crate::resolver::{InvalidUrlResolver, ResolveUrl, UrlResolver};
use serde::de::DeserializeOwned;

/// Response type
pub struct Response<T> {
    status: StatusCode,
    headers: HeaderMap,
    body: T,
}

impl<T> Response<T> {
    /// Constructor
    pub fn new(status: StatusCode, headers: HeaderMap, body: T) -> Self {
        Response {
            status,
            headers,
            body,
        }
    }

    /// HTTP status
    pub fn status(&self) -> &StatusCode {
        &self.status
    }

    /// Response headers
    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    /// Response body
    pub fn body(&self) -> &T {
        &self.body
    }

    /// Consume this instance to return the body
    pub fn into_body(self) -> T {
        self.body
    }

    /// Consume this instance to return the status, headers and body
    pub fn into_parts(self) -> (StatusCode, HeaderMap, T) {
        (self.status, self.headers, self.body)
    }
}

/// The pretend HTTP client
///
/// This struct is the entry point for `pretend` clients. It can be constructed with
/// an HTTP client implementation, and `pretend` annotated traits will automatically
/// be implemented by this struct.
///
/// See crate level documentation for more information
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
    /// Constructor
    ///
    /// This constructor takes a client implementation and an URL resolver.
    /// Prefer using [`Pretend::for_client`] and [`Pretend::with_url`].
    pub fn new(client: C, resolver: R) -> Pretend<C, R> {
        Pretend { client, resolver }
    }

    /// Set the base URL
    ///
    /// Set the base URL for this client.
    pub fn with_url(self, url: Url) -> Pretend<C, UrlResolver> {
        self.with_url_resolver(UrlResolver::new(url))
    }

    /// Set the URL resolver
    ///
    /// Set the URL resolver for this client.
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
    /// Constructor
    ///
    /// This constructor takes a client implementation and
    /// return an incomplete `Pretend` client. Use [`Pretend::with_url`] to
    /// set the base URL.
    pub fn for_client(client: C) -> Pretend<C, InvalidUrlResolver> {
        Pretend {
            client,
            resolver: InvalidUrlResolver,
        }
    }
}

/// JSON body
///
/// This wrapper type indicates that a method should return
/// a JSON-serialized body.
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
    /// Deserialized value
    pub fn value(self) -> T {
        self.value
    }
}

impl<T> AsRef<T> for Json<T>
where
    T: DeserializeOwned,
{
    fn as_ref(&self) -> &T {
        &self.value
    }
}

/// JSON result
///
/// This wrapper type indicate that a method should return
/// JSON-serialized bodies.
///
/// When the HTTP request is successful, the `Ok` variant will
/// be returned, and when the HTTP request has failed, the
/// `Err` variant will be returned.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum JsonResult<T, E>
where
    T: DeserializeOwned,
    E: DeserializeOwned,
{
    /// Successful value
    Ok(T),
    /// Error value
    Err(E),
}
