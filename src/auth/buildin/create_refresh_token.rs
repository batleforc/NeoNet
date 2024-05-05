use crate::{
    database::{repo::Repository, user::SearchUser, PersistenceConfig},
    model::{
        token_claim::{TokenClaims, TokenConfig, TokenError},
        user::User,
    },
};
use std::fmt::Display;
pub enum CreateRefreshTokenError {
    DatabaseError(String),
    TokenError(TokenError),
}

impl Display for CreateRefreshTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateRefreshTokenError::DatabaseError(e) => write!(f, "Database error: {}", e),
            CreateRefreshTokenError::TokenError(e) => write!(f, "Token error: {}", e),
        }
    }
}

#[tracing::instrument(skip(database), level = "debug")]
pub async fn create_refresh_token(
    database: &(dyn Repository<User, SearchUser, dyn PersistenceConfig> + Sync),
    user: &mut User,
    token_config: TokenConfig,
    app_name: String,
) -> Result<String, CreateRefreshTokenError> {
    let mut token_claim = TokenClaims::new(
        user.id.clone(),
        user.username.clone(),
        app_name,
        true,
        token_config.clone(),
    );
    let token = match token_claim.sign_token(token_config) {
        Ok(token) => token,
        Err(e) => return Err(CreateRefreshTokenError::TokenError(e)),
    };
    user.token.push(token.clone());
    match database.update(user.clone()).await {
        Ok(_) => {}
        Err(e) => return Err(CreateRefreshTokenError::DatabaseError(e.to_string())),
    }
    Ok(token)
}
