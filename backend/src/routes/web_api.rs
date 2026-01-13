use axum::{response::Html, routing::get};

use crate::handlers::{self};
pub async fn web_api() -> axum::Router {
    axum::Router::new()
    .route("/", get(handlers::first_route::first_route))
    .route("/hello", get(|| async {
        Html("<h1>Hello from Web API!</h1>")
    }))
}
