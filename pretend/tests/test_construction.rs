mod runtimes;
mod server;

use pretend::interceptor::NoopRequestInterceptor;
use pretend::resolver::UrlResolver;
use pretend::{pretend, Pretend, Result, Url};
use pretend_reqwest::Client;

#[pretend]
trait TestApi {
    #[request(method = "GET", path = "/method")]
    async fn get(&self) -> Result<String>;
}

#[tokio::test]
async fn pretend_with_only_client_cannot_be_used() {
    let client = Pretend::for_client(Client::default());
    let result = client.get().await;
    assert!(result.is_err());
}

fn pretend_construct_with_client_and_resolver() {
    runtimes::block_on(async {
        let url = Url::parse(server::URL).unwrap();
        let client = Client::default();
        let resolver = UrlResolver::new(url);
        let client = Pretend::for_client(client).with_url_resolver(resolver);
        let result = client.get().await;
        assert!(result.is_ok());
    })
}

fn pretend_construct_with_client_resolver_and_interceptor() {
    runtimes::block_on(async {
        let url = Url::parse(server::URL).unwrap();
        let client = Client::default();
        let resolver = UrlResolver::new(url);
        let interceptor = NoopRequestInterceptor;
        let client = Pretend::for_client(client)
            .with_url_resolver(resolver)
            .with_request_interceptor(interceptor);
        let result = client.get().await;
        assert!(result.is_ok());
    })
}

fn pretend_construct_with_constructor() {
    runtimes::block_on(async {
        let url = Url::parse(server::URL).unwrap();
        let client = Client::default();
        let resolver = UrlResolver::new(url);
        let interceptor = NoopRequestInterceptor;
        let client = Pretend::new(client, resolver, interceptor);
        let result = client.get().await;
        assert!(result.is_ok());
    })
}

#[test]
fn pretend_constructors() {
    server::test(|| {
        pretend_construct_with_client_and_resolver();
        pretend_construct_with_client_resolver_and_interceptor();
        pretend_construct_with_constructor();
    })
}
