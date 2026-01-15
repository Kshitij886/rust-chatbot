use std::sync::Arc;

use axum::{Json, extract::State, http::Error, response::IntoResponse};
use reqwest::StatusCode;
use serde_json::Value;

use crate::{
    adapters::gemini_response::AIChatBotProvider,
    types::{AppState, ai_response_types::ModelResponse, message_req_types::MessageReq},
    web::{json_err, json_resp},
};

pub async fn get_answer_from_ai(
    State(state): State<AppState>,
    Json(payload): Json<MessageReq>,
) -> impl IntoResponse {
    match state.ai_chat_provider.get_answer(payload).await {
        Ok(answer) => match serde_json::from_str::<ModelResponse>(&answer) {
            Ok(data) => {
                let res = data.candidates[0].content.parts[0].text.clone();
                Ok(json_resp((Some(StatusCode::OK)), Some(res)))
            }
            Err(e) => {
                tracing::debug!("Error {}", e);
                Err(json_err(
                    Some(StatusCode::INTERNAL_SERVER_ERROR),
                    Some("Error parsing json".to_string()),
                ))
            }
        },
        Err(e) => {
            tracing::debug!("Error {}", e);
            Err(json_err(
                Some(StatusCode::INTERNAL_SERVER_ERROR),
                Some("Error getting answer".to_string()),
            ))
        }
    }
}
