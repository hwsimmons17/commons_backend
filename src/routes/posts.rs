use axum::{extract::State, Extension, Json};
use chrono::Utc;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{app_state::AppState, oauth::User, repository::post::Post};

#[derive(Serialize, Deserialize)]
pub struct CreatePostRequest {
    content: String,
}

pub async fn create_post(
    State(app_state): State<AppState>,
    Extension(user): Extension<User>,
    Json(payload): Json<CreatePostRequest>,
) -> Result<(), (StatusCode, String)> {
    if payload.content == "" {
        return Err((StatusCode::BAD_REQUEST, "Content is empty".to_string()));
    }

    let post = Post {
        id: Uuid::new_v4(),
        created_at: Utc::now(),
        content: payload.content,
        parent_id: None,
        creator_id: user.id,
    };

    match app_state.post_repo.lock().await.create(post).await {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Error creating post: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error creating post".to_string(),
            ))
        }
    }
}
