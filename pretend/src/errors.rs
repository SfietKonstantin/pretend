use crate::{Method, StatusCode, Url};
use std::{error, result};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid request")]
    Request(#[source] Box<dyn error::Error>),
    #[error("Failed to call {method} {url}")]
    Response {
        method: Method,
        url: Url,
        source: Box<dyn error::Error>,
    },
    #[error("Failed to read response body")]
    Body(#[source] Box<dyn error::Error>),
    #[error("HTTP {0}")]
    Status(StatusCode),
}

pub type Result<T> = result::Result<T, Error>;
