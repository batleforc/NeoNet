use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub enum Role {
    Owner,
    Admin,
    Moderator,
    User,
}

impl Role {
    pub fn from_str(role: &str) -> Option<Role> {
        match role {
            "Owner" => Some(Role::Owner),
            "Admin" => Some(Role::Admin),
            "Moderator" => Some(Role::Moderator),
            "User" => Some(Role::User),
            _ => None,
        }
    }
}

impl ToString for Role {
    fn to_string(&self) -> String {
        match self {
            Role::Owner => "Owner".to_string(),
            Role::Admin => "Admin".to_string(),
            Role::Moderator => "Moderator".to_string(),
            Role::User => "User".to_string(),
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
