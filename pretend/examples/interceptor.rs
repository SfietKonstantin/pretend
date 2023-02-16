use pretend::http::header::AUTHORIZATION;
use pretend::http::HeaderValue;
use pretend::interceptor::{InterceptRequest, Request};
use pretend::{pretend, Error, Pretend, Result, Url};
use pretend_reqwest::Client;

// This example show how to use an interceptor to override headers

#[pretend]
trait HttpBin {
    #[request(method = "GET", path = "/get")]
    async fn get(&self) -> Result<String>;
}

struct AuthInterceptor {
    auth: String,
}

impl AuthInterceptor {
    fn new(auth: String) -> Self {
        AuthInterceptor { auth }
    }
}

impl InterceptRequest for AuthInterceptor {
    fn intercept(&self, mut request: Request) -> Result<Request> {
        // Create the header, reporting failure if the header is invalid
        let header = format!("Bearer {}", self.auth);
        let header = HeaderValue::from_str(&header).map_err(|err| Error::Request(Box::new(err)))?;

        // Set the authorization header in the request
        request.headers.append(AUTHORIZATION, header);
        Ok(request)
    }
}

fn create_pretend() -> impl HttpBin {
    let url = Url::parse("https://httpbin.org").unwrap();
    Pretend::for_client(Client::default())
        .with_url(url)
        .with_request_interceptor(AuthInterceptor::new("test".to_string()))
}

#[tokio::main]
async fn main() {
    let pretend = create_pretend();

    let future = pretend.get();
    let result = future.await.unwrap();
    println!("{}", result);
}
