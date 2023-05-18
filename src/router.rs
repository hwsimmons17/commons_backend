use std::sync::Arc;

use crate::{
    app_state::AppState, repository::user::{UserRepository, DynUserRepo}, routes::users::create_user,
};
use axum::{
    routing::{post},
    Router,
};
use tokio::sync::Mutex;

pub fn create_router<U>(
    user_repo: U,
) -> Router
where
    U: UserRepository,
{
    let user_repo = Arc::new(Mutex::new(user_repo)) as DynUserRepo;
    let app_state = AppState {
        user_repo,
    };

    Router::new()
        .route("/user", post(create_user))
        .with_state(app_state)
}
