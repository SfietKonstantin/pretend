mod runtimes;
mod server;

use self::api::TestApi;
use pretend::http::HeaderValue;
use pretend::interceptor::{InterceptRequest, Request};
use pretend::{Pretend, Result, Url};
use pretend_reqwest::Client;

mod api {
    use pretend::{pretend, Json, Result};
    use std::collections::HashMap;

    #[pretend]
    pub trait TestApi {
        #[request(method = "GET", path = "/headers")]
        async fn headers(&self) -> Result<Json<HashMap<String, String>>>;
    }
}

struct AuthInterceptor;

impl InterceptRequest for AuthInterceptor {
    fn intercept(&self, mut request: Request) -> Result<Request> {
        let value = HeaderValue::from_static("Bearer abcde");
        request.headers.append("Authorization", value);
        Ok(request)
    }
}

fn new_client() -> impl TestApi {
    let url = Url::parse(server::URL).unwrap();
    Pretend::for_client(Client::default())
        .with_url(url)
        .with_request_interceptor(AuthInterceptor)
}

#[test]
fn pretend_interceptor_modifies_header() {
    server::test(|| {
        runtimes::block_on(async {
            let result = new_client().headers().await.unwrap();
            let headers = result.value();
            assert_eq!(headers.get("authorization").unwrap(), "Bearer abcde");
        })
    })
}
