use askama::Template;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
};
use axum::{Json, Router, Server};
use std::vec::Vec;
use uuid::Uuid;

pub async fn frontend_runner<S>(port: i32, api_url: S)
where
    S: Into<String>,
{
    let app = Router::new()
        .route("/", get(homepage_handler))
        .route("/post/:id", get(post_handler));
    let addr = format!("127.0.0.1:{port}");
    Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/// Serve Homepage
async fn homepage_handler() -> impl IntoResponse {
    let homepage = HomepageTemplate { posts: vec![] };
    HtmlTemplate(homepage)
}

async fn post_handler(Path(id): Path<Uuid>) -> impl IntoResponse {
    let post = PostTemplate::default();
    HtmlTemplate(post)
}

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}

#[derive(Template)]
#[template(path = "index.html")]
struct HomepageTemplate {
    posts: Vec<PostTemplate>,
}

#[derive(Template, Default)]
#[template(path = "post.html")]
struct PostTemplate {
    title: String,
    date: String,
    content: String,
}
