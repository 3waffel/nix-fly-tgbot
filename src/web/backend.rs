use askama::Template;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
};
use axum::{Json, Router, Server};
use chrono::{offset::Utc, DateTime};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::SystemTime;
use std::vec::Vec;
use uuid::Uuid;

pub async fn backend_runner(port: i32) {
    let shared_posts = Posts::default();

    let app = Router::new()
        .route("/", get(index_handler).post(create_post))
        // TODO
        .route("/posts/:id", get(get_post))
        .route("/edit", post(update_post))
        .route("/delete", post(delete_post))
        .with_state(shared_posts);

    let addr = format!("127.0.0.1:{port}");
    Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/// home page
async fn index_handler(State(posts): State<Posts>) -> impl IntoResponse {
    let posts = posts.read().unwrap();
    if posts.is_empty() {
        let hello = Post::new()
            .title("demo")
            .date("2020-01-01")
            .content("Hello World!");
        return Json(vec![hello]);
    }

    let posts = posts.values().cloned().collect::<Vec<_>>();
    Json(posts)
}

#[derive(Debug, Deserialize)]
struct CreatePostPayload {
    title: String,
    content: String,
}

/// handle post request and create post
async fn create_post(
    State(posts): State<Posts>,
    Json(input): Json<CreatePostPayload>,
) -> impl IntoResponse {
    let time: DateTime<Utc> = SystemTime::now().into();
    let post = Post::new()
        .title(input.title)
        .date(format!("{}", time.format("%d/%m/%Y %T")))
        .content(input.content);

    let mut posts = posts.write().unwrap();
    posts.insert(post.id, post.clone());
    (StatusCode::CREATED, Json(post))
}

async fn get_post(Path(id): Path<Uuid>) -> impl IntoResponse {}

async fn update_post() -> Result<impl IntoResponse, StatusCode> {
    Ok(())
}

async fn delete_post() -> impl IntoResponse {
    StatusCode::NOT_FOUND
}

type Posts = Arc<RwLock<HashMap<Uuid, Post>>>;

/// Post
#[derive(Debug, Clone, Default, Serialize)]
struct Post {
    id: Uuid,
    title: String,
    date: String,
    content: String,
}

impl Post {
    fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            ..Default::default()
        }
    }

    fn title<S>(mut self, title: S) -> Self
    where
        S: Into<String>,
    {
        self.title = title.into();
        self
    }

    fn date<S>(mut self, date: S) -> Self
    where
        S: Into<String>,
    {
        self.date = date.into();
        self
    }

    fn content<S>(mut self, content: S) -> Self
    where
        S: Into<String>,
    {
        self.content = content.into();
        self
    }
}
