use repository::user::UserRepository;
use router::create_router;
use std::net::SocketAddr;


pub mod app_state;
pub mod router;
pub mod repository;
pub mod routes;



pub async fn run<U: UserRepository>(
    user_repo: U,
) {
    let app = create_router(user_repo);
    let address = SocketAddr::from(([0, 0, 0, 0], 8080));

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}