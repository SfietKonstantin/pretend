use pretend::{header, pretend, request, Json, JsonResult, Pretend, Response, Result, Url};
use pretend_reqwest::Client;
use serde::Deserialize;

// This example show how to receive various response types.
// It uses the Github API (that returns JSON)

#[derive(Clone, Debug, Deserialize)]
struct Contributor {
    login: String,
}

type Contributors = Vec<Contributor>;

#[derive(Clone, Debug, Deserialize)]
struct GithubError {
    message: String,
}

type ContributorsResult = JsonResult<Contributors, GithubError>;

#[pretend]
trait Github {
    #[request(method = "GET", path = "/repos/SfietKonstantin/{repo}/contributors")]
    #[header(name = "User-Agent", value = "pretend example")]
    async fn string(&self, repo: &str) -> Result<String>;

    #[request(method = "GET", path = "/repos/SfietKonstantin/{repo}/contributors")]
    #[header(name = "User-Agent", value = "pretend example")]
    async fn string_response(&self, repo: &str) -> Result<Response<String>>;

    #[request(method = "GET", path = "/repos/SfietKonstantin/{repo}/contributors")]
    #[header(name = "User-Agent", value = "pretend example")]
    async fn bytes(&self, repo: &str) -> Result<Vec<u8>>;

    #[request(method = "GET", path = "/repos/SfietKonstantin/{repo}/contributors")]
    #[header(name = "User-Agent", value = "pretend example")]
    async fn bytes_response(&self, repo: &str) -> Result<Response<Vec<u8>>>;

    #[request(method = "GET", path = "/repos/SfietKonstantin/{repo}/contributors")]
    #[header(name = "User-Agent", value = "pretend example")]
    async fn json(&self, repo: &str) -> Result<Json<Contributors>>;

    #[request(method = "GET", path = "/repos/SfietKonstantin/{repo}/contributors")]
    #[header(name = "User-Agent", value = "pretend example")]
    async fn json_response(&self, repo: &str) -> Result<Response<Json<Contributors>>>;

    #[request(method = "GET", path = "/repos/SfietKonstantin/{repo}/contributors")]
    #[header(name = "User-Agent", value = "pretend example")]
    async fn json_result(&self, repo: &str) -> Result<ContributorsResult>;

    #[request(method = "GET", path = "/repos/SfietKonstantin/{repo}/contributors")]
    #[header(name = "User-Agent", value = "pretend example")]
    async fn json_result_response(&self, repo: &str) -> Result<Response<ContributorsResult>>;
}

fn create_pretend() -> impl Github {
    let url = Url::parse("https://api.github.com").unwrap();
    Pretend::for_client(Client::default()).with_url(url)
}

#[tokio::main]
async fn main() {
    let pretend = create_pretend();

    // The following successful calls will return different kind of responses.
    // Either bodies are return alone, or with status and headers by using Response.
    let result = pretend.string("pretend").await.unwrap();
    println!("{}", result);

    let result = pretend.string_response("pretend").await.unwrap();
    println!("HTTP {}, {}", result.status(), result.body());

    let result = pretend.bytes("pretend").await.unwrap();
    let body = String::from_utf8_lossy(&result);
    println!("{}", body);

    let result = pretend.bytes_response("pretend").await.unwrap();
    let body = String::from_utf8_lossy(result.body());
    println!("HTTP {}, {}", result.status(), body);

    let result = pretend.json("pretend").await.unwrap();
    println!("{:?}", result.value());

    let result = pretend.json_response("pretend").await.unwrap();
    println!("HTTP {}, {:?}", result.status(), result.body());

    let result = pretend.json_result("pretend").await.unwrap();
    println!("{:?}", result);

    let result = pretend.json_result_response("pretend").await.unwrap();
    println!("HTTP {}, {:?}", result.status(), result.body());

    // The following calls will fail for a non-existing repo

    // These calls fail on an error, as they only return a body when successful.
    let result = pretend.string("non-existing").await;
    assert!(result.is_err());

    let result = pretend.bytes("non-existing").await;
    assert!(result.is_err());

    let result = pretend.json("non-existing").await;
    assert!(result.is_err());

    // These calls returns a Response. It is possible to inspect the status
    // except for Json, as body deserialization will fail.
    let result = pretend.string_response("non-existing").await.unwrap();
    assert_eq!(result.status().as_u16(), 404);

    let result = pretend.bytes_response("non-existing").await.unwrap();
    assert_eq!(result.status().as_u16(), 404);

    let result = pretend.json_response("non-existing").await;
    assert!(result.is_err());

    // By using JsonResult, a different body can be returned on error
    let result = pretend.json_result("non-existing").await.unwrap();
    println!("{:?}", result);

    let result = pretend.json_result_response("non-existing").await.unwrap();
    println!("HTTP {}, {:?}", result.status(), result.body());
}
