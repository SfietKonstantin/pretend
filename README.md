# pretend

[![Build](https://github.com/SfietKonstantin/pretend/workflows/ci/badge.svg)](https://github.com/SfietKonstantin/pretend/actions)
[![codecov](https://codecov.io/gh/SfietKonstantin/pretend/branch/main/graph/badge.svg)](https://codecov.io/gh/SfietKonstantin/pretend)

`pretend` is a modular, [Feign]-inspired HTTP client based on macros. It's goal is to decouple
the definition of a REST API from it's implementation.


Some features:
- Declarative
- Asynchronous-first implementations
- HTTP client agnostic
- JSON support thanks to serde

[Feign]: https://github.com/OpenFeign/feign

## Getting started

A REST API is described by annotating a trait:

```rust
use pretend::{pretend, request, Result};

#[pretend]
trait HttpBin {
    #[request(method = "POST", path = "/anything")]
    async fn post_anything(&self, body: &'static str) -> Result<String>;
}
```

Under the hood, `pretend` will implement this trait for `Pretend`. An instance of this
struct can be constructed by passing a client implementation, and the REST API's base url. In
the following example, we are using the `reqwest` based client.

```rust
use pretend::{Pretend, Url};
use pretend_reqwest::Client;

#[tokio::main]
async fn main() {
    let client = Client::default();
    let url = Url::parse("https://httpbin.org").unwrap();
    let pretend = Pretend::for_client(client).with_url(url);
    let response = pretend.post_anything("hello").await.unwrap();
    assert!(response.contains("hello"));
}
```

## Sending headers, query parameters and bodies

Headers are provided as attributes using `header`.

```rust
use pretend::{header, pretend, request, Result};

#[pretend]
trait HttpBin {
    #[request(method = "GET", path = "/get")]
    #[header(name = "X-Test-Header-1", value = "abc")]
    #[header(name = "X-Test-Header-2", value = "other")]
    async fn get_with_headers(&self, value: i32, custom: &str) -> Result<()>;
}
```

Query parameters and bodies are provided as method parameters. Body type is guessed based on
the parameter name:

- Parameter `body` will be sent as raw bytes. This requires the body to have 'static lifetime.
- Parameter `form` will be serialized as form-encoded using `serde`.
- Parameter `json` will be serialized as JSON using `serde`.

Query parameter is passed with the `query` parameter. It is also serialized using `serde`.

```rust
use pretend::{pretend, request, Json, Result, Serialize};

#[derive(Serialize)]
struct Data {
    value: i32,
}

#[pretend]
trait HttpBin {
    #[request(method = "POST", path = "/anything")]
    async fn post_bytes(&self, body: &'static [u8]) -> Result<()>;

    #[request(method = "POST", path = "/anything")]
    async fn post_string(&self, body: &'static str) -> Result<()>;

    #[request(method = "POST", path = "/anything")]
    async fn post_with_query_params(&self, query: &Data) -> Result<()>;

    #[request(method = "POST", path = "/anything")]
    async fn post_json(&self, json: &Data) -> Result<()>;
}
```

## Handling responses

`pretend` support a wide range of response types, based on the return type of the method.
The body can be returned as a `Vec<u8>`, a string or as JSON by using the `Json` wrapper
type. The unit type `()` can also be used if the body should be discarded.

`JsonResult` is also offered as a convenience type. It will deserialize into a value type
or an error type depending on the HTTP status code.

When retrieving body alone, an HTTP error will cause the method to return an error. It is
possible to prevent the method to fail and access the HTTP status code by wrapping these
types inside a `Response`. This also allows accessing response headers.

```rust
use pretend::{pretend, request, Deserialize, Json, JsonResult, Response, Result};

#[derive(Deserialize)]
struct Data {
    value: i32,
}

#[derive(Deserialize)]
struct Error {
    error: String,
}

#[pretend]
trait HttpBin {
    #[request(method = "POST", path = "/anything")]
    async fn read_bytes(&self) -> Result<Vec<u8>>;

    #[request(method = "POST", path = "/anything")]
    async fn read_string(&self) -> Result<String>;

    #[request(method = "POST", path = "/anything")]
    async fn read_json(&self) -> Result<Json<Data>>;

    #[request(method = "POST", path = "/anything")]
    async fn read_json_result(&self) -> Result<JsonResult<Data, Error>>;

    #[request(method = "POST", path = "/anything")]
    async fn read_status(&self) -> Result<Response<()>>;
}
```

## Templating

Request paths and headers support templating. A value between braces will be replaced by
a parameter with the same name. The replacement is done with `format!`, meaning that
any type that implement `Display` is supported.

```rust
use pretend::{header, pretend, request, Deserialize, Json, Pretend, Result};
use pretend_reqwest::Client;
use std::collections::HashMap;

#[derive(Deserialize)]
struct Data {
    url: String,
    headers: HashMap<String, String>,
}

#[pretend]
trait HttpBin {
    #[request(method = "POST", path = "/{path}")]
    #[header(name = "X-{header}", value = "{value}$")]
    async fn read(&self, path: &str, header: &str, value: i32) -> Result<Json<Data>>;
}

#[tokio::main]
async fn main() {
    let client = Client::default();
    let url = Url::parse("https://httpbin.org").unwrap();
    let pretend = Pretend::for_client(client).with_url(url);
    let response = pretend.read("anything", "My-Header", 123).await.unwrap();
    let data = response.value();
    assert_eq!(data.url, "https://httpbin.org/anything");
    assert_eq!(*data.headers.get("X-My-Header").unwrap(), "123$".to_string());
}
```

## Documentation

For more information, please refer to the API reference.