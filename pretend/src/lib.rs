//! `pretend` HTTP client
//!
//! `pretend` is a modular, [Feign]-inspired, HTTP client based on macros. It's goal is to decouple
//! the definition of a REST API from it's implementation.
//!
//! Some features:
//! - Declarative
//! - Support Asynchronous and blocking requests
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
//! use pretend::{pretend, Result};
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
//! # use pretend::{pretend, Result};
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
//! use pretend::{pretend, Result};
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
//! use pretend::{pretend, Json, Result};
//! use serde::Serialize;
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
//! use pretend::{pretend, Json, JsonResult, Response, Result};
//! use serde::Deserialize;
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
//!     async fn get_bytes(&self) -> Result<Vec<u8>>;
//!
//!     #[request(method = "POST", path = "/anything")]
//!     async fn get_string(&self) -> Result<String>;
//!
//!     #[request(method = "POST", path = "/anything")]
//!     async fn get_json(&self) -> Result<Json<Data>>;
//!
//!     #[request(method = "POST", path = "/anything")]
//!     async fn get_json_result(&self) -> Result<JsonResult<Data, Error>>;
//!
//!     #[request(method = "POST", path = "/anything")]
//!     async fn get_status(&self) -> Result<Response<()>>;
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
//! use pretend::{pretend, Json, Pretend, Result, Url};
//! use pretend_reqwest::Client;
//! use serde::Deserialize;
//! use std::collections::HashMap;
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
//!     async fn get(&self, path: &str, header: &str, value: i32) -> Result<Json<Data>>;
//! }
//!
//! # #[tokio::main]
//! # async fn main() {
//! let client = Client::default();
//! let url = Url::parse("https://httpbin.org").unwrap();
//! let pretend = Pretend::for_client(client).with_url(url);
//! let response = pretend.get("anything", "My-Header", 123).await.unwrap();
//! let data = response.value();
//! assert_eq!(data.url, "https://httpbin.org/anything");
//! assert_eq!(*data.headers.get("X-My-Header").unwrap(), "123$".to_string());
//! # }
//! ```
//!
//! # URL resolvers
//!
//! `pretend` uses URL resolvers to resolve a full URL from the path in `request`. By default
//! the URL resolver will simply append the path to a base URL. More advanced resolvers can
//! be implemented with the [resolver] module.
//!
//! # Request interceptors
//!
//! `pretend` uses request interceptors to customize auto-generated requests. They can be useful
//! when dealing with authentication. They can be implemented with the [interceptor] module.
//!
//! ```rust
//! use pretend::http::header::AUTHORIZATION;
//! use pretend::http::HeaderValue;
//! use pretend::interceptor::{InterceptRequest, Request};
//! use pretend::{pretend, Error, Json, Pretend, Result, Url};
//! use pretend_reqwest::Client;
//! use serde::Deserialize;
//! use std::collections::HashMap;
//!
//! #[derive(Deserialize)]
//! struct Data {
//!     url: String,
//!     headers: HashMap<String, String>,
//! }
//!
//! #[pretend]
//! trait HttpBin {
//!     #[request(method = "GET", path = "/get")]
//!     async fn get(&self) -> Result<Json<Data>>;
//! }
//!
//! struct AuthInterceptor {
//!     auth: String,
//! }
//!
//! impl AuthInterceptor {
//!     fn new(auth: String) -> Self {
//!         AuthInterceptor { auth }
//!     }
//! }
//!
//! impl InterceptRequest for AuthInterceptor {
//!     fn intercept(&self, mut request: Request) -> Result<Request> {
//!         // Create the header, reporting failure if the header is invalid
//!         let header = format!("Bearer {}", self.auth);
//!         let header = HeaderValue::from_str(&header).map_err(|err| Error::Request(Box::new(err)))?;
//!
//!         // Set the authorization header in the request
//!         request.headers.append(AUTHORIZATION, header);
//!         Ok(request)
//!     }
//! }
//! # #[tokio::main]
//! # async fn main() {
//! let client = Client::default();
//! let url = Url::parse("https://httpbin.org").unwrap();
//! let auth_interceptor = AuthInterceptor::new("test".to_string());
//! let pretend = Pretend::for_client(client).with_url(url).with_request_interceptor(auth_interceptor);
//! let response = pretend.get().await.unwrap();
//! let data = response.value();
//! assert_eq!(*data.headers.get("Authorization").unwrap(), "Bearer test".to_string());
//! # }
//! ```
//!
//! # Examples
//!
//! More examples are available in the [examples folder].
//!
//! [examples folder]: https://github.com/SfietKonstantin/pretend/tree/main/pretend/examples
//!
//! # Blocking requests
//!
//! When all methods in the `pretend`-annotated trait are async, `pretend` will generate
//! an async implementation. To generate a blocking implementation, simply remove the `async`
//! keyword.
//!
//! Blocking implementations needs a blocking client implementation to be used. In the following
//! example, we are using one provided by [`reqwest`]
//!
//! ```rust
//! use pretend::{pretend, Pretend, Result, Url};
//! use pretend_reqwest::BlockingClient;
//!
//! #[pretend]
//! trait HttpBin {
//!     #[request(method = "POST", path = "/anything")]
//!     fn post_anything(&self, body: &'static str) -> Result<String>;
//! }
//!
//! # fn main() {
//! let client = BlockingClient::default();
//! let url = Url::parse("https://httpbin.org").unwrap();
//! let pretend = Pretend::for_client(client).with_url(url);
//! let response = pretend.post_anything("hello").unwrap();
//! assert!(response.contains("hello"));
//! # }
//! ```
//!
//! [`reqwest`]: https://crates.io/crates/pretend-reqwest
//!
//! # Non-Send implementation
//!
//! Today, Rust does not support futures in traits. `pretend` uses `async_trait` to workaround
//! that limitation. By default, `async_trait` adds the `Send` bound to futures. This implies
//! that `Pretend` itself is `Send` and `Sync`, and implies that the client implementation it uses
//! is also `Send` and `Sync`.
//!
//! However, some clients are not thread-safe, and cannot be shared between threads. To use
//! these clients with `Pretend`, you have to opt-out from the `Send` constraint on returned
//! futures by using `#[pretend(?Send)]`. This is similar to what is done in [`async_trait`].
//!
//! [`async_trait`]: https://docs.rs/async-trait/latest/async_trait/
//!
//! Clients implementations that are not thread-safe are usually called "local clients".
//!
//! # Non-Send errors
//!
//! `pretend` boxes errors returned by the client in [`Error`]. By default, it requires the error
//! to be `Send + Sync`. For some clients, especially local ones, this bound cannot be guaranteed.
//!
//! `pretend` offers the feature `local-error` as an escape latch. When enabled, this feature will
//! drop the `Send + Sync` bound on boxed errors. This feature is enabled by default for
//! `pretend-awc`, a local client that returns non-Send errors.
//!
//! # Available client implementations
//!
//! `pretend` can be used with the following HTTP clients
//!
//! - [`reqwest`](https://crates.io/crates/pretend-reqwest) (async and blocking)
//! - [`isahc`](https://crates.io/crates/pretend-isahc) (async)
//! - [`awc`](https://crates.io/crates/pretend-awc) (local async)
//! - [`ureq`](https://crates.io/crates/pretend-ureq) (blocking)
//!
//! These client implementations depends on the latest major release of each HTTP client at
//! time of the release. The `default` feature for each of the HTTP client crate is also mapped
//! to the `pretend-*` crate. To enable HTTP client features, you should add it as a dependency
//! and enable them here. If needed, you can play with the `default-features` option on the
//! `pretend-*` crate.
//!
//! The following snippet will enable `reqwest` default features
//!
//! ```toml
//! [dependencies]
//! pretend-reqwest = "0.2.2"
//! ```
//!
//! In the following snippet, no feature of `reqwest` will be enabled
//!
//! ```toml
//! [dependencies]
//! pretend-reqwest = { version = "0.2.2", default-features = false }
//! ```
//!
//! To use `reqwest` with rustls instead of the native-tls, you can do the following:
//!
//! ```toml
//! [dependencies]
//! pretend-reqwest = { version = "0.2.2", default-features = false }
//! reqwest = { version = "*", default-features = false, features = ["rustls-tls"] }
//! ```
//!
//! # Implementing a `pretend` HTTP client
//!
//! `pretend` clients wraps HTTP clients from other crates. They allow [`Pretend`] to execute
//! HTTP requests. See the [client] module level documentation for more information about
//! how to implement a client.
//!
//! # MSRV
//!
//! MSRV for the `pretend` ecosystem is Rust **1.44**.
//!
//! # The future
//!
//! Here is a quick roadmap
//!
//! - Introduce more attributes to mark method parameters (body, json, params)

