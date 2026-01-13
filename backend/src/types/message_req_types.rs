use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MessageReq {
    pub current_question : String,
    pub last_messages : Vec<LastMessages>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LastMessages {
    pub user_message : Vec<String>,
    pub ai_message : Vec<String>,
}