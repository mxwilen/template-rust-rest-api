use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct Response {
    pub status_code: i32,
    pub message: String,
    pub content: String,
}
