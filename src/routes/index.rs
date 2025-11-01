use axum::{
    extract::Json,
};
use log::info;

use crate::models::message::Message;

#[utoipa::path(
    get,
    path = "/",
    tag = "home",
    responses(
        (status = 200, description = "index route", body = String)
    )
)]
pub async fn index() -> Json<Message> {
    info!("Handling index request");
    Json(Message { content: "Välkommen 1337!".into() })
}