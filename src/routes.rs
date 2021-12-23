use crate::handlers::user;
use axum::routing::{get, post};
use axum::Router;

pub async fn hi() -> &'static str {
    "hi"
}

pub fn new() -> Router {
    Router::new()
        .route("/", get(hi))
        .route("/user", post(user::create))
        .route("/user/:id", get(user::info))
}
