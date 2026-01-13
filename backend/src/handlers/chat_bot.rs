use std::sync::Arc;

use axum::{extract::State, http::Error, response::IntoResponse};
use reqwest::StatusCode;

use crate::{adapters::gemini_response::AIChatBotProvider, types::message_req_types::MessageReq, web::{json_err, json_resp}};

pub async fn get_answer_from_ai(
    State(provider) : State<Arc<dyn AIChatBotProvider +Send + Sync>>
    payload: MessageReq,
) -> impl IntoResponse{
    match get_answer(payload).await {
        Ok(answer) => {
            Ok(json_resp((Some(StatusCode::OK)), Some(answer)))
        },
        Err(e) => {
            tracing::debug!("Error {}", e);
            Err(json_err(Some(StatusCode::INTERNAL_SERVER_ERROR), Some("Error getting answer".to_string())))
        }
    }
}