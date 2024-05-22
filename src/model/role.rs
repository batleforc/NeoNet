use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub enum Role {
    Owner,
    Admin,
    Moderator,
    User,
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::Owner => write!(f, "Owner"),
            Role::Admin => write!(f, "Admin"),
            Role::Moderator => write!(f, "Moderator"),
            Role::User => write!(f, "User"),
        }
    }
}

impl TryInto<Role> for String {
    type Error = ();

    fn try_into(self) -> Result<Role, Self::Error> {
        match self.as_str() {
            "Owner" => Ok(Role::Owner),
            "Admin" => Ok(Role::Admin),
            "Moderator" => Ok(Role::Moderator),
            "User" => Ok(Role::User),
            _ => Err(()),
        }
    }
}
