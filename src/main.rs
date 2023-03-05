use actix_web::{post, web, App, HttpResponse, HttpServer};

mod chat_api;

mod services;
use services::chat::get_openai_response;

use crate::chat_api::chat_res::ChatMessage;

#[post("/chat")]
async fn chat_req(req: web::Json<Vec<ChatMessage>>) -> HttpResponse {
    // TODO: 先硬编码到这里，后面抽到env
    let api_key = "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";
    let gpt_response = get_openai_response(req.into_inner(), api_key)
        .await
        .expect("Failed to get GPT response");
    HttpResponse::Ok().json(gpt_response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(chat_req))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
