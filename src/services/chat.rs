use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, Result};

use crate::chat_api::chat_req::ChatCompletionReq;
use crate::chat_api::chat_res::{ChatChoice, ChatCompletionRes, ChatMessage};

pub async fn get_openai_response(
    prompt: Vec<ChatMessage>,
    api_key: &str,
) -> Result<Vec<ChatChoice>> {
    let endpoint = "https://api.openai.com/v1/chat/completions";
    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap(),
    );
    println!("{:?} {:?}", &prompt, &headers);
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));
    let req_body = ChatCompletionReq {
        model: "gpt-3.5-turbo".to_string(),
        message: prompt,
    };
    let client = Client::new();
    let res = client
        .post(endpoint)
        .headers(headers)
        .json(&req_body)
        .send()
        .await?
        .json::<ChatCompletionRes>()
        .await?;
    println!("{:?}", &res);
    Ok(res.choices.unwrap_or_else(Vec::new))
}
