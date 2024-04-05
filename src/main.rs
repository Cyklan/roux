use std::{io, path::PathBuf};

use poem::{
    endpoint::StaticFilesEndpoint, get, handler, listener::TcpListener, Response, Route, Server,
};

mod routes;

const EMBEDDED_CSS: &'static str = include_str!(concat!(env!("OUT_DIR"), "/style.css"));
const EMBEDDED_JS: &'static str = include_str!(concat!(env!("OUT_DIR"), "/index.mjs"));

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

#[handler]
async fn my_element() -> Response {
    Response::builder()
        .header("Content-Type", "text/html")
        .body("<my-element name=\"mom ❤️\" id=\"foo\"></my-element>")
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
        .nest("/", routes::index::routes())
        .at("/public/index.css", get(css))
        .at("/public/index.js", get(js))
        .nest("/public", StaticFilesEndpoint::new(public_dir()?))
        .at("/my-element", get(my_element));

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}

fn public_dir() -> io::Result<PathBuf> {
    let mut path = std::env::current_exe()?;
    path.pop();
    path.push("public");
    Ok(path)
}
