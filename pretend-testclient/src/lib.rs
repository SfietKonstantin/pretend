use pretend::client::{
    Client as PClient, RequestBuilder as PRequestBuilder, ResponseBody as PResponseBody,
};
use pretend::{
    async_trait, DeserializeOwned, Error, HeaderMap, Method, Response, Result, Serialize, Url,
};
use std::io;

pub type ClientCallback =
    fn(Method, Url, HeaderMap, Option<Vec<u8>>) -> Result<Response<Option<Vec<u8>>>>;

pub struct Client {
    callback: ClientCallback,
}

impl Client {
    pub fn new(callback: ClientCallback) -> Self {
        Client { callback }
    }
}

#[async_trait]
impl PClient for Client {
    type Request = Request;
    type RequestBuilder = RequestBuilder;
    type ResponseBody = ResponseBody;

    fn request_builder(&self, method: Method, url: Url) -> Result<Self::RequestBuilder> {
        Ok(RequestBuilder {
            request: Request {
                method,
                url,
                headers: HeaderMap::new(),
                body: None,
            },
        })
    }

    async fn execute(&self, request: Self::Request) -> Result<Response<Self::ResponseBody>> {
        let response = (self.callback)(request.method, request.url, request.headers, request.body)?;
        let body = ResponseBody {
            body: response.body().clone(),
        };
        Ok(Response::new(
            *response.status(),
            response.headers().clone(),
            body,
        ))
    }
}

pub struct Request {
    method: Method,
    url: Url,
    headers: HeaderMap,
    body: Option<Vec<u8>>,
}

pub struct RequestBuilder {
    request: Request,
}

impl PRequestBuilder for RequestBuilder {
    type Request = Request;

    fn headers(mut self, headers: HeaderMap) -> Result<Self> {
        self.request.headers = headers;
        Ok(self)
    }

    fn body(mut self, body: Vec<u8>) -> Result<Self> {
        self.request.body = Some(body);
        Ok(self)
    }

    fn query<T>(mut self, query: &T) -> Result<Self>
    where
        T: Serialize,
    {
        {
            let url = &mut self.request.url;
            let mut pairs = url.query_pairs_mut();
            let serializer = serde_urlencoded::Serializer::new(&mut pairs);

            query
                .serialize(serializer)
                .map_err(|err| Error::Request(Box::new(err)))?;
        }
        Ok(self)
    }

    fn form<T>(mut self, form: &T) -> Result<Self>
    where
        T: Serialize,
    {
        let encoded =
            serde_urlencoded::to_string(form).map_err(|err| Error::Request(Box::new(err)))?;
        self.request.body = Some(encoded.as_bytes().to_vec());
        Ok(self)
    }

    fn json<T>(mut self, json: &T) -> Result<Self>
    where
        T: Serialize,
    {
        let encoded = serde_json::to_vec(json).map_err(|err| Error::Request(Box::new(err)))?;
        self.request.body = Some(encoded);
        Ok(self)
    }

    fn build(self) -> Self::Request {
        self.request
    }
}

pub struct ResponseBody {
    body: Option<Vec<u8>>,
}

impl ResponseBody {
    fn get_body(self) -> Result<Vec<u8>> {
        self.body.ok_or_else(|| {
            let error = io::Error::new(io::ErrorKind::Other, "No body");
            Error::Body(Box::new(error))
        })
    }
}

#[async_trait]
impl PResponseBody for ResponseBody {
    async fn text(self) -> Result<String> {
        let body = self.get_body()?;
        String::from_utf8(body).map_err(|err| Error::Body(Box::new(err)))
    }

    async fn bytes(self) -> Result<Vec<u8>> {
        self.get_body()
    }

    async fn json<T>(self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let body = self.get_body()?;
        serde_json::from_slice(&body).map_err(|err| Error::Body(Box::new(err)))
    }
}
