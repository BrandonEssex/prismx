// src/node.rs

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub parent: Option<Uuid>,
    pub children: Vec<Uuid>,
    pub shard: Option<String>,
    pub tags: Vec<String>,
}