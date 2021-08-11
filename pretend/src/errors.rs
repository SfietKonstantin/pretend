use crate::StatusCode;
use std::{error, result};
use thiserror::Error;

macro_rules! impl_error {
    ($ty:ty, $($b:tt)*) => {
        /// Pretend errors
        ///
        /// This error type wraps errors emitted
        /// by `pretend` or by client implementations.
        #[derive(Error, Debug)]
        pub enum Error {
            /// Error when creating a client implementation
            #[error("Failed to create client")]
            Client(#[source] $ty),
            /// Error when building the request
            #[error("Invalid request")]
            Request(#[source] $ty),
            /// Error when executing the request
            #[error("Failed to execute request")]
            Response(#[source] $ty),
            /// Error when parsing the response body
            #[error("Failed to read response body")]
            Body(#[source] $ty),
            /// HTTP status error
            ///
            /// This error is returned when the request failed with
            /// an HTTP error status. It is only returned when methods
            /// returns bodies.
            #[error("HTTP {0}")]
            Status(StatusCode),
        }

        impl Error {
            /// Construct a new `Client` error
            pub fn client<E>(err: E) -> Self
            where
                E: $($b)*,
            {
                Error::Client(Box::new(err))
            }

            /// Construct a new `Request` error
            pub fn request<E>(err: E) -> Self
            where
                E: $($b)*,
            {
                Error::Request(Box::new(err))
            }

            /// Construct a new `Response` error
            pub fn response<E>(err: E) -> Self
            where
                E: $($b)*,
            {
                Error::Response(Box::new(err))
            }

            /// Construct a new `Body` error
            pub fn body<E>(err: E) -> Self
            where
                E: $($b)*,
            {
                Error::Body(Box::new(err))
            }
        }
    };
}

#[cfg(not(feature = "local-error"))]
impl_error!(
    Box<dyn error::Error + 'static + Send + Sync>,
    error::Error + 'static + Send + Sync
);
#[cfg(feature = "local-error")]
impl_error!(Box<dyn error::Error + 'static>, error::Error + 'static);

/// Pretend Result type
pub type Result<T> = result::Result<T, Error>;
