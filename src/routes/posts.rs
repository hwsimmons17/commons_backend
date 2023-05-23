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
        creator_name: user.name,
        creator_picture: user.picture
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

#[axum_macros::debug_handler]
pub async fn get_posts(
    State(app_state): State<AppState>,
    Extension(user): Extension<User>,
) -> Result<Json<Vec<Post>>, (StatusCode, String)> {
    let posts: Vec<Post>;
    match app_state.post_repo.lock().await.read_all().await {
        Ok(p) => posts = p,
        Err(e) => {
            eprintln!("Error geting posts: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error getting posts".to_string(),
            ));
        }
    };

    return Ok(Json::from(posts));
}
