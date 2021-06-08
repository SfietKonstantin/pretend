mod runtimes;
mod server;

use pretend::http::{HeaderValue, StatusCode};
use pretend::{pretend, Error, Json, JsonResult, Pretend, Response, Result, Url};
use pretend_reqwest::Client;

type TestDataResult = JsonResult<server::TestData, server::ErrorData>;

#[pretend]
trait TestApi {
    #[request(method = "GET", path = "/{status}/text")]
    async fn get_unit(&self, status: i32) -> Result<()>;
    #[request(method = "GET", path = "/{status}/text")]
    async fn get_unit_response(&self, status: i32) -> Result<Response<()>>;
    #[request(method = "GET", path = "/{status}/text")]
    async fn get_text(&self, status: i32) -> Result<String>;
    #[request(method = "GET", path = "/{status}/text")]
    async fn get_text_response(&self, status: i32) -> Result<Response<String>>;
    #[request(method = "GET", path = "/{status}/text")]
    async fn get_bytes(&self, status: i32) -> Result<Vec<u8>>;
    #[request(method = "GET", path = "/{status}/text")]
    async fn get_bytes_response(&self, status: i32) -> Result<Response<Vec<u8>>>;
    #[request(method = "GET", path = "/{status}/json")]
    async fn get_json(&self, status: i32) -> Result<Json<server::TestData>>;
    #[request(method = "GET", path = "/{status}/json")]
    async fn get_json_response(&self, status: i32) -> Result<Response<Json<server::TestData>>>;
    #[request(method = "GET", path = "/{status}/json")]
    async fn get_json_result(&self, status: i32) -> Result<TestDataResult>;
    #[request(method = "GET", path = "/{status}/json")]
    async fn get_json_result_response(&self, status: i32) -> Result<Response<TestDataResult>>;
}

fn new_pretend() -> impl TestApi {
    let url = Url::parse(server::URL).unwrap();
    let client = Client::default();
    Pretend::for_client(client).with_url(url)
}

fn get_err_status<T>(result: Result<T>) -> Option<u16> {
    match result {
        Err(Error::Status(status)) => Some(status.as_u16()),
        _ => None,
    }
}

fn is_body_err<T>(result: Result<T>) -> bool {
    match result {
        Err(Error::Body(_)) => true,
        _ => true,
    }
}

#[test]
fn test_output() {
    server::test(|| {
        runtimes::block_on(async {
            test_status_unit().await;
            test_status_unit_response().await;
            test_status_text().await;
            test_status_text_response().await;
            test_status_bytes().await;
            test_status_bytes_response().await;
            test_status_json().await;
            test_status_json_response().await;
            test_status_json_result().await;
            test_status_json_result_response().await;
        })
    })
}

async fn test_status_unit() {
    let api = new_pretend();

    let result = api.get_unit(200).await.unwrap();
    assert_eq!(result, ());

    let result = api.get_unit(402).await;
    assert_eq!(get_err_status(result), Some(402));
}

async fn test_status_unit_response() {
    let api = new_pretend();

    let expected_header = HeaderValue::from_str("yes").unwrap();

    let result = api.get_unit_response(200).await.unwrap();
    let header = result.headers().get("x-lovely").unwrap();
    assert_eq!(*result.status(), StatusCode::from_u16(200).unwrap());
    assert_eq!(*header, expected_header);
    assert_eq!(*result.body(), ());

    let result = api.get_unit_response(402).await.unwrap();
    let header = result.headers().get("x-lovely").unwrap();
    assert_eq!(*result.status(), StatusCode::from_u16(402).unwrap());
    assert_eq!(*header, expected_header);
    assert_eq!(*result.body(), ());
}

async fn test_status_text() {
    let api = new_pretend();

    let result = api.get_text(200).await.unwrap();
    assert_eq!(result, "Hello World");

    let result = api.get_text(402).await;
    assert_eq!(get_err_status(result), Some(402));
}

async fn test_status_text_response() {
    let api = new_pretend();

    let result = api.get_text_response(200).await.unwrap();
    assert_eq!(*result.status(), StatusCode::from_u16(200).unwrap());
    assert_eq!(result.body(), "Hello World");

    let result = api.get_text_response(402).await.unwrap();
    assert_eq!(*result.status(), StatusCode::from_u16(402).unwrap());
    assert_eq!(result.body(), "Error");
}

async fn test_status_bytes() {
    let api = new_pretend();

    let result = api.get_bytes(200).await.unwrap();
    assert_eq!(String::from_utf8_lossy(&result), "Hello World");

    let result = api.get_bytes(402).await;
    assert_eq!(get_err_status(result), Some(402));
}

async fn test_status_bytes_response() {
    let api = new_pretend();

    let result = api.get_bytes_response(200).await.unwrap();
    assert_eq!(*result.status(), StatusCode::from_u16(200).unwrap());
    assert_eq!(String::from_utf8_lossy(&result.body()), "Hello World");

    let result = api.get_bytes_response(402).await.unwrap();
    assert_eq!(*result.status(), StatusCode::from_u16(402).unwrap());
    assert_eq!(String::from_utf8_lossy(&result.body()), "Error");
}

async fn test_status_json() {
    let api = new_pretend();

    let expected = server::TestData {
        first: "Hello".to_string(),
        second: 123,
    };
    let result = api.get_json(200).await.unwrap();
    assert_eq!(result.value(), expected);

    let result = api.get_json(402).await;
    assert_eq!(get_err_status(result), Some(402));
}

async fn test_status_json_response() {
    let api = new_pretend();

    let expected = server::TestData {
        first: "Hello".to_string(),
        second: 123,
    };
    let result = api.get_json_response(200).await.unwrap();
    assert_eq!(*result.status(), StatusCode::from_u16(200).unwrap());
    assert_eq!(result.into_body().value(), expected);

    let result = api.get_json_response(402).await;
    assert!(is_body_err(result));
}

async fn test_status_json_result() {
    let api = new_pretend();

    let expected = server::TestData {
        first: "Hello".to_string(),
        second: 123,
    };
    let result = api.get_json_result(200).await.unwrap();
    assert_eq!(result, JsonResult::Ok(expected));

    let expected = server::ErrorData {
        message: "Error".to_string(),
    };
    let result = api.get_json_result(402).await.unwrap();
    assert_eq!(result, JsonResult::Err(expected));
}

async fn test_status_json_result_response() {
    let api = new_pretend();

    let expected = server::TestData {
        first: "Hello".to_string(),
        second: 123,
    };
    let result = api.get_json_result_response(200).await.unwrap();
    assert_eq!(*result.status(), StatusCode::from_u16(200).unwrap());
    assert_eq!(result.into_body(), JsonResult::Ok(expected));

    let expected = server::ErrorData {
        message: "Error".to_string(),
    };
    let result = api.get_json_result_response(402).await.unwrap();
    assert_eq!(*result.status(), StatusCode::from_u16(402).unwrap());
    assert_eq!(result.into_body(), JsonResult::Err(expected));
}
