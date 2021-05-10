use crate::StatusCode;
use std::{error, result};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to create client")]
    Client(#[source] Box<dyn error::Error>),
    #[error("Invalid request")]
    Request(#[source] Box<dyn error::Error>),
    #[error("Failed to execute request")]
    Response(#[source] Box<dyn error::Error>),
    #[error("Failed to read response body")]
    Body(#[source] Box<dyn error::Error>),
    #[error("HTTP {0}")]
    Status(StatusCode),
}

pub type Result<T> = result::Result<T, Error>;
