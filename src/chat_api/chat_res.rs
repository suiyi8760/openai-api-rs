use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatCompletionRes {
    id: String,
    object: String,
    created: i64,
    pub choices: Vec<ChatChoice>,
    usage: ChatUsage,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatChoice {
    index: i32,
    message: ChatMessage,
    finish_reason: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatMessage {
    role: String,    // system assitance user 三种角色
    content: String, // 聊天内容
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatUsage {
    prompt_tokens: i32,
    completion_tokens: i32,
    total_tokens: i32,
}
