use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// A simple data type we'll send and receive as JSON.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Message {
    pub content: String,
}
