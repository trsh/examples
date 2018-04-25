extern crate actix;
extern crate actix_web;
#[macro_use]
extern crate askama;

use actix::prelude::*;
use actix_web::{http, server, App, HttpRequest, HttpResponse, Result};
use askama::Template;

#[derive(Template)]
#[template(path = "content.html")]
struct ContentTemplate<'a> {
    name: &'a str,
}

#[derive(Template)]
#[template(path = "layout.html")]
struct LayoutTemplate<'a> {
    content: &'a str,
}


fn index(_req: HttpRequest) -> Result<HttpResponse> {
    let tmpl = ContentTemplate { name: "world" };
    let tmpl_body = tmpl.render().unwrap();

    let layout = LayoutTemplate {
        content: &tmpl_body,
    };
    let layout_body = layout.render().unwrap();

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(layout_body))
}

fn main() {
    let sys = System::new("template-askama");

    // start http server
    server::new(move || App::new().resource("/", |r| r.method(http::Method::GET).f(index)))
        .bind("0.0.0.0:8080")
        .unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();
}
