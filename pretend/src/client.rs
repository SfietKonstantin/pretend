//! Client traits
//!
//! This module contains traits to be implemented by a `pretend` client. `pretend` clients
//! implementations are responsible of executing the actual HTTP requests. Implementations
//! delegates to other crates.
//!
//! `pretend` support 3 kind of clients, [`Client`] for asynchronous clients that creates
//! Futures that are `Send`, [`NonSendClient`] for asynchronous clients that creates
//! Futures that are not `Send` and [`BlockingClient`] for blocking clients.
//!
//! These traits wrap the underlying HTTP client and are responsible of executing the request
//! via `execute`. This method takes a method, url, header and body (as raw bytes) and should
//! return a response with raw bytes as body.
//!
//! Today, Rust do not support futures in traits. This crate is using `async_trait` to workaround
//! that limitation. This also create a split between the trait creating futures that are `Send`
//! and the one creating futures that are not.
//!
//! To make asynchronous client implementations compile, they should be marked with
//! [`client::async_trait`](`self::async_trait`). For [`NonSendClient`],
//! `#[client::async_trait(?Send)]` should be used.

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

/// `pretend` non-send client
///
/// See module level documentation for more information.
#[async_trait(?Send)]
pub trait NonSendClient {
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

#[async_trait(?Send)]
impl<C> NonSendClient for C
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
