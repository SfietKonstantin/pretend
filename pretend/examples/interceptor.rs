use pretend::http::HeaderValue;
use pretend::interceptor::{InterceptRequest, Request};
use pretend::{pretend, Pretend, Result, Url};
use pretend_reqwest::Client;

// This example show how to use an interceptor to override headers

#[pretend]
trait HttpBin {
    #[request(method = "GET", path = "/get")]
    async fn get(&self) -> Result<String>;
}

struct AuthInterceptor;

impl InterceptRequest for AuthInterceptor {
    fn intercept(&self, mut request: Request) -> Result<Request> {
        let value = HeaderValue::from_static("Bearer abcde");
        request.headers.append("Authorization", value);
        Ok(request)
    }
}

fn create_pretend() -> impl HttpBin {
    let url = Url::parse("https://httpbin.org").unwrap();
    Pretend::for_client(Client::default())
        .with_url(url)
        .with_request_interceptor(AuthInterceptor)
}

#[tokio::main]
async fn main() {
    let pretend = create_pretend();

    let future = pretend.get();
    let result = future.await.unwrap();
    println!("{}", result);
}
