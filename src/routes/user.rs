use axum::{
    extract::Json, 
    http::StatusCode,
};
use log::info;

use crate::models::user::User;

#[utoipa::path(
    get,
    path = "/user",
    tag = "user",
    responses(
        (status = 200, description = "Get user", body = User)
    )
)]
pub async fn get_user() -> Json<User> {
    info!("Handling get_user request");
    Json(User { id: 1, name: "Alice".into() })
}


#[utoipa::path(
    post,
    path = "/user",
    tag = "user",
    request_body = User,
    responses(
        (status = 201, description = "User created", body = User)
    )
)]
pub async fn create_user(Json(user): Json<User>) -> (StatusCode, Json<User>) {
    info!("Handling create_user request");
    (StatusCode::CREATED, Json(user))
}