use crate::repository::user::DynUserRepo;

#[derive(Clone)]
pub struct AppState {
    pub user_repo: DynUserRepo,
}
