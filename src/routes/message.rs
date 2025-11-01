use axum::{
    extract::Json
};
use log::info;

use crate::models::message::Message;

#[utoipa::path(
    get,
    path = "/message",
    tag = "message",
    responses(
        (status = 200, description = "Get message", body = String)
    )
)]
pub async fn list_message() -> Json<Vec<String>> {
    info!("Handling list_messages request");
    Json(vec!["Hello from the server!".to_string()])
}

#[utoipa::path(
    post,
    path = "/message",
    responses(
        (status = 200, description = "create message", body = String)
    )
)]
pub async fn create_message(Json(message): Json<Message>) -> Json<String> {
    info!("Handling create_message request");
    Json(format!("New message: {}", message.content))
}