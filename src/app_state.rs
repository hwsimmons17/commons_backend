use crate::{
    oauth::OAuth,
    repository::{post::DynPostRepo, user::DynUserRepo},
};

#[derive(Clone)]
pub struct AppState {
    pub user_repo: DynUserRepo,
    pub post_repo: DynPostRepo,
    pub oauth: OAuth,
}
