use serde::Serialize;

use crate::types::message_req_types::MessageReq;

pub mod ai;

fn prompt_from(messages : MessageReq) -> String {
    format!(
        "The following is a conversation between a user and an AI assistant. The assistant is helpful, creative, clever, and very friendly.\n\nUser: {}\nAI: and the previous messages are: {:?}",
        messages.current_question,
        messages.last_messages    
    )
}

#[allow(async_fn_in_trait)]
pub trait AIChatBotProvider {
    async fn get_answer(&self, messages : MessageReq) -> Result<String, reqwest::Error>;    
}

#[derive(Debug, Clone)]
pub struct GeminiQueryProvider {
    api_key: String,
}
impl GeminiQueryProvider{
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

impl AIChatBotProvider for GeminiQueryProvider {
    async  fn get_answer(&self, messages : MessageReq) -> Result<String, reqwest::Error> {
        let prompt = prompt_from(messages);
        ai::get_suggestions(&self.api_key.as_str(), prompt.to_string()).await
    }
}