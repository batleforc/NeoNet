use async_trait::async_trait;

use crate::{
    config::AuthConfig,
    database::{repo::Repository, user::SearchUser, PersistenceConfig},
    model::{
        role::Role,
        user::{CreateUser, User},
    },
};

use super::auth_handler_enum::{
    CallbackRequestError, ConfigValidateError, LoginRequestError, LoginRequestRetun,
    LogoutRequestError, RefreshRequestError, RegisterRequestError, ValidateRequestError,
};

#[async_trait]
pub trait AuthHandler<R>
where
    R: Repository<User, SearchUser, dyn PersistenceConfig> + ?Sized + Sync,
{
    // The name of the authentication handler
    fn get_name(&self) -> String;

    // The version of the authentication handler
    fn get_version(&self) -> String;

    // The description of the authentication handler
    fn get_description(&self) -> String;

    // Return the kind of authentication handler
    fn get_kind(&self) -> String;

    // If the authentication handler should have a ZKP aknowledgement before registering
    fn register_require_zkp(&self) -> bool;

    // The init config function that will be called at the start of the service to make sure that the configuration is correct and instanciate the handler
    // This function should return an error if the configuration is not correct
    // This function should return Ok(()) if the configuration is correct
    async fn init_config(
        &mut self,
        config: AuthConfig,
        app_name: String,
    ) -> Result<(), ConfigValidateError>;

    // The login function that will be called when the user tries to login
    // It username and password can be empty strings
    // The function has the responsibility to validate the username and password
    // and return a refresh token if the validation is successful
    async fn login(
        &self,
        database: &R,
        username: String,
        password: String,
    ) -> Result<LoginRequestRetun, LoginRequestError>;

    // The callback function that will be called when the user tries to login
    // using a third-party service
    // The path used will be like /api/auth/callback/{name of the handler}
    // This function should return the token if the authentication is successful
    async fn callback(&self, database: &R, code: String) -> Result<String, CallbackRequestError>;

    // The register function that will be called when the user tries to register
    // The function should return whether the registration was successful or not
    // This function could be called by the callback function if the user is not found
    // ! The username should be unique and unchangeable
    async fn register(
        &self,
        database: &R,
        user: CreateUser,
        role: Role,
    ) -> Result<(), RegisterRequestError>;

    // The logout function that will be called when the user tries to logout
    async fn logout(&self, database: &R, token: String) -> Result<(), LogoutRequestError>;

    // The validate function that will be called every time the user tries to access a protected route
    // The function should return the user information if the token is valid
    async fn validate(
        &self,
        database: &R,
        token: String,
        refresh: bool,
    ) -> Result<User, ValidateRequestError>;

    // The refresh function that will be called when the user tries to refresh the token
    // The function should return a new access token if the refresh is successful
    async fn refresh(&self, database: &R, token: String) -> Result<String, RefreshRequestError>;
}
