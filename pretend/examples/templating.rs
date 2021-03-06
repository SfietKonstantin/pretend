use pretend::{pretend, Pretend, Result, Url};
use pretend_reqwest::Client;

// This example show how to use templating to customize paths and headers

#[pretend]
trait HttpBin {
    #[request(method = "GET", path = "/{path}")]
    #[header(name = "X-{header}", value = "{first}-{second}")]
    #[header(name = "X-Test", value = "{value}")]
    async fn get(
        &self,
        path: &str,
        header: &str,
        first: i32,
        second: i32,
        value: &str,
    ) -> Result<String>;
}

fn create_pretend() -> impl HttpBin {
    let url = Url::parse("https://httpbin.org").unwrap();
    Pretend::for_client(Client::default()).with_url(url)
}

#[tokio::main]
async fn main() {
    let pretend = create_pretend();

    let future = pretend.get("get", "Header", 1, 2, "something");
    let result = future.await.unwrap();
    println!("{}", result);
}
