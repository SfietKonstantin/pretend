mod runtimes;
mod server;

use pretend::resolver::UrlResolver;
use pretend::{pretend, request, Pretend, Result, Url};
use pretend_reqwest::BlockingClient as RBlockingClient;
use pretend_reqwest::Client as RClient;

#[pretend(?Send)]
trait TestApiLocal {
    #[request(method = "GET", path = "/method")]
    async fn get(&self) -> Result<String>;
}

#[pretend]
trait TestApiBlocking {
    #[request(method = "GET", path = "/method")]
    fn get(&self) -> Result<String>;
}

#[test]
fn pretend_with_local_and_blocking() {
    let url = Url::parse(server::URL).unwrap();

    server::test(|| {
        runtimes::block_on(async {
            let client = Pretend::new(RClient::default(), UrlResolver::new(url.clone()));
            let result = TestApiLocal::get(&client).await;
            assert!(result.is_ok());
        });

        {
            let client = Pretend::new(RBlockingClient::default(), UrlResolver::new(url.clone()));
            let result = TestApiBlocking::get(&client);
            assert!(result.is_ok());
        }
    })
}