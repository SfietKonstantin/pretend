//! Client traits
//!
//! This module contains traits to be implemented by a `pretend` client. `pretend` clients
//! implementations are responsible of executing the actual HTTP requests. Implementations
//! delegates to other crates.
//!
//! The central trait to implement is [`Client`]. This trait wraps the underlying
//! HTTP client and is responsible of executing the request via [`Client::execute`].
//! This method takes a method, url, header and body (as raw bytes) and should return
//! a response with raw bytes as body.
//!
//! Implementations should be marked with [`client::async_trait`] due to Rust limitations
//! with futures and traits.

pub use async_trait::async_trait;
pub use bytes::Bytes;
pub use http::Method;

use crate::{HeaderMap, Response, Result, Url};

/// `pretend` Client
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
