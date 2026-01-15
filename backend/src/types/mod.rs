use serde::{Deserialize, Serialize};

use crate::adapters::gemini_response::{AIChatBotProvider, GeminiQueryProvider};
pub mod message_req_types;
pub mod ai_response_types;
#[derive(Serialize, Deserialize)]
pub struct DSResponse<T> {
    pub data: Option<T>,
    pub err: Option<T>,
}

impl<T> DSResponse<T> {
    pub fn new(data: Option<T>, err: Option<T>) -> Self {
        Self { data, err }
    }
}

pub struct DSError<T> {
    pub data: Option<T>,
    pub err: Option<T>,
}

impl<T> DSError<T> {
    pub fn new(data: Option<T>, err: Option<T>) -> Self {
        Self { data, err }
    }
}

#[derive(Clone)]
pub struct AppState {
    pub ai_chat_provider: GeminiQueryProvider,
}
