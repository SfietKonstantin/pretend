mod runtimes;
mod server;

use self::api::TestApi;
use pretend::resolver::UrlResolver;
use pretend::{Pretend, Url};
use pretend_reqwest::Client as RClient;

mod api {
    use pretend::{pretend, request, Result};

    #[pretend]
    pub trait TestApi {
        #[request(method = "GET", path = "/method")]
        async fn get(&self) -> Result<String>;
    }
}

fn new_client() -> impl TestApi {
    let url = Url::parse(server::URL).unwrap();
    Pretend::new(RClient::default(), UrlResolver::new(url))
}

#[test]
fn pretend_generates_pub_visibility() {
    server::test(|| {
        runtimes::block_on(async {
            let result = new_client().get().await;
            assert!(result.is_ok());
        })
    })
}
