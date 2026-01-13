use axum::{Json, response::IntoResponse};
use reqwest::StatusCode;

use crate::types::DSResponse;



pub async fn first_route() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(DSResponse {
            data: Some("hello World".to_string()),
            err: None
        })
    )
}
