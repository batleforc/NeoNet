use crate::{
    auth::auth_handler_enum::LogoutRequestError,
    database::{repo::Repository, user::SearchUser},
    model::{
        token_claim::{TokenClaims, TokenConfig},
        user::User,
    },
};

#[tracing::instrument(skip(database), level = "debug")]
pub async fn logout_handler(
    database: &(dyn Repository<User, SearchUser> + Sync),
    token: String,
    auth_type: String,
    config: TokenConfig,
) -> Result<(), LogoutRequestError> {
    let mut user = match database.find_one(SearchUser::token(token.clone())).await {
        Ok(user) => {
            tracing::debug!("User found: {:?}", user);
            user
        }
        Err(err) => {
            tracing::error!("Error while searching for user: {:?}", err);
            return Err(LogoutRequestError::Unknown(
                "Token does not exist".to_string(),
            ));
        }
    };
    match user.clone().auth_type {
        Some(auth_type) => {
            if auth_type != auth_type {
                return Err(LogoutRequestError::InvalidData(
                    "Please use the auth method that you used to register".to_string(),
                ));
            }
        }
        None => {
            tracing::error!("User has no auth type");
            return Err(LogoutRequestError::InvalidData(
                "Please contact your administrator".to_string(),
            ));
        }
    }

    // Delete token
    user.token.retain(|t| {
        if t == &token || t == "" {
            return false;
        }
        match TokenClaims::validate_token(t.clone(), true, config.clone()) {
            Ok(_) => true,
            Err(_) => false,
        }
    });
    match database.update(user).await {
        Ok(_) => {
            tracing::info!("User logged out successfully");
        }
        Err(err) => {
            tracing::error!("Error while updating user: {:?}", err);
            return Err(LogoutRequestError::Unknown(
                "Error while updating user".to_string(),
            ));
        }
    }
    Ok(())
}
