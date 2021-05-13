use std::iter::FromIterator;
use syn::{Error, Result};

pub(crate) const INVALID_CLIENT_KIND: &str =
    "Expected `#[pretend]`, `#[pretend(blocking)]` or `#[pretend(non_send)]`";
pub(crate) const CODEGEN_FAILURE: &str = "Failed to generate pretend implementation";
pub(crate) const METHOD_FAILURE: &str = "Failed to generate method implementation";
pub(crate) const UNSUPPORTED_TRAIT_ITEM: &str = "Only methods are supported";
pub(crate) const UNSUPPORTED_GENERICS: &str = "Generics are not supported";
pub(crate) const MUST_NOT_ASYNC: &str = "Function must not be async for blocking client";
pub(crate) const MUST_ASYNC: &str = "Function must be async";
pub(crate) const UNSUPPORTED_RECEIVER: &str = "Method must take `&self` as receiver";
pub(crate) const MISSING_REQUEST: &str = "Method must have the `#[request]` attribute";
pub(crate) const TOO_MANY_REQUESTS: &str = "Method must have the `#[request]` attribute only once";
pub(crate) const TOO_MANY_REQUESTS_HINT: &str = "`#[request]` attribute defined here";
pub(crate) const INVALID_REQUEST: &str =
    "`#[request]` attribute must only have `method` and `path`";
pub(crate) const TOO_MANY_BODIES: &str = "Method can only have at most one body parameter";
pub(crate) const TOO_MANY_BODIES_HINT: &str = "Body parameter defined here";
pub(crate) const INVALID_HEADER: &str = "`#[header]` attribute must only have `name` and `value`";

pub(crate) struct Report<T> {
    values: Vec<T>,
    errors: Vec<Error>,
}

pub(crate) trait ErrorsExt {
    fn into_result<T, F>(self, new_err: F) -> Result<T>
    where
        F: FnOnce() -> Error;
}

impl<T> Report<T> {
    pub(crate) fn into_result<F>(self, new_err: F) -> Result<Vec<T>>
    where
        F: FnOnce() -> Error,
    {
        if self.errors.is_empty() {
            Ok(self.values)
        } else {
            self.errors.into_result(new_err)
        }
    }
}

impl<T> FromIterator<Result<T>> for Report<T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Result<T>>,
    {
        let mut values = Vec::new();
        let mut errors = Vec::new();

        for item in iter {
            match item {
                Ok(item) => values.push(item),
                Err(err) => errors.push(err),
            }
        }

        Report { values, errors }
    }
}

impl ErrorsExt for Vec<Error> {
    fn into_result<T, F>(self, new_err: F) -> Result<T>
    where
        F: FnOnce() -> Error,
    {
        let mut errors = new_err();
        for err in self {
            errors.combine(err)
        }
        Err(errors)
    }
}
