use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::role::Role;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub role: Role,
    pub enabled: bool,
    pub username: String,
    pub password: String,
    pub profile_picture: Option<String>,
    pub description: Option<String>,
    pub humeur: Option<String>,
    pub device: HashMap<String, bool>,
}

impl User {
    pub fn new(
        role: Role,
        enabled: bool,
        username: String,
        password: String,
        profile_picture: Option<String>,
        description: Option<String>,
        humeur: Option<String>,
        device: HashMap<String, bool>,
    ) -> Self {
        User {
            id: uuid::Uuid::new_v4(),
            role,
            enabled,
            username,
            password,
            profile_picture,
            description,
            humeur,
            device,
        }
    }
    pub fn set_password(&mut self, password: &str) {
        self.password = bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap();
    }
    pub fn check_password(&self, password: &str) -> bool {
        match bcrypt::verify(password, &self.password) {
            Ok(result) => result,
            Err(_) => false,
        }
    }
}
