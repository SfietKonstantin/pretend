mod server;

use pretend::resolver::UrlResolver;
use pretend::{pretend, request, Pretend, Result, Url};
use pretend_reqwest::Client as RClient;

#[pretend]
trait TestApi {
    #[request(method = "GET", path = "/get")]
    async fn get(&self) -> Result<String>;
}

#[tokio::test]
async fn pretend_with_only_client_cannot_be_used() {
    let client = Pretend::for_client(RClient::default());
    let result = client.get().await;
    assert!(result.is_err());
}

#[test]
fn pretend_construct_with_client_and_resolver() {
    server::test(async {
        let url = Url::parse(server::URL).unwrap();
        let client = Pretend::new(RClient::default(), UrlResolver::new(url));
        let result = client.get().await;
        assert!(result.is_ok());
    })
}