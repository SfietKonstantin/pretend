#![doc(hidden)]

//! Internal module used by the code generator

use crate::client::{BlockingClient, Bytes, Client, LocalClient, Method};
use crate::interceptor::{InterceptRequest, Request};
use crate::resolver::ResolveUrl;
use crate::{Error, HeaderMap, Json, JsonResult, Pretend, Response, Result};
use http::header::{HeaderName, CONTENT_TYPE};
use http::HeaderValue;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::str::FromStr;
use url::Url;

/// Request body
pub enum Body<'a, T>
where
    T: Serialize,
{
    /// No body
    None,
    /// Raw bytes
    Raw(Bytes),
    /// Form
    Form(&'a T),
    /// Json
    Json(&'a T),
}

/// Helper for pretend code generator
pub struct MacroSupport<'p, C, R, I>
where
    R: ResolveUrl,
    I: InterceptRequest,
{
    pretend: &'p Pretend<C, R, I>,
}

impl<'p, C, R, I> MacroSupport<'p, C, R, I>
where
    R: ResolveUrl,
    I: InterceptRequest,
{
    /// Constructor
    ///
    /// It wraps a `Pretend` instance
    pub fn new(pretend: &'p Pretend<C, R, I>) -> Self {
        MacroSupport { pretend }
    }

    /// Create an url from the resolver and a path
    pub fn create_url(&self, path: &str) -> Result<Url> {
        let resolver = &self.pretend.resolver;
        let result = resolver.resolve_url(path);
        result.map_err(Error::request)
    }

    /// Execute a request
    ///
    /// Execute a request from request components.
    /// Serialize the body if needed.
    pub async fn request<'a, T>(
        &'a self,
        method: Method,
        url: Url,
        headers: HeaderMap,
        body: Body<'a, T>,
    ) -> Result<Response<Bytes>>
    where
        C: Client,
        T: Serialize,
    {
        let client = &self.pretend.client;
        let (method, url, headers, body) = self.prepare_request(method, url, headers, body)?;
        client.execute(method, url, headers, body).await
    }

    /// Execute a request on a local client
    ///
    /// Execute a request from request components.
    /// Serialize the body if needed.
    pub async fn request_local<'a, T>(
        &'a self,
        method: Method,
        url: Url,
        headers: HeaderMap,
        body: Body<'a, T>,
    ) -> Result<Response<Bytes>>
    where
        C: LocalClient,
        T: Serialize,
    {
        let client = &self.pretend.client;
        let (method, url, headers, body) = self.prepare_request(method, url, headers, body)?;
        client.execute(method, url, headers, body).await
    }

    /// Execute a blocking request
    ///
    /// Execute a request from request components.
    /// Serialize the body if needed.
    pub fn request_blocking<'a, T>(
        &'a self,
        method: Method,
        url: Url,
        headers: HeaderMap,
        body: Body<'a, T>,
    ) -> Result<Response<Bytes>>
    where
        C: BlockingClient,
        T: Serialize,
    {
        let client = &self.pretend.client;
        let (method, url, headers, body) = self.prepare_request(method, url, headers, body)?;
        client.execute(method, url, headers, body)
    }

    fn prepare_request<'a, T>(
        &'a self,
        method: Method,
        url: Url,
        mut headers: HeaderMap,
        body: Body<'a, T>,
    ) -> Result<(Method, Url, HeaderMap, Option<Bytes>)>
    where
        T: Serialize,
    {
        let (headers, body) = match body {
            Body::None => (headers, None),
            Body::Raw(raw) => (headers, Some(raw)),
            Body::Form(form) => {
                let content_type = HeaderValue::from_static("application/x-www-form-urlencoded");
                headers.insert(CONTENT_TYPE, content_type);

                let encoded = serde_urlencoded::to_string(form);
                let encoded = encoded.map_err(Error::request)?;
                let body = Some(Bytes::from(encoded));

                (headers, body)
            }
            Body::Json(json) => {
                let content_type = HeaderValue::from_static("application/json");
                headers.insert(CONTENT_TYPE, content_type);

                let encoded = serde_json::to_vec(json);
                let encoded = encoded.map_err(Error::request)?;
                let body = Some(Bytes::from(encoded));

                (headers, body)
            }
        };

        let request = Request::new(method, url, headers);
        let request = self.pretend.interceptor.intercept(request)?;
        Ok((request.method, request.url, request.headers, body))
    }
}

/// Update the query component of an Url
pub fn build_query<T>(mut url: Url, query: &T) -> Result<Url>
where
    T: Serialize,
{
    {
        let mut pairs = url.query_pairs_mut();
        let serializer = serde_urlencoded::Serializer::new(&mut pairs);
        let result = query.serialize(serializer);
        result.map_err(Error::request)?;
    }
    Ok(url)
}

