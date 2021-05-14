mod clients_tester;
mod runtimes;
mod server;

use clients_tester::{ClientsTester, TestableClient, TokioTestableClient};
use pretend::client::Client;
use pretend::Url;
use pretend_isahc::Client as IClient;
use pretend_reqwest::Client as RClient;

fn create_testable_client<C>(client: C) -> Box<dyn TestableClient>
where
    C: Client + 'static,
{
    Box::new(TokioTestableClient::new(client, runtimes::create_runtime()))
}

#[test]
fn test_clients() {
    server::test(|| {
        let url = Url::parse(server::URL).unwrap();
        let clients = vec![
            create_testable_client(RClient::default()),
            create_testable_client(IClient::new().unwrap()),
        ];
        let tester = ClientsTester::new(url, clients);
        tester.test();
    });
}
