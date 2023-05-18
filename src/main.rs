use commons_backend::{
     repository::supabase::SupabaseRepo, run,
};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let supabase_url = std::env::var("SUPABASE_URL").expect("SUPABASE_URL must be set.");
    let supabase_api_key =
        std::env::var("SUPABASE_API_KEY").expect("SUPABASE_API_KEY must be set.");

    let user_repo = SupabaseRepo::new(&supabase_url, &supabase_api_key);


    run(user_repo).await
}
