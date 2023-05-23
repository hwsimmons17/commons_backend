use axum::async_trait;
use chrono::{DateTime, Utc};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::repository::post::{Post, PostRepository};

use super::SupabaseRepo;

const TABLE: &str = "textPosts";

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
struct JoinPost {
    id: Uuid,
    created_at: DateTime<Utc>,
    content: String,
    parent_id: Option<Uuid>,
    creator_id: u64,
    #[serde(rename = "facebookUsers")]
    facebook_users: FacebookUsers,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
struct FacebookUsers {
    name: String,
    profile_picture: String,
}

impl JoinPost {
    fn convert_to_post(&self) -> Post {
        return Post {
            id: self.id,
            created_at: self.created_at,
            content: self.content.clone(),
            parent_id: self.parent_id,
            creator_id: self.creator_id,
            creator_name: self.facebook_users.name.clone(),
            creator_picture: self.facebook_users.profile_picture.clone()
        };
    }
}

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
        match self
            .client
            .from(TABLE)
            .select("*, facebookUsers!inner(name, profile_picture)")
            .order("created_at")
            .limit(20)
            .execute()
            .await
        {
            Ok(r) => match r.text().await {
                Ok(t) => {
                    let body: Result<Vec<JoinPost>, serde_json::Error> = serde_json::from_str(&t);
                    match body {
                        Ok(b) => Ok(b.iter().map(|post| post.convert_to_post()).collect()),
                        Err(e) => {
                            eprintln!("Error parsing posts: {}", e);
                            Err("Could not read posts".to_string())
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error reading posts: {}", e);
                    Err("Could not read posts".to_string())
                }
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

#[cfg(test)]
mod tests {
    use crate::repository::{post::PostRepository, supabase::SupabaseRepo};

    #[tokio::test]
    async fn play_around_with_joins() {
        dotenv::dotenv().ok();
        let supabase_url = std::env::var("SUPABASE_URL").expect("SUPABASE_URL must be set.");
        let supabase_api_key =
            std::env::var("SUPABASE_API_KEY").expect("SUPABASE_API_KEY must be set.");
        let repo = SupabaseRepo::new(&supabase_url, &supabase_api_key);

        let posts = repo.read_all().await.unwrap();
        println!("Posts: {:?}", posts);
    }
}
