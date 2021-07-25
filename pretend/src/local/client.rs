use super::Result;
use crate::{client, HeaderMap, Response, Url};
use async_trait::async_trait;
use bytes::Bytes;
use http::Method;

/// `pretend` local client
///
/// See module level documentation for more information.
#[async_trait(?Send)]
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

#[async_trait(?Send)]
impl<C> Client for C
where
    C: client::Client,
{
    async fn execute(
        &self,
        method: Method,
        url: Url,
        headers: HeaderMap,
        body: Option<Bytes>,
    ) -> Result<Response<Bytes>> {
        let response = C::execute(self, method, url, headers, body).await?;
        Ok(response)
    }
}
