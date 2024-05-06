use serde::{Deserialize, Serialize};

use crate::{database::SearchEntity, model::role::Role};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SearchUser {
    pub username: Option<String>,
    pub role: Option<Vec<Role>>,
    pub enabled: Option<bool>,
    pub auth_type: Option<Vec<String>>,
    pub token: Option<String>,
}

impl SearchUser {
    pub fn username(username: String) -> Self {
        SearchUser {
            username: Some(username),
            ..Default::default()
        }
    }
    pub fn role(role: Vec<Role>) -> Self {
        SearchUser {
            role: Some(role),
            ..Default::default()
        }
    }
    pub fn enabled(enabled: bool) -> Self {
        SearchUser {
            enabled: Some(enabled),
            ..Default::default()
        }
    }
    pub fn auth_type(auth_type: Vec<String>) -> Self {
        SearchUser {
            auth_type: Some(auth_type),
            ..Default::default()
        }
    }
    pub fn token(token: String) -> Self {
        SearchUser {
            token: Some(token),
            ..Default::default()
        }
    }
}

impl SearchEntity for SearchUser {}
