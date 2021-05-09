use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum IError {
    #[error("Only methods are supported")]
    UnsupportedTraitItem,
    #[error("Generics in `{0}` are not supported")]
    UnsupportedGenerics(String),
    #[error("`{0}` must take `&self` as receiver")]
    UnsupportedReceiver(String),
    #[error("`{0}` must have the attribute `#[request(method=\"...\", path=\"...\"]`")]
    MissingRequest(String),
    #[error("`{0}` can have the attribute `#[request]` only once")]
    TooManyRequests(String),
    #[error("`{0}` must have a `method`")]
    MissingMethod(String),
    #[error("`{0}` must have a `path`")]
    MissingPath(String),
    #[error("`{0}` cannot have both `form` and `json` as input parameters")]
    TooManyBodies(String),
}

pub(crate) type IResult<T> = Result<T, IError>;
