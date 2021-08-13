use pretend::{pretend, request, Pretend, Result, Url};

#[pretend]
trait HttpBin {
    #[request(method = "GET", path = "/get")]
    async fn get(&self) -> Result<()>;
}

#[pretend]
trait HttpBinSync {
    #[request(method = "GET", path = "/get")]
    fn get(&self) -> Result<()>;
}

#[tokio::test]
async fn test_reqwest_no_https_feature() {
    // With default-features = false
    // reqwest will not use any TLS crate,
    // and thus, cannot handle https calls

    let url = Url::parse("https://httpbin.org").unwrap();
    let pretend = Pretend::for_client(pretend_reqwest::Client::default()).with_url(url);
    let result = pretend.get().await;

    let error = anyhow::Error::from(result.unwrap_err());
    assert!(format!("{:?}", error).contains("scheme is not http"));
}

#[test]
fn test_ureq_no_https_feature() {
    // With default-features = false
    // ureq will not use any TLS crate,
    // and thus, cannot handle https calls

    let url = Url::parse("https://httpbin.org").unwrap();
    let pretend =
        Pretend::for_client(pretend_ureq::Client::new(pretend_ureq::ureq::agent())).with_url(url);
    let result = pretend.get();

    let error = anyhow::Error::from(result.unwrap_err());
    assert!(format!("{:?}", error).contains("ureq was build without HTTP support"));
}
