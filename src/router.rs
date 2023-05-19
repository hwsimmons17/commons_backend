use std::sync::Arc;

use crate::{
    app_state::AppState,
    repository::user::{DynUserRepo, UserRepository},
    routes::users::create_user,
};
use axum::{routing::post, Router};
use reqwest::Method;
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};

pub fn create_router<U>(user_repo: U) -> Router
where
    U: UserRepository,
{
    let user_repo = Arc::new(Mutex::new(user_repo)) as DynUserRepo;
    let app_state = AppState { user_repo };

    Router::new()
        .route("/user", post(create_user))
        .layer(
            CorsLayer::new()
                .allow_methods([Method::GET, Method::POST])
                .allow_origin(Any),
        )
        .with_state(app_state)
}
