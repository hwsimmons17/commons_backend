use std::collections::BTreeMap;

use base64::{engine::general_purpose, Engine};
use chrono::Utc;
use hmac::Hmac;
use jwt::VerifyWithKey;
use sha2::Sha256;

#[derive(Clone)]
pub struct OAuth {
    key: Hmac<Sha256>,
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
}

impl OAuth {
    pub fn new(private_key: String) -> Self {
        let key: Hmac<Sha256> =
            hmac::Mac::new_from_slice(&general_purpose::STANDARD.decode(private_key).unwrap())
                .expect("Error encoding private key to hmac");

        Self { key }
    }

    pub fn verify_header(&self, auth_header: &str) -> Result<User, String> {
        let claims: BTreeMap<String, String>;
        match auth_header.verify_with_key(&self.key) {
            Ok(c) => claims = c,
            Err(e) => {
                eprintln!("{}", e);
                return Err("Error verifying token string".to_string());
            }
        };

        match claims["expiry"].parse::<i64>() {
            Ok(e) => {
                let now = Utc::now();
                if now.timestamp_millis() > e * 1_000 {
                    return Err("Token expired".to_string());
                }
            }
            Err(_) => return Err("Error parsing expiry".to_string()),
        }

        let id: u64;
        match claims["id"].parse() {
            Ok(i) => id = i,
            Err(_) => return Err("Error parsing id".to_string()),
        };

        let name = claims["name"].clone();
        if name == "" {
            return Err("Name cannot be empty".to_string());
        };

        let email = claims["email"].clone();
        if email == "" {
            return Err("Email cannot be empty".to_string());
        }

        Ok(User { id, name, email })
    }
}

#[cfg(test)]
mod tests {
    use super::OAuth;

    #[test]
    fn new_saves_key() {
        dotenv::dotenv().ok();
        let private_key =
            std::env::var("SUPABASE_JWT_SECRET").expect("SUPABASE_JWT_SECRET must be set.");

        OAuth::new(private_key);
    }

    #[test]
    fn verify_header_works() {
        dotenv::dotenv().ok();
        let private_key =
            std::env::var("SUPABASE_JWT_SECRET").expect("SUPABASE_JWT_SECRET must be set.");
        let oauth = OAuth::new(private_key);

        let auth_header = "eyJhbGciOiJIUzI1NiIsInR5cGUiOiJKV1QifQ.eyJlbWFpbCI6Imh1bnRlcndzaW1tb25zQGdtYWlsLmNvbSIsIm5hbWUiOiJIdW50ZXIgU2ltbW9ucyIsImV4cGlyeSI6IjE2ODQ3ODg5NTUiLCJpZCI6IjE0MzY2MjUyNTcwODY1NzEifQ.tgVqB6cXX4CdpIuXvYwuydHYT7wds4hR0OcIQ1TRASs".to_string();

        oauth.verify_header(&auth_header).unwrap();
    }
}
