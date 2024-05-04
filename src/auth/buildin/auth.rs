use async_trait::async_trait;

use crate::{
    auth::{
        auth_handler::AuthHandler,
        auth_handler_enum::{
            CallbackRequestError, LoginRequestError, LoginRequestRetun, LogoutRequestError,
            RefreshRequestError, RegisterRequestError, ValidateRequestError,
        },
    },
    database::{repo::Repository, user::SearchUser, PersistenceConfig},
    model::user::User,
};

#[derive(Debug, Clone)]
pub struct BuildInAuthHandler {
    pub enabled: bool,
}

#[async_trait]
impl AuthHandler<dyn Repository<User, SearchUser, dyn PersistenceConfig>> for BuildInAuthHandler {
    fn get_name(&self) -> String {
        "BuildInAuthHandler".to_string()
    }

    fn get_version(&self) -> String {
        "0.1.0".to_string()
    }

    fn get_description(&self) -> String {
        "Authentication handler that uses a build-in user database".to_string()
    }

    fn is_enabled(&self) -> bool {
        self.enabled
    }

    async fn login(
        &self,
        _database: &dyn Repository<User, SearchUser, dyn PersistenceConfig>,
        _username: String,
        _password: String,
    ) -> Result<LoginRequestRetun, LoginRequestError> {
        Err(LoginRequestError::Unknown("Not implemented".to_string()))
    }

    async fn callback(
        &self,
        _database: &dyn Repository<User, SearchUser, dyn PersistenceConfig>,
        _code: String,
    ) -> Result<String, CallbackRequestError> {
        Err(CallbackRequestError::Unknown("Not implemented".to_string()))
    }

    async fn register(
        &self,
        _database: &dyn Repository<User, SearchUser, dyn PersistenceConfig>,
        _user: User,
    ) -> Result<(), RegisterRequestError> {
        Err(RegisterRequestError::Unknown("Not implemented".to_string()))
    }

    async fn logout(
        &self,
        _database: &dyn Repository<User, SearchUser, dyn PersistenceConfig>,
        _token: String,
    ) -> Result<(), LogoutRequestError> {
        Err(LogoutRequestError::Unknown("Not implemented".to_string()))
    }

    async fn validate(
        &self,
        _database: &dyn Repository<User, SearchUser, dyn PersistenceConfig>,
        _token: String,
    ) -> Result<User, ValidateRequestError> {
        Err(ValidateRequestError::Unknown("Not implemented".to_string()))
    }

    async fn refresh(
        &self,
        _database: &dyn Repository<User, SearchUser, dyn PersistenceConfig>,
        _token: String,
    ) -> Result<String, RefreshRequestError> {
        Err(RefreshRequestError::Unknown("Not implemented".to_string()))
    }
}
