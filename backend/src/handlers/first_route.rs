use axum::{Json, response::IntoResponse};
use reqwest::StatusCode;

use crate::{types::DSResponse, web::json_resp};

pub async fn first_route() -> impl IntoResponse {
    json_resp(Some(StatusCode::OK), Some("Hello world!"))
}
