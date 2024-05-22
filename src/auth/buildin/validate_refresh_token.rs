use crate::{
    auth::auth_handler_enum::ValidateRequestError,
    database::{repo::Repository, user::SearchUser},
    model::{token_claim::TokenClaims, user::User},
};

#[tracing::instrument(skip(database), level = "debug")]
pub async fn validate_refresh_token_handler(
    database: &(dyn Repository<User, SearchUser> + Sync),
    token: String,
    target_auth_type: String,
    refresh: bool,
    claim: TokenClaims,
) -> Result<User, ValidateRequestError> {
    let user = match database
        .find_one(SearchUser::username(claim.username.clone()))
        .await
    {
        Ok(user) => {
            tracing::debug!("User found: {:?}", user);
            user
        }
        Err(err) => {
            tracing::error!("Error while searching for user: {:?}", err);
            return Err(ValidateRequestError::Unknown(
                "Token does not exist".to_string(),
            ));
        }
    };
    match user.clone().auth_type {
        Some(auth_type) => {
            if auth_type != target_auth_type {
                return Err(ValidateRequestError::InvalidData(
                    "Please use the auth method that you used to register".to_string(),
                ));
            }
        }
        None => {
            tracing::error!("User has no auth type");
            return Err(ValidateRequestError::InvalidData(
                "Please contact your administrator".to_string(),
            ));
        }
    }
    if refresh && !user.token.contains(&token) {
        return Err(ValidateRequestError::InvalidData(
            "Token does not exist".to_string(),
        ));
    }
    Ok(user)
}
