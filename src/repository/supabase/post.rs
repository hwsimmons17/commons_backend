use axum::async_trait;
use reqwest::StatusCode;
use uuid::Uuid;

use crate::repository::post::{Post, PostRepository};

use super::SupabaseRepo;

const TABLE: &str = "textPosts";

#[async_trait]
impl PostRepository for SupabaseRepo {
    async fn create(&mut self, post: Post) -> Result<Post, String> {
        match self
            .client
            .from(TABLE)
            .insert(post.format_for_postrest())
            .execute()
            .await
        {
            Ok(r) => {
                if r.status() == StatusCode::CREATED {
                    return Ok(post);
                }

                println!("Error creating post: {:?}", r.text().await);
                return Err("Post not created".to_string());
            }
            Err(_) => return Err("Post not created".to_string()),
        }
    }

    async fn read_all(&self) -> Result<Vec<Post>, String> {
        match self.client.from(TABLE).select("*").execute().await {
            Ok(r) => match r.text().await {
                Ok(t) => {
                    let body: Result<Vec<Post>, serde_json::Error> = serde_json::from_str(&t);
                    match body {
                        Ok(b) => Ok(b),
                        Err(_) => Err("Could not read posts".to_string()),
                    }
                }
                Err(_) => Err("Could not read posts".to_string()),
            },
            Err(_) => return Err("Could not read posts".to_string()),
        }
    }

    async fn update_post(&mut self, post: Post) -> Result<Post, String> {
        match self
            .client
            .from(TABLE)
            .eq("id", post.id.to_string())
            .update(post.format_for_postrest())
            .execute()
            .await
        {
            Ok(r) => {
                if r.status() == StatusCode::OK {
                    return Ok(post);
                }
                println!("Error updating post: {:?}", r.text().await);
                return Err("Post not updated".to_string());
            }
            Err(_) => return Err("Post not updated".to_string()),
        }
    }

    async fn delete_post(&mut self, id: Uuid) -> Result<Option<Post>, String> {
        match self
            .client
            .from(TABLE)
            .eq("id", id.to_string())
            .delete()
            .execute()
            .await
        {
            Ok(r) => match r.text().await {
                Ok(t) => {
                    let body: Result<Vec<Post>, serde_json::Error> = serde_json::from_str(&t);
                    match body {
                        Ok(posts) => {
                            if posts.len() == 0 {
                                return Err("Post not deleted".to_string());
                            }
                            return Ok(Some(posts[0].clone()));
                        }
                        Err(_) => Err("Post not deleted".to_string()),
                    }
                }
                Err(_) => Err("Post not deleted".to_string()),
            },
            Err(_) => return Err("Post not deleted".to_string()),
        }
    }
}

impl Post {
    fn format_for_postrest(&self) -> String {
        format!(
            r#"[{{"id": "{}","content": "{}", "creator_id": "{}"}}]"#,
            self.id, self.content, self.creator_id
        )
    }
}
