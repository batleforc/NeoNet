use serde::{Deserialize, Serialize};

use crate::{database::SearchEntity, model::role::Role};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SearchUser {
    pub username: Option<String>,
    pub role: Option<Vec<Role>>,
    pub enabled: Option<bool>,
    pub auth_type: Option<Vec<String>>,
}

impl SearchEntity for SearchUser {}
