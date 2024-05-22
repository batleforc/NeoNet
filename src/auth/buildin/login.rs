use crate::{
    auth::auth_handler_enum::LoginRequestError,
    database::{repo::Repository, user::SearchUser},
    model::user::User,
};

#[tracing::instrument(skip(database, password), level = "debug")]
pub async fn login_handler(
    database: &(dyn Repository<User, SearchUser> + Sync),
    username: String,
    password: String,
    target_auth_type: String,
) -> Result<User, LoginRequestError> {
    let user = match database.find_one(SearchUser::username(username)).await {
        Ok(user) => {
            tracing::debug!("User found: {:?}", user);
            user
        }
        Err(err) => {
            tracing::error!("Error while searching for user: {:?}", err);
            return Err(LoginRequestError::Unknown(
                "Error while searching for user".to_string(),
            ));
        }
    };
    match user.clone().auth_type {
        Some(auth_type) => {
            if auth_type != target_auth_type {
                return Err(LoginRequestError::Unauthorized(
                    "Please use the auth method that you used to register".to_string(),
                ));
            }
        }
        None => {
            tracing::error!("User has no auth type");
            return Err(LoginRequestError::Unauthorized(
                "Please contact your administrator".to_string(),
            ));
        }
    }
    if user.check_password(password) {
        tracing::info!("User logged in successfully");
        Ok(user)
    } else {
        tracing::error!("Invalid password");
        Err(LoginRequestError::Unauthorized(
            "Invalid password".to_string(),
        ))
    }
}
