use std::net::SocketAddr;

use axum::Json;
use axum::http::StatusCode;
use axum::response::Html;
use axum::routing::get;
use minijinja::{Environment, path_loader, context};
use once_cell::sync::Lazy;
use serde::Serialize;

static ENV: Lazy<Environment<'static>> = Lazy::new(|| {
    let mut env = Environment::new();
    env.set_loader(path_loader("templates"));
    env
});

#[derive(Serialize)]
pub struct Page {
    title: &'static str,
}

#[tokio::main]
async fn main() {
    let app = axum::Router::new()
        .route("/healthz", get(healthz))
        .route("/", get(root))
        .route("/posts", get(posts_index));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn healthz() -> (StatusCode, Json<&'static str>) {
    (StatusCode::OK, Json("ok"))
}

async fn root() -> (StatusCode, Html<String>) {
    let tmpl = ENV.get_template("index.html").unwrap();

    let page = Page { title: "Home" };
    let ctx = context!(page);

    (StatusCode::OK, Html(tmpl.render(ctx).unwrap()))
}

async fn posts_index() -> (StatusCode, Html<String>) {
    let tmpl = ENV.get_template("posts.html").unwrap();

    let page = Page { title: "Posts" };
    let ctx = context!(page);

    (StatusCode::OK, Html(tmpl.render(ctx).unwrap()))
}
