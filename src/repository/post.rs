use std::sync::Arc;

use axum::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct Post {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub content: String,
    pub parent_id: Option<Uuid>,
    pub creator_id: u64,
    pub creator_name: String,
    pub creator_picture: String,
}

pub type DynPostRepo = Arc<Mutex<dyn PostRepository>>;

#[async_trait]
pub trait PostRepository: Send + Sync + 'static {
    async fn create(&mut self, post: Post) -> Result<Post, String>;
    async fn read_all(&self) -> Result<Vec<Post>, String>;
    async fn update_post(&mut self, post: Post) -> Result<Post, String>;
    async fn delete_post(&mut self, id: Uuid) -> Result<Option<Post>, String>;
}
