use async_trait::async_trait;

use crate::{
    database::{repo::Repository, user::SearchUser, PersistenceConfig},
    model::user::User,
};

use super::auth_handler_enum::{
    CallbackRequestError, LoginRequestError, LoginRequestRetun, LogoutRequestError,
    RefreshRequestError, ValidateRequestError,
};

#[async_trait]
pub trait AuthHandler<R>
where
    R: Repository<User, SearchUser, dyn PersistenceConfig>,
{
    // The name of the authentication handler
    fn get_name(&self) -> String;

    // The login function that will be called when the user tries to login
    // It username and password can be empty strings
    // The function has the responsibility to validate the username and password
    // and return a token if the validation is successful
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

    // The logout function that will be called when the user tries to logout
    async fn logout(&self, database: &R, token: String) -> Result<(), LogoutRequestError>;

    // The validate function that will be called every time the user tries to access a protected route
    // The function should return the user information if the token is valid
    async fn validate(&self, database: &R, token: String) -> Result<User, ValidateRequestError>;

    // The refresh function that will be called when the user tries to refresh the token
    async fn refresh(&self, database: &R, token: String) -> Result<String, RefreshRequestError>;
}
