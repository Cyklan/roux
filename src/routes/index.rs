use askama::Template;
use poem::{handler, middleware::SetHeader, web::Path, EndpointExt};

#[derive(Template)]
#[template(path = "base.html")]
struct BaseTemplate<'a> {
    title: &'a str,
    nav_target: &'a str,
}

#[derive(Template)]
#[template(path = "counter.html")]
struct CounterTemplate {
    count: i32,
}

#[handler]
pub fn get() -> String {
    let base = BaseTemplate {
        title: "Rust, HTMX and Web Components!",
        nav_target: "/foo",
    };
    base.render().unwrap()
}

#[handler]
pub fn get_counter(Path(count): Path<i32>) -> String {
    let counter = CounterTemplate { count: count + 1 };
    counter.render().unwrap()
}

pub fn routes() -> impl poem::EndpointExt {
    poem::Route::new()
        .at("/", get)
        .at("/counter/:count", get_counter)
        .with(SetHeader::new().overriding("Content-Type", "text/html; charset=utf-8"))
}
