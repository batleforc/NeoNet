use crate::{
    auth::{
        auth_handler::AuthHandler,
        auth_handler_enum::{
            CallbackRequestError, ConfigValidateError, LoginRequestError, LoginRequestRetun,
            LogoutRequestError, RefreshRequestError, RegisterRequestError, ValidateRequestError,
        },
    },
    config::AuthConfig,
    database::{
        self, repo::Repository, repo_error::RepoSelectError, user::SearchUser, PersistenceConfig,
    },
    model::{
        role::Role,
        token_claim::TokenConfig,
        user::{CreateUser, User},
    },
};
use async_trait::async_trait;
use tracing::{debug, error, info};

use super::{create_refresh_token::create_refresh_token, login};

// The authentication handler that uses a build-in user database
// The handler will use the User model to store the user information
// The handler will use the TokenConfig to generate the token
// The TokenConfig requires the access_token_sign, refresh_token_sign, access_token_exp, and refresh_token_exp
// The _exp fields are the expiration time in seconds
#[derive(Debug, Clone)]
pub struct BuildInAuthHandler {
    pub require_zkp: bool,
    pub token_config: TokenConfig,
    pub description: String,
    pub app_name: String,
}

#[async_trait]
impl AuthHandler<dyn Repository<User, SearchUser, dyn PersistenceConfig> + Sync>
    for BuildInAuthHandler
{
    fn get_name(&self) -> String {
        "buildin".to_string()
    }

    fn get_version(&self) -> String {
        "0.1.0".to_string()
    }

    fn get_description(&self) -> String {
        self.description.clone()
    }

    fn register_require_zkp(&self) -> bool {
        self.require_zkp
    }

    fn get_kind(&self) -> String {
        "build-in".to_string()
    }

    #[tracing::instrument(level = "debug", skip(config))]
    async fn init_config(
        &mut self,
        config: AuthConfig,
        app_name: String,
    ) -> Result<(), ConfigValidateError> {
        if !config.enabled {
            error!("The authentication handler is not enabled and should not be instanciated");
            return Err(ConfigValidateError::InvalidData(
                "The authentication handler is not enabled and should not be instanciated"
                    .to_string(),
            ));
        }
        if config.kind != self.get_kind() {
            error!("The authentication handler kind is not build-in");
            return Err(ConfigValidateError::InvalidData(
                "The authentication handler kind is not build-in".to_string(),
            ));
        }
        if config.version != "" && config.version != self.get_version() {
            error!("The authentication handler version does not match");
            return Err(ConfigValidateError::InvalidData(
                "The authentication handler version does not match".to_string(),
            ));
        }
        let mut token_config = TokenConfig::default();

        if let Some(refresh_token_sign) = config.extra_fields.get("refresh_token_sign") {
            token_config.refresh_token_sign = refresh_token_sign.to_string();
        } else {
            error!("The refresh_token_sign is missing from the configuration");
            return Err(ConfigValidateError::InvalidData(
                "The refresh_token_sign is missing from the configuration".to_string(),
            ));
        }

        if let Some(access_token_sign) = config.extra_fields.get("access_token_sign") {
            token_config.access_token_sign = access_token_sign.to_string();
        } else {
            error!("The access_token_sign is missing from the configuration");
            return Err(ConfigValidateError::InvalidData(
                "The access_token_sign is missing from the configuration".to_string(),
            ));
        }

        if let Some(refresh_token_expire) = config.extra_fields.get("refresh_token_expire") {
            match refresh_token_expire.parse::<usize>() {
                Ok(duration) => {
                    if duration < 60 {
                        error!(
                            "The refresh_token_expire is too short, keep it at least 60 seconds"
                        );
                        return Err(ConfigValidateError::InvalidData(
                            "The refresh_token_expire is too short".to_string(),
                        ));
                    }
                    token_config.refresh_token_exp = duration
                }
                Err(err) => {
                    error!("The refresh_token_expire is not a number: {}", err);
                    return Err(ConfigValidateError::InvalidData(
                        "The refresh_token_expire is not a number".to_string(),
                    ));
                }
            }
        } else {
            error!("The refresh_token_expire is missing from the configuration");
            return Err(ConfigValidateError::InvalidData(
                "The refresh_token_expire is missing from the configuration".to_string(),
            ));
        }

        if let Some(access_token_expire) = config.extra_fields.get("access_token_expire") {
            match access_token_expire.parse::<usize>() {
                Ok(duration) => {
                    if duration < 60 {
                        error!("The access_token_expire is too short, keep it at least 60 seconds");
                        return Err(ConfigValidateError::InvalidData(
                            "The access_token_expire is too short".to_string(),
                        ));
                    }
                    token_config.access_token_exp = duration
                }
                Err(err) => {
                    error!("The access_token_expire is not a number: {}", err);
                    return Err(ConfigValidateError::InvalidData(
                        "The access_token_expire is not a number".to_string(),
                    ));
                }
            }
        } else {
            error!("The access_token_expire is missing from the configuration");
            return Err(ConfigValidateError::InvalidData(
                "The access_token_expire is missing from the configuration".to_string(),
            ));
        }

        if token_config.access_token_exp > token_config.refresh_token_exp {
            error!("The access_token_expire should be lower than the refresh_token_expire");
            return Err(ConfigValidateError::InvalidData(
                "The access_token_expire should be lower than the refresh_token_expire".to_string(),
            ));
        }

        self.token_config = token_config;
        // The name of the authentication handler will not be checked
        self.require_zkp = config.require_zkp;
        self.description = config.description.clone();
        self.app_name = app_name;

        Ok(())
    }

    #[tracing::instrument(skip(database, password), level = "debug")]
    async fn login(
        &self,
        database: &(dyn Repository<User, SearchUser, dyn PersistenceConfig> + Sync),
        username: String,
        password: String,
    ) -> Result<LoginRequestRetun, LoginRequestError> {
        if username.is_empty() || password.is_empty() {
            return Err(LoginRequestError::InvalidData(
                "Username and password cannot be empty".to_string(),
            ));
        }
        match login::login_handler(database, username, password, self.get_name()).await {
            Ok(user) => {
                // Generate refresh JWT
                // Save refresh JWT in user
                // Return JWT
                match create_refresh_token(
                    database,
                    &mut user.clone(),
                    self.token_config.clone(),
                    self.app_name.clone(),
                )
                .await
                {
                    Ok(token) => {
                        info!("User logged in: {}", user.username);
                        Ok(LoginRequestRetun::JWT(token))
                    }
                    Err(err) => {
                        error!("Error creating refresh token: {}", err.to_string());
                        Err(LoginRequestError::Unknown(
                            "Error creating refresh token".to_string(),
                        ))
                    }
                }
            }
            Err(err) => Err(err),
        }
    }

    #[tracing::instrument(skip(_database), level = "debug")]
    async fn callback(
        &self,
        _database: &(dyn Repository<User, SearchUser, dyn PersistenceConfig> + Sync),
        _code: String,
    ) -> Result<String, CallbackRequestError> {
        Err(CallbackRequestError::DoesNotSupport)
    }

    #[tracing::instrument(skip(database, user), level = "debug")]
    async fn register(
        &self,
        database: &(dyn Repository<User, SearchUser, dyn PersistenceConfig> + Sync),
        user: CreateUser,
        role: Role,
    ) -> Result<(), RegisterRequestError> {
        info!("Trying to register user: {}", user.username.clone());
        match database
            .find_one(SearchUser::username(user.username.clone()))
            .await
        {
            Ok(_) => {
                error!("User already exists");
                return Err(RegisterRequestError::InvalidData(
                    "User already exists".to_string(),
                ));
            }
            Err(err) => {
                if err != RepoSelectError::NoRowFound {
                    error!("Error finding user: {}", err.to_string());
                    return Err(RegisterRequestError::Unknown(
                        "Error finding user".to_string(),
                    ));
                }
                debug!("User not found, proceeding with registration");
            }
        };
        let mut user = User::new(
            "".to_string(),
            role,
            true,
            user.username.clone(),
            "".to_string(),
            None,
            None,
            None,
            Some(self.get_name()),
            chrono::Utc::now(),
            chrono::Utc::now(),
            vec![],
        );
        user.set_password(user.password.clone());

        match database.create(user.clone()).await {
            Ok(_) => {
                info!("User registered: {}", user.username.clone());
                Ok(())
            }
            Err(err) => {
                error!("Error registering user: {}", err.to_string());
                match err {
                    database::repo_error::RepoCreateError::InvalidData(err) => {
                        Err(RegisterRequestError::InvalidData(err))
                    }
                    database::repo_error::RepoCreateError::Unknown(_) => Err(
                        RegisterRequestError::Unknown("Error registering user".to_string()),
                    ),
                }
            }
        }
    }

    #[tracing::instrument(skip(_database), level = "debug")]
    async fn logout(
        &self,
        _database: &(dyn Repository<User, SearchUser, dyn PersistenceConfig> + Sync),
        _token: String,
    ) -> Result<(), LogoutRequestError> {
        todo!("Implement logout");
    }

    #[tracing::instrument(skip(_database), level = "debug")]
    async fn validate(
        &self,
        _database: &(dyn Repository<User, SearchUser, dyn PersistenceConfig> + Sync),
        _token: String,
        _refresh: bool,
    ) -> Result<User, ValidateRequestError> {
        // Check if the token is valid
        // If refresh is true, check if the token is a refresh token then verify in the database
        todo!("Implement validate");
    }

    #[tracing::instrument(skip(_database), level = "debug")]
    async fn refresh(
        &self,
        _database: &(dyn Repository<User, SearchUser, dyn PersistenceConfig> + Sync),
        _token: String,
    ) -> Result<String, RefreshRequestError> {
        todo!("Implement refresh");
    }
}