#![warn(missing_docs)]
#![forbid(unsafe_code)]

pub mod client;
pub mod interceptor;
pub mod internal;
pub mod resolver;

mod errors;

pub use self::errors::{Error, Result};
pub use http;
pub use http::{HeaderMap, StatusCode};
pub use pretend_codegen::pretend;
pub use serde;
pub use url;
pub use url::Url;

use crate::interceptor::{InterceptRequest, NoopRequestInterceptor};
use crate::resolver::{InvalidUrlResolver, ResolveUrl, UrlResolver};
use serde::de::DeserializeOwned;
use std::ops::{Deref, DerefMut};

/// Response type
#[derive(Clone, Debug, Eq, PartialEq)]
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
#[derive(Clone, Debug)]
pub struct Pretend<C, R, I>
where
    R: ResolveUrl,
    I: InterceptRequest,
{
    client: C,
    resolver: R,
    interceptor: I,
}

impl<C, R, I> Pretend<C, R, I>
where
    R: ResolveUrl,
    I: InterceptRequest,
{
    /// Constructor
    ///
    /// This constructor takes a client implementation, an URL resolver and
    /// an interceptor. Prefer using [`Pretend::for_client`] and [`Pretend::with_url`].
    pub fn new(client: C, resolver: R, interceptor: I) -> Pretend<C, R, I> {
        Pretend {
            client,
            resolver,
            interceptor,
        }
    }

    /// Set the base URL
    ///
    /// Set the base URL for this client.
    pub fn with_url(self, url: Url) -> Pretend<C, UrlResolver, I> {
        self.with_url_resolver(UrlResolver::new(url))
    }

    /// Set the request interceptor
    ///
    /// Set the request interceptor for this client.
    pub fn with_request_interceptor<II>(self, interceptor: II) -> Pretend<C, R, II>
    where
        II: InterceptRequest,
    {
        Pretend::new(self.client, self.resolver, interceptor)
    }

    /// Set the URL resolver
    ///
    /// Set the URL resolver for this client.
    pub fn with_url_resolver<RR>(self, resolver: RR) -> Pretend<C, RR, I>
    where
        RR: ResolveUrl,
    {
        Pretend::new(self.client, resolver, self.interceptor)
    }
}

impl<C> Pretend<C, InvalidUrlResolver, NoopRequestInterceptor> {
    /// Constructor
    ///
    /// This constructor takes a client implementation and
    /// return an incomplete `Pretend` client. Use [`Pretend::with_url`] to
    /// set the base URL.
    pub fn for_client(client: C) -> Pretend<C, InvalidUrlResolver, NoopRequestInterceptor> {
        Pretend::new(client, InvalidUrlResolver, NoopRequestInterceptor)
    }
}

/// JSON body
///
/// This wrapper type indicates that a method should return
/// a JSON-serialized body.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
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

impl<T> AsMut<T> for Json<T>
where
    T: DeserializeOwned,
{
    fn as_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T> Deref for Json<T>
where
    T: DeserializeOwned,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for Json<T>
where
    T: DeserializeOwned,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
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
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
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
