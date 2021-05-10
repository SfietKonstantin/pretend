//! Client SPI
//!
//! This module contains traits that can be implemented by a `pretend` client.

pub use async_trait::async_trait;
pub use bytes::Bytes;
pub use http::Method;

use crate::{HeaderMap, Response, Result, Url};

#[async_trait]
pub trait Client {
    async fn execute(
        &self,
        method: Method,
        url: Url,
        headers: HeaderMap,
        body: Option<Bytes>,
    ) -> Result<Response<Bytes>>;
}
