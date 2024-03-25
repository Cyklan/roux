use askama::Template;
use poem::{handler, middleware::SetHeader, EndpointExt};

#[derive(Template)]
#[template(path = "base.html")]
struct BaseTemplate<'a> {
    title: &'a str,
}

#[handler]
pub fn get() -> String {
    let base = BaseTemplate {
        title: "Hello, Roux!",
    };
    base.render().unwrap()
}

pub fn routes() -> impl poem::EndpointExt {
    poem::Route::new()
        .at("/", get)
        .with(SetHeader::new().overriding("Content-Type", "text/html; charset=utf-8"))
}
