use askama::Template;
use poem::{handler, middleware::SetHeader, EndpointExt};

#[derive(Template)]
#[template(path = "foo.html")]
pub struct FooTemplate<'a> {
    title: &'a str,
    nav_target: &'a str,
}

#[handler]
pub fn get() -> String {
    let base = FooTemplate {
        title: "Rust, HTMX and Web Components! - foo",
        nav_target: "/",
    };
    base.render().unwrap()
}

pub fn routes() -> impl poem::EndpointExt {
    poem::Route::new()
        .at("/", get)
        .with(SetHeader::new().overriding("Content-Type", "text/html; charset=utf8"))
}
