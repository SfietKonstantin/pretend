use crate::StatusCode;
use std::{error, result};
use thiserror::Error;

/// Pretend errors for local clients
///
/// This error type wraps errors emitted
/// by `pretend` or by client implementations.
/// It does not require wrapped errors to be
/// Send and Sync, and should be used with local
/// clients.
#[derive(Error, Debug)]
pub enum Error {
    /// Error when creating a client implementation
    #[error("Failed to create client")]
    Client(#[source] Box<dyn error::Error + 'static>),
    /// Error when building the request
    #[error("Invalid request")]
    Request(#[source] Box<dyn error::Error + 'static>),
    /// Error when executing the request
    #[error("Failed to execute request")]
    Response(#[source] Box<dyn error::Error + 'static>),
    /// Error when parsing the response body
    #[error("Failed to read response body")]
    Body(#[source] Box<dyn error::Error + 'static>),
    /// HTTP status error
    ///
    /// This error is returned when the request failed with
    /// an HTTP error status. It is only return when methods
    /// returns bodies.
    #[error("HTTP {0}")]
    Status(StatusCode),
}

/// Pretend Result type
pub type Result<T> = result::Result<T, Error>;

impl From<crate::Error> for Error {
    fn from(err: crate::errors::Error) -> Self {
        match err {
            crate::Error::Client(err) => Error::Client(err),
            crate::Error::Request(err) => Error::Request(err),
            crate::Error::Response(err) => Error::Response(err),
            crate::Error::Body(err) => Error::Body(err),
            crate::Error::Status(status) => Error::Status(status),
        }
    }
}
