pub mod user;

pub struct SupabaseRepo {
    client: postgrest::Postgrest,
}

impl SupabaseRepo {
    pub fn new(url: &str, api_key: &str) -> Self {
        let client = postgrest::Postgrest::new(url)
            .insert_header("apikey", api_key)
            .insert_header("Authorization", format!("Bearer {}", api_key));
        SupabaseRepo { client }
    }
}
