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
}

impl SearchEntity for SearchUser {}
