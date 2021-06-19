use crate::StatusCode;
use std::{error, result};
use thiserror::Error;

/// Pretend errors
///
/// This error type wraps errors emitted
/// by `pretend` or by client implementations.
#[derive(Error, Debug)]
pub enum Error {
    /// Error when creating a client implementation
    #[error("Failed to create client")]
    Client(#[source] Box<dyn error::Error + Send + Sync>),
    /// Error when building the request
    #[error("Invalid request")]
    Request(#[source] Box<dyn error::Error + Send + Sync>),
    /// Error when executing the request
    #[error("Failed to execute request")]
    Response(#[source] Box<dyn error::Error + Send + Sync>),
    /// Error when parsing the response body
    #[error("Failed to read response body")]
    Body(#[source] Box<dyn error::Error + Send + Sync>),
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
