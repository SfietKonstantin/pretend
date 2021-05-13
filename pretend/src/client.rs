//! Client traits
//!
//! This module contains traits to be implemented by a `pretend` client. `pretend` clients
//! implementations are responsible of executing the actual HTTP requests. Implementations
//! delegates to other crates.
//!
//! `pretend` support 3 kind of clients:
//!
//! - [`Client`] for asynchronous clients
//! - [`LocalClient`] for asynchronous local clients (clients that are not `Send` + `Sync`)
//! - [`BlockingClient`] for blocking clients
//!
//! These traits wrap the underlying HTTP client and are responsible of executing the request
//! via `execute`. This method takes a method, url, header and body (as raw bytes) and should
//! return a response with raw bytes as body.
//!
//! Since this crate uses `async_trait` to support futures in trait, `Client`
//! implementations should be marked with `#[client::async_trait]` and
//! `LocalClient` should use `#[client::async_trait(?Send)]`.

pub use async_trait::async_trait;
pub use bytes::Bytes;
pub use http::Method;

use crate::{HeaderMap, Response, Result, Url};

/// `pretend` client
///
/// See module level documentation for more information.
#[async_trait]
pub trait Client {
    /// Execute a request
    async fn execute(
        &self,
        method: Method,
        url: Url,
        headers: HeaderMap,
        body: Option<Bytes>,
    ) -> Result<Response<Bytes>>;
}

/// `pretend` local client
///
/// See module level documentation for more information.
#[async_trait(?Send)]
pub trait LocalClient {
    /// Execute a request
    async fn execute(
        &self,
        method: Method,
        url: Url,
        headers: HeaderMap,
        body: Option<Bytes>,
    ) -> Result<Response<Bytes>>;
}

/// `pretend` blocking client
///
/// See module level documentation for more information.
pub trait BlockingClient {
    /// Execute a request
    fn execute(
        &self,
        method: Method,
        url: Url,
        headers: HeaderMap,
        body: Option<Bytes>,
    ) -> Result<Response<Bytes>>;
}

/// `pretend` local client
///
/// See module level documentation for more information.
#[async_trait(?Send)]
impl<C> LocalClient for C
where
    C: Client,
{
    async fn execute(
        &self,
        method: Method,
        url: Url,
        headers: HeaderMap,
        body: Option<Bytes>,
    ) -> Result<Response<Bytes>> {
        Client::execute(self, method, url, headers, body).await
    }
}
