use std::str::FromStr;

use chrono::{DateTime, Utc};
use mongodb::bson::{doc, oid::ObjectId, Document};
use serde::{Deserialize, Serialize};

use crate::{
    database::SearchEntity,
    model::{role::Role, user::User},
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SearchUser {
    pub username: Option<String>,
    pub role: Option<Vec<Role>>,
    pub enabled: Option<bool>,
    pub auth_type: Option<Vec<String>>,
}

impl SearchEntity for SearchUser {}

impl SearchUser {
    #[tracing::instrument(level = "trace")]
    pub fn turn_into_search(&self) -> Option<Document> {
        let mut search = doc! {};
        if self == &SearchUser::default() {
            return None;
        }
        if let Some(username) = &self.username {
            if username.starts_with('/') || username.ends_with('/') {
                search.insert(
                    "username",
                    doc! { "$regex": username.replace('/', ""), "$options": "i" },
                );
            } else {
                search.insert("username", username);
            }
        }
        if let Some(role) = &self.role {
            search.insert(
                "role",
                doc! { "$in": role.iter().map(|r| r.to_string()).collect::<Vec<String>>()},
            );
        }
        if let Some(enabled) = &self.enabled {
            search.insert("enabled", enabled);
        }
        if let Some(auth_type) = &self.auth_type {
            search.insert("auth_type", doc! { "$in": auth_type });
        }
        Some(search)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMongo {
    pub _id: ObjectId,
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
}

impl UserMongo {
    pub fn get_collection(database: mongodb::Database) -> mongodb::Collection<UserMongo> {
        database.collection("user")
    }
}

impl From<User> for UserMongo {
    fn from(user: User) -> Self {
        UserMongo {
            _id: ObjectId::from_str(user.id.as_str()).unwrap(),
            role: user.role,
            enabled: user.enabled,
            username: user.username,
            password: user.password,
            profile_picture: user.profile_picture,
            description: user.description,
            humeur: user.humeur,
            auth_type: user.auth_type,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

impl TryInto<User> for UserMongo {
    type Error = ();

    fn try_into(self) -> Result<User, Self::Error> {
        Ok(User {
            id: self._id.to_string(),
            role: self.role,
            enabled: self.enabled,
            username: self.username,
            password: self.password,
            profile_picture: self.profile_picture,
            description: self.description,
            humeur: self.humeur,
            auth_type: self.auth_type,
            created_at: self.created_at,
            updated_at: self.updated_at,
        })
    }
}
