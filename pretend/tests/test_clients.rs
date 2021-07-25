mod clients_tester;
mod runtimes;
mod server;

use clients_tester::{
    ClientsTester, TestableClient, TokioTestableClient, TokioTestableLocalClient,
};
use pretend::client::{Bytes, Client, Method};
use pretend::local::client::Client as LocalClient;
use pretend::local::Result;
use pretend::{HeaderMap, Response, Url};
use pretend_awc::Client as AClient;
use pretend_isahc::Client as IClient;
use pretend_reqwest::{BlockingClient as RBlockingClient, Client as RClient};

fn create_testable<C>(client: C) -> Box<dyn TestableClient>
where
    C: Client + 'static,
{
    Box::new(TokioTestableClient::new(
        Box::new(client),
        runtimes::create_runtime(),
    ))
}

fn create_testable_local<C>(client: C) -> Box<dyn TestableClient>
where
    C: LocalClient + 'static,
{
    Box::new(TokioTestableLocalClient::new(
        Box::new(client),
        runtimes::create_runtime(),
    ))
}

struct TestableAwcClient;

#[actix_web::main]
async fn awc_execute(
    method: Method,
    url: Url,
    headers: HeaderMap,
    body: Option<Bytes>,
) -> Result<Response<Bytes>> {
    AClient::default().execute(method, url, headers, body).await
}

impl TestableClient for TestableAwcClient {
    fn execute(
        &self,
        method: Method,
        url: Url,
        headers: HeaderMap,
        body: Option<Bytes>,
    ) -> Result<Response<Bytes>> {
        awc_execute(method, url, headers, body)
    }
}

#[test]
fn test_all_clients() {
    server::test(|| {
        let url = Url::parse(server::URL).unwrap();
        test_clients(url.clone());
        test_local_clients(url.clone());
        test_blocking_clients(url.clone());
    });
}

fn test_clients(url: Url) {
    let clients = vec![
        create_testable(RClient::default()),
        create_testable(IClient::new().unwrap()),
    ];
    let tester = ClientsTester::new(url, clients);
    tester.test();
}

fn test_local_clients(url: Url) {
    let clients: Vec<Box<dyn TestableClient>> = vec![
        create_testable_local(RClient::default()),
        create_testable_local(IClient::new().unwrap()),
        Box::new(TestableAwcClient),
    ];
    let tester = ClientsTester::new(url, clients);
    tester.test();
}

fn test_blocking_clients(url: Url) {
    let clients: Vec<Box<dyn TestableClient>> = vec![Box::new(RBlockingClient::default())];
    let tester = ClientsTester::new(url, clients);
    tester.test();
}
