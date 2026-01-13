use axum::routing::get;

use crate::handlers::{self};
pub async fn web_api()-> axum::Router {
    axum::Router::new()
    .route("/", get(handlers::first_route::first_route))
}