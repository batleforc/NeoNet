use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::database::Entity;

use super::role::Role;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub role: Role,
    pub enabled: bool,
    pub username: String,
    pub password: String,
    pub profile_picture: Option<String>,
    pub description: Option<String>,
    pub humeur: Option<String>,
    pub auth_type: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub token: Vec<String>,
}

impl Entity for User {}

impl User {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: String,
        role: Role,
        enabled: bool,
        username: String,
        password: String,
        profile_picture: Option<String>,
        description: Option<String>,
        humeur: Option<String>,
        auth_type: Option<String>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        token: Vec<String>,
    ) -> Self {
        User {
            id,
            role,
            enabled,
            username,
            password,
            profile_picture,
            description,
            humeur,
            auth_type,
            created_at,
            updated_at,
            token,
        }
    }
    pub fn set_password(&mut self, password: &str) {
        self.password = bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap();
    }
    pub fn check_password(&self, password: &str) -> bool {
        bcrypt::verify(password, &self.password).unwrap_or(false)
    }
}