/// Append a component to a header
pub fn build_header(headers: &mut HeaderMap, name: &str, value: &str) -> Result<()> {
    let name = HeaderName::from_str(name).map_err(Error::request)?;
    let value = HeaderValue::from_str(value).map_err(Error::request)?;
    headers.append(name, value);
    Ok(())
}

/// Trait to convert into a response
///
/// This trait is responsible to convert a raw body
/// into a response. Implementations of these traits
/// handle raw bytes, strings, JSON and responses
pub trait IntoResponse<T> {
    fn into_response(self) -> Result<T>;
}

impl IntoResponse<()> for Response<Bytes> {
    fn into_response(self) -> Result<()> {
        if self.status.is_success() {
            Ok(())
        } else {
            Err(Error::Status(self.status))
        }
    }
}

impl IntoResponse<Response<()>> for Response<Bytes> {
    fn into_response(self) -> Result<Response<()>> {
        let (status, headers, _) = self.into_parts();
        Ok(Response::new(status, headers, ()))
    }
}

impl IntoResponse<String> for Response<Bytes> {
    fn into_response(self) -> Result<String> {
        if self.status.is_success() {
            Ok(parse_string_body(&self))
        } else {
            Err(Error::Status(self.status))
        }
    }
}

impl IntoResponse<Response<String>> for Response<Bytes> {
    fn into_response(self) -> Result<Response<String>> {
        let body = parse_string_body(&self);
        Ok(Response::new(self.status, self.headers, body))
    }
}

fn parse_string_body(response: &Response<Bytes>) -> String {
    // Taken from reqwest
    let content_type = response.headers.get(CONTENT_TYPE);
    let content_type = content_type
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<mime::Mime>().ok());
    let encoding_name = content_type
        .as_ref()
        .and_then(|mime| mime.get_param("charset").map(|charset| charset.as_str()))
        .unwrap_or("utf-8");

    let encoding = encoding_rs::Encoding::for_label(encoding_name.as_bytes());
    let encoding = encoding.unwrap_or(encoding_rs::UTF_8);

    let (text, _, _) = encoding.decode(&response.body);
    text.to_string()
}

impl IntoResponse<Vec<u8>> for Response<Bytes> {
    fn into_response(self) -> Result<Vec<u8>> {
        if self.status.is_success() {
            Ok(self.body.to_vec())
        } else {
            Err(Error::Status(self.status))
        }
    }
}

impl IntoResponse<Response<Vec<u8>>> for Response<Bytes> {
    fn into_response(self) -> Result<Response<Vec<u8>>> {
        Ok(Response::new(self.status, self.headers, self.body.to_vec()))
    }
}

impl<T> IntoResponse<Json<T>> for Response<Bytes>
where
    T: DeserializeOwned,
{
    fn into_response(self) -> Result<Json<T>> {
        if self.status.is_success() {
            let value = parse_json(self.body)?;
            Ok(Json { value })
        } else {
            Err(Error::Status(self.status))
        }
    }
}

impl<T> IntoResponse<Response<Json<T>>> for Response<Bytes>
where
    T: DeserializeOwned,
{
    fn into_response(self) -> Result<Response<Json<T>>> {
        let value = parse_json(self.body)?;
        let body = Json { value };
        Ok(Response::new(self.status, self.headers, body))
    }
}

impl<T, E> IntoResponse<JsonResult<T, E>> for Response<Bytes>
where
    T: DeserializeOwned,
    E: DeserializeOwned,
{
    fn into_response(self) -> Result<JsonResult<T, E>> {
        if self.status.is_success() {
            let value = parse_json(self.body)?;
            Ok(JsonResult::Ok(value))
        } else {
            let value = parse_json(self.body)?;
            Ok(JsonResult::Err(value))
        }
    }
}

impl<T, E> IntoResponse<Response<JsonResult<T, E>>> for Response<Bytes>
where
    T: DeserializeOwned,
    E: DeserializeOwned,
{
    fn into_response(self) -> Result<Response<JsonResult<T, E>>> {
        if self.status.is_success() {
            let value = parse_json(self.body)?;
            Ok(Response::new(
                self.status,
                self.headers,
                JsonResult::Ok(value),
            ))
        } else {
            let value = parse_json(self.body)?;
            Ok(Response::new(
                self.status,
                self.headers,
                JsonResult::Err(value),
            ))
        }
    }
}

fn parse_json<T>(body: Bytes) -> Result<T>
where
    T: DeserializeOwned,
{
    serde_json::from_slice(body.as_ref()).map_err(Error::body)
}
