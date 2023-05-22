use axum::{
    extract::State,
    http::{self, Request},
    middleware::Next,
    response::Response,
};
use reqwest::StatusCode;

use crate::{app_state::AppState, oauth::User};

pub async fn auth<B>(
    State(app_state): State<AppState>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());
    let auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let user: User;
    match app_state.oauth.verify_header(auth_header) {
        Ok(u) => user = u,
        Err(e) => {
            eprintln!("Error in auth middleware: {}", e);
            return Err(StatusCode::UNAUTHORIZED);
        }
    };
    req.extensions_mut().insert(user);

    Ok(next.run(req).await)
}
