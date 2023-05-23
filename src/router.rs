use std::sync::Arc;

use crate::{
    app_state::AppState,
    oauth::OAuth,
    repository::{
        post::{DynPostRepo, PostRepository},
        user::{DynUserRepo, UserRepository},
    },
    routes::{
        auth::auth,
        posts::{create_post, get_posts},
        users::create_user,
    },
};
use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use tokio::sync::Mutex;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

pub fn create_router<U, P>(user_repo: U, post_repo: P, oauth: OAuth) -> Router
where
    U: UserRepository,
    P: PostRepository,
{
    let user_repo = Arc::new(Mutex::new(user_repo)) as DynUserRepo;
    let post_repo = Arc::new(Mutex::new(post_repo)) as DynPostRepo;
    let app_state = AppState {
        user_repo,
        post_repo,
        oauth,
    };

    Router::new()
        .route("/posts", post(create_post))
        .route("/posts", get(get_posts))
        .layer(middleware::from_fn_with_state(app_state.clone(), auth))
        .route("/user", post(create_user))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(app_state)
}
