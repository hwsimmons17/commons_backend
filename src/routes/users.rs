use axum::{extract::State, Json};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{app_state::AppState, repository::user::User};

#[derive(Serialize, Deserialize)]
pub struct PostUserRequest {
    email: String,
    name: String,
    picture: String,
    id: String,
}

#[axum_macros::debug_handler]
pub async fn create_user(
    State(app_state): State<AppState>,
    Json(payload): Json<PostUserRequest>,
) -> Result<(), (StatusCode, String)> {
    let id: u64;
    match payload.id.parse() {
        Ok(i) => id = i,
        Err(_) => return Err((StatusCode::BAD_REQUEST, "ID is not correct".to_string())),
    }
    if id == 0 {
        return Err((StatusCode::BAD_REQUEST, "ID is empty".to_string()));
    }
    if payload.email == "" {
        return Err((StatusCode::BAD_REQUEST, "Email is empty".to_string()));
    }
    if payload.name == "" {
        return Err((StatusCode::BAD_REQUEST, "Name is empty".to_string()));
    }
    if payload.picture == "" {
        return Err((StatusCode::BAD_REQUEST, "Picture is empty".to_string()));
    }

    let user = User {
        email: payload.email,
        name: payload.name,
        picture: payload.picture,
        id,
    };

    match app_state.user_repo.lock().await.create(user).await {
        Ok(_) => return Ok(()),
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error saving user".to_string(),
            ))
        }
    }
}
