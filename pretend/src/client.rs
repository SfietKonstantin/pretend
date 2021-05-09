//! Client SPI
//!
//! This module contains traits that can be implemented by a `pretend` client.

use crate::{async_trait, DeserializeOwned, HeaderMap, Method, Response, Result, Serialize, Url};

pub trait RequestBuilder: Sized {
    type Request;
    fn headers(self, headers: HeaderMap) -> Result<Self>;
    fn body(self, body: Vec<u8>) -> Result<Self>;
    fn query<T>(self, query: &T) -> Result<Self>
    where
        T: Serialize;
    fn form<T>(self, form: &T) -> Result<Self>
    where
        T: Serialize;
    fn json<T>(self, json: &T) -> Result<Self>
    where
        T: Serialize;
    fn build(self) -> Self::Request;
}

#[async_trait]
pub trait ResponseBody {
    async fn text(self) -> Result<String>;
    async fn bytes(self) -> Result<Vec<u8>>;
    async fn json<T>(self) -> Result<T>
    where
        T: DeserializeOwned;
}

#[async_trait]
pub trait Client {
    type Request: Send;
    type RequestBuilder: RequestBuilder<Request = Self::Request> + Send;
    type ResponseBody: ResponseBody + Send;

    fn request_builder(&self, method: Method, url: Url) -> Result<Self::RequestBuilder>;
    async fn execute(&self, request: Self::Request) -> Result<Response<Self::ResponseBody>>;
}
