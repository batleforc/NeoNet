use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub enum Role {
    Owner,
    Admin,
    Moderator,
    User,
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
