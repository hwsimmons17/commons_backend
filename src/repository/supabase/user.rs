use axum::async_trait;
use reqwest::StatusCode;

use crate::repository::user::{User, UserRepository};

use super::SupabaseRepo;

#[async_trait]
impl UserRepository for SupabaseRepo {
    async fn create(&mut self, user: User) -> Result<User, String> {
        match self
            .client
            .from("facebookUsers")
            .insert(format!(
                r#"[{{"person_id": "{}", "name": "{}", "email": "{}", "profile_picture": "{}"}}]"#,
                user.id, user.name, user.email, user.picture
            ))
            .execute()
            .await
        {
            Ok(r) => {
                if r.status() == StatusCode::CREATED {
                    return Ok(user);
                }
                if r.status() == StatusCode::CONFLICT {
                    return self.update(user).await;
                }

                println!("Error creating user: {:?}", r.text().await);
                return Err("User not created".to_string());
            }
            Err(_) => return Err("User not created".to_string()),
        }
    }

    async fn read(&self, phone_number: u64) -> Result<Vec<User>, String> {
        match self
            .client
            .from("facebookUsers")
            .eq("phone_number", phone_number.to_string())
            .select("*")
            .execute()
            .await
        {
            Ok(r) => match r.text().await {
                Ok(t) => {
                    let body: Result<Vec<User>, serde_json::Error> = serde_json::from_str(&t);
                    match body {
                        Ok(b) => Ok(b),
                        Err(_) => Err("Could not read users".to_string()),
                    }
                }
                Err(_) => Err("Could not read users".to_string()),
            },
            Err(_) => return Err("Could not read users".to_string()),
        }
    }

    async fn update(&mut self, user: User) -> Result<User, String> {
        match self
            .client
            .from("facebookUsers")
            .eq("id", user.id.to_string())
            .update(format!(
                r#"[{{"person_id": "{}", "name": "{}", "email": "{}", "profile_picture": "{}"}}]"#,
                user.id, user.name, user.email, user.picture
            ))
            .execute()
            .await
        {
            Ok(r) => {
                if r.status() == StatusCode::OK {
                    return Ok(user);
                }
                return Err("User not updated".to_string());
            }
            Err(_) => return Err("User not updated".to_string()),
        }
    }

    async fn delete(&mut self, id: u64) -> Result<Option<User>, String> {
        match self
            .client
            .from("facebookUsers")
            .eq("id", id.to_string())
            .delete()
            .execute()
            .await
        {
            Ok(r) => match r.text().await {
                Ok(t) => {
                    let body: Result<Vec<User>, serde_json::Error> = serde_json::from_str(&t);
                    match body {
                        Ok(users) => {
                            if users.len() == 0 {
                                return Err("User not deleted".to_string());
                            }
                            return Ok(Some(users[0].clone()));
                        }
                        Err(_) => Err("User not deleted".to_string()),
                    }
                }
                Err(_) => Err("User not deleted".to_string()),
            },
            Err(_) => return Err("User not deleted".to_string()),
        }
    }
}

fn unwrap_read_user(res: Result<Vec<User>, String>) -> Result<User, String> {
    match res {
        Ok(users) => {
            if users.len() == 0 {
                return Err("Expected len of users to be greater than 0".to_string());
            }
            return Ok(users[0].clone());
        }
        Err(e) => Err(e),
    }
}
