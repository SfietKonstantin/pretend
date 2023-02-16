//! Request interceptors
//!
//! `pretend` support request interceptors. They are used
//! to post-process an automatically generated request before
//! it is being executed.
//!
//! Interceptors can facilitate setting common headers, like
//! an user-agent, or injecting an authentication token.
//!
//! By default a [`NoopRequestInterceptor`] is used, and will
//! not modify the request.
//!
//! Custom interceptors are defined by implementing [`InterceptRequest`].
//!
//! # Error handling
//!
//! Request interceptors are allowed to fail. They return a pretend
//! `Result` that will be returned as is to the caller. Prefer using
//! `Error::Request` when reporting an error.

pub use crate::client::Method;
pub use crate::{HeaderMap, Result, Url};

/// An HTTP request
#[non_exhaustive]
pub struct Request {
    /// Request method
    pub method: Method,
    /// Full request URL
    pub url: Url,
    /// Request headers
    pub headers: HeaderMap,
}

impl Request {
    /// Constructor
    ///
    /// This constructor can be used
    /// to construct customized requests
    /// inside a request interceptor.
    pub fn new(method: Method, url: Url, headers: HeaderMap) -> Self {
        Request {
            method,
            url,
            headers,
        }
    }
}

/// Describe a request interceptor
///
/// See module level documentation for more information.
pub trait InterceptRequest {
    /// Intercept a request, returning a customized request
    fn intercept(&self, request: Request) -> Result<Request>;
}

/// Default request interceptor
///
/// This request interceptor will not modify
/// the request.
#[derive(Clone, Copy, Debug, Default)]
pub struct NoopRequestInterceptor;

impl InterceptRequest for NoopRequestInterceptor {
    fn intercept(&self, request: Request) -> Result<Request> {
        Ok(request)
    }
}
