use repository::user::UserRepository;
use router::create_router;
use std::net::SocketAddr;

pub mod app_state;
pub mod repository;
pub mod router;
pub mod routes;

pub async fn run<U: UserRepository>(user_repo: U) {
    let app = create_router(user_repo);
    let port = std::env::var("PORT").unwrap_or("8080".to_string());
    let address = SocketAddr::from(([0, 0, 0, 0], port.parse().unwrap()));

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
