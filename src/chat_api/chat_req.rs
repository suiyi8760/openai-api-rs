use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatCompletionReq {
    pub model: String,
    pub message: Vec<super::chat_res::ChatMessage>,
    // 可选类型
    // temperature: f32,
    // top_p: f32,
    // n: i32,
    // stream: bool,
    // user: String,
    // TODO: 还有部分类型没列出，需要再加上
}


