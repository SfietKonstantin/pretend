use actix_web::dev::Server;
use actix_web::http::{HeaderName, HeaderValue, StatusCode};
use actix_web::web::{Either, Form, HttpRequest, Json, Path, Query};
use actix_web::{delete, get, patch, post, put, App, HttpResponse, HttpServer, Responder};
use pretend::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::future::Future;
use std::io;
use std::sync::mpsc::{channel, Sender};
use std::thread::{spawn, JoinHandle};

pub const URL: &str = "http://localhost:9999";

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TestData {
    pub first: String,
    pub second: i32,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ErrorData {
    pub message: String,
}

const HELLO_WORLD: &str = "Hello World";
const ERROR: &str = "Error";

#[get("/get")]
async fn get() -> impl Responder {
    HELLO_WORLD
}

#[post("/post")]
async fn post() -> impl Responder {
    HELLO_WORLD
}

#[put("/put")]
async fn put() -> impl Responder {
    HELLO_WORLD
}

#[patch("/patch")]
async fn patch() -> impl Responder {
    HELLO_WORLD
}

#[delete("/delete")]
async fn delete() -> impl Responder {
    HELLO_WORLD
}

#[get("/query")]
async fn query(info: Query<HashMap<String, String>>) -> impl Responder {
    Json(info.0)
}

#[post("/post/string")]
async fn post_with_string(body: String) -> impl Responder {
    body
}

#[post("/post/json")]
async fn post_with_json(json: Json<TestData>) -> impl Responder {
    json
}

#[post("/post/form")]
async fn post_with_form(form: Form<TestData>) -> impl Responder {
    Json(form.0)
}

#[get("/{status}/text")]
async fn get_text(status: Path<u16>) -> impl Responder {
    let response = if status.0 < 400 { HELLO_WORLD } else { ERROR };
    HttpResponse::build(StatusCode::try_from(status.0).unwrap())
        .content_type("plain/text")
        .header("x-lovely", "yes")
        .body(response)
}

#[get("/{status}/json")]
async fn get_json(status: Path<u16>) -> impl Responder {
    let mut builder = HttpResponse::build(StatusCode::try_from(status.0).unwrap());
    let builder = builder
        .content_type("application/json")
        .header("x-lovely", "yes");

    if status.0 < 400 {
        Either::A(builder.json(TestData {
            first: "Hello".to_string(),
            second: 123,
        }))
    } else {
        Either::B(builder.json(ErrorData {
            message: "Error".to_string(),
        }))
    }
}

fn map_headers((n, v): (&HeaderName, &HeaderValue)) -> Option<(String, String)> {
    let n = n.to_string();
    let v = v.to_str().ok()?;
    Some((n, v.to_string()))
}

#[get("/headers")]
async fn headers(request: HttpRequest) -> impl Responder {
    let headers = request
        .headers()
        .iter()
        .filter_map(map_headers)
        .collect::<HashMap<_, _>>();
    Json(headers)
}

struct ServerRunner {
    server: Server,
    handle: JoinHandle<io::Result<()>>,
}

impl ServerRunner {
    fn start() -> Self {
        let (send, recv) = channel();
        let handle = spawn(move || Self::run_server(send));
        let server = recv.recv().unwrap();

        ServerRunner { server, handle }
    }

    fn stop(self) {
        Self::stop_server(self.server);
        self.handle.join().unwrap().unwrap();
    }

    #[actix_web::main]
    async fn run_server(send: Sender<Server>) -> io::Result<()> {
        let supplier = || {
            App::new()
                .service(get)
                .service(post)
                .service(put)
                .service(patch)
                .service(delete)
                .service(query)
                .service(headers)
                .service(post_with_string)
                .service(post_with_json)
                .service(post_with_form)
                .service(get_text)
                .service(get_json)
        };
        let http_server = HttpServer::new(supplier)
            .bind("localhost:9999")?
            .shutdown_timeout(5);
        let server = http_server.run();
        send.send(server.clone()).unwrap();
        server.await
    }

    #[actix_web::main]
    async fn stop_server(server: Server) {
        server.stop(true).await;
    }
}

#[allow(unused)]
pub fn test<F>(f: F)
where
    F: Future<Output = ()> + 'static,
{
    let server = ServerRunner::start();
    run_future(f);
    server.stop();
}

#[allow(unused)]
pub fn test_sync<F>(f: F)
where
    F: FnOnce(),
{
    let server = ServerRunner::start();
    f();
    server.stop();
}

#[allow(unused)]
fn run_future<F>(f: F)
where
    F: Future<Output = ()> + 'static,
{
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    runtime.block_on(f);
}
