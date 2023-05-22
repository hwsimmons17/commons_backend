use oauth::OAuth;
use repository::{post::PostRepository, user::UserRepository};
use router::create_router;
use std::net::SocketAddr;

pub mod app_state;
pub mod oauth;
pub mod repository;
pub mod router;
pub mod routes;

pub async fn run<U: UserRepository, P: PostRepository>(user_repo: U, post_repo: P, oauth: OAuth) {
    let app = create_router(user_repo, post_repo, oauth);
    let port = std::env::var("PORT").unwrap_or("8080".to_string());
    let address = SocketAddr::from(([0, 0, 0, 0], port.parse().unwrap()));

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
