use poem::{get, handler, listener::TcpListener, Response, Route, Server};

mod routes;

// const EMBEDDED_CSS: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/public/index.css"));
const EMBEDDED_CSS: &'static str = include_str!(concat!(env!("OUT_DIR"), "/index.css"));
const EMBEDDED_JS: &'static str = include_str!(concat!(env!("OUT_DIR"), "/dist/assets/index.js"));

#[handler]
async fn css() -> Response {
    Response::builder()
        .header("Content-Type", "text/css")
        .body(EMBEDDED_CSS)
}

#[handler]
async fn js() -> Response {
    Response::builder()
        .header("Content-Type", "application/javascript")
        .body(EMBEDDED_JS)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
        .nest("/", routes::index::routes())
        // .at("/public", StaticFilesEndpoint::new("public"));
        .at("/public/index.css", get(css))
        .at("/public/index.js", get(js));

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
