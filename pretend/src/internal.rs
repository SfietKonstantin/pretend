#![doc(hidden)]

//! Internal classes used by the macro

use crate::client::{Client, RequestBuilder, ResponseBody};
use crate::{
    async_trait, DeserializeOwned, Error, HeaderMap, Json, JsonResult, Method, Pretend, ResolveUrl,
    Response, Result,
};
use http::header::HeaderName;
use http::HeaderValue;
use std::str::FromStr;

pub struct MacroSupport<'a, C, R>
where
    C: Client + Send + Sync,
    R: ResolveUrl + Send + Sync,
{
    pretend: &'a Pretend<C, R>,
}

impl<'a, C, R> MacroSupport<'a, C, R>
where
    C: Client + Send + Sync,
    R: ResolveUrl + Send + Sync,
{
    pub fn new(pretend: &'a Pretend<C, R>) -> Self {
        MacroSupport { pretend }
    }

    pub fn request(
        &self,
        method: Method,
        path: &str,
        headers: HeaderMap,
    ) -> Result<C::RequestBuilder> {
        let resolver = &self.pretend.resolver;
        let url = resolver
            .resolve_url(path)
            .map_err(|err| Error::Request(Box::new(err)))?;

        let client = &self.pretend.client;
        client.request_builder(method, url)?.headers(headers)
    }

    pub async fn execute(&self, request: C::Request) -> Result<Response<C::ResponseBody>> {
        let result = self.pretend.client.execute(request).await;
        result
    }
}

pub trait IntoBody {
    fn into_body(self) -> Vec<u8>;
}

impl IntoBody for String {
    fn into_body(self) -> Vec<u8> {
        self.into_bytes()
    }
}

impl<'a> IntoBody for &'a str {
    fn into_body(self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl IntoBody for Vec<u8> {
    fn into_body(self) -> Vec<u8> {
        self
    }
}

impl<'a> IntoBody for &'a [u8] {
    fn into_body(self) -> Vec<u8> {
        self.to_vec()
    }
}

#[async_trait]
pub trait MacroResponseSupport<T> {
    async fn into_response(self) -> Result<T>;
}

#[async_trait]
impl<B> MacroResponseSupport<String> for Response<B>
where
    B: ResponseBody + Send,
{
    async fn into_response(self) -> Result<String> {
        if self.status.is_success() {
            self.body.text().await
        } else {
            Err(Error::Status(self.status))
        }
    }
}

#[async_trait]
impl<B> MacroResponseSupport<Response<String>> for Response<B>
where
    B: ResponseBody + Send,
{
    async fn into_response(self) -> Result<Response<String>> {
        let body = self.body.text().await?;
        Ok(Response::new(self.status, self.headers, body))
    }
}

#[async_trait]
impl<B> MacroResponseSupport<Vec<u8>> for Response<B>
where
    B: ResponseBody + Send,
{
    async fn into_response(self) -> Result<Vec<u8>> {
        if self.status.is_success() {
            self.body.bytes().await
        } else {
            Err(Error::Status(self.status))
        }
    }
}

#[async_trait]
impl<B> MacroResponseSupport<Response<Vec<u8>>> for Response<B>
where
    B: ResponseBody + Send,
{
    async fn into_response(self) -> Result<Response<Vec<u8>>> {
        let body = self.body.bytes().await?;
        Ok(Response::new(self.status, self.headers, body))
    }
}

#[async_trait]
impl<B, T> MacroResponseSupport<Json<T>> for Response<B>
where
    B: ResponseBody + Send,
    T: DeserializeOwned,
{
    async fn into_response(self) -> Result<Json<T>> {
        if self.status.is_success() {
            let value = self.body.json::<T>().await?;
            Ok(Json { value })
        } else {
            Err(Error::Status(self.status))
        }
    }
}

#[async_trait]
impl<B, T> MacroResponseSupport<Response<Json<T>>> for Response<B>
where
    B: ResponseBody + Send,
    T: DeserializeOwned,
{
    async fn into_response(self) -> Result<Response<Json<T>>> {
        let value = self.body.json::<T>().await?;
        let body = Json { value };
        Ok(Response::new(self.status, self.headers, body))
    }
}

#[async_trait]
impl<B, T, E> MacroResponseSupport<JsonResult<T, E>> for Response<B>
where
    B: ResponseBody + Send,
    T: DeserializeOwned,
    E: DeserializeOwned,
{
    async fn into_response(self) -> Result<JsonResult<T, E>> {
        if self.status.is_success() {
            let value = self.body.json::<T>().await?;
            Ok(JsonResult::Ok(value))
        } else {
            let value = self.body.json::<E>().await?;
            Ok(JsonResult::Err(value))
        }
    }
}

#[async_trait]
impl<B, T, E> MacroResponseSupport<Response<JsonResult<T, E>>> for Response<B>
where
    B: ResponseBody + Send,
    T: DeserializeOwned,
    E: DeserializeOwned,
{
    async fn into_response(self) -> Result<Response<JsonResult<T, E>>> {
        if self.status.is_success() {
            let value = self.body.json::<T>().await?;
            Ok(Response::new(
                self.status,
                self.headers,
                JsonResult::Ok(value),
            ))
        } else {
            let value = self.body.json::<E>().await?;
            Ok(Response::new(
                self.status,
                self.headers,
                JsonResult::Err(value),
            ))
        }
    }
}

#[async_trait]
impl<B> MacroResponseSupport<Response<()>> for Response<B>
where
    B: ResponseBody + Send,
{
    async fn into_response(self) -> Result<Response<()>> {
        Ok(Response::new(self.status, self.headers, ()))
    }
}

pub fn build_header(headers: &mut HeaderMap, name: &str, value: &str) -> Result<()> {
    let name = HeaderName::from_str(name).map_err(|err| Error::Request(Box::new(err)))?;
    let value = HeaderValue::from_str(value).map_err(|err| Error::Request(Box::new(err)))?;
    headers.append(name, value);
    Ok(())
}
