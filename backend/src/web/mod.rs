use axum::Json;
use reqwest::StatusCode;
use serde_json::error;

use crate::types::{DSError, DSResponse};

pub fn json_resp<T>(code: Option<StatusCode>, data: T) -> (StatusCode, Json<DSResponse<T>>) {
    return (
        code.unwrap_or_default(),
        Json(DSResponse {
            data: Some(data),
            err: None,
        }),
    );
}
pub fn json_err<T>(code: Option<StatusCode>, error: T) -> (StatusCode, Json<DSResponse<T>>) {
    return (
        code.unwrap_or_default(),
        Json(DSResponse {
            data: None,
            err: Some(error),
        }),
    );
}
