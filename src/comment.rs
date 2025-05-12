// src/comment.rs

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub id: Uuid,
    pub author: String,
    pub body: String,
    pub timestamp: String,
    pub node_ref: Option<Uuid>,
}