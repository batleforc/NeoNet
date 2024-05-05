use jsonwebtoken::{encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenConfig {
    pub refresh_token_sign: String,
    pub access_token_sign: String,
    pub refresh_token_exp: usize,
    pub access_token_exp: usize,
}

impl Default for TokenConfig {
    fn default() -> Self {
        Self {
            refresh_token_sign: Default::default(),
            access_token_sign: Default::default(),
            refresh_token_exp: Default::default(),
            access_token_exp: Default::default(),
        }
    }
}

impl TokenConfig {
    pub fn new(
        refresh_token_sign: String,
        access_token_sign: String,
        refresh_token_exp: usize,
        access_token_exp: usize,
    ) -> Self {
        Self {
            refresh_token_sign,
            access_token_sign,
            refresh_token_exp,
            access_token_exp,
        }
    }
    pub fn get_key(&self, refresh: bool) -> String {
        match refresh {
            true => self.refresh_token_sign.clone(),
            false => self.access_token_sign.clone(),
        }
    }
    pub fn get_exp(&self, refresh: bool) -> usize {
        match refresh {
            true => self.refresh_token_exp,
            false => self.access_token_exp,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenError {
    InvalidSignToken(String),
    InvalidToken(String),
    WrongTokenType(String),
}

impl std::fmt::Display for TokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenError::InvalidSignToken(msg) => write!(f, "Invalid sign token: {}", msg),
            TokenError::InvalidToken(msg) => write!(f, "Invalid token: {}", msg),
            TokenError::WrongTokenType(msg) => write!(f, "Wrong token type: {}", msg),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, PartialEq)]
pub struct TokenClaims {
    pub sub: String,   // subject
    pub email: String, // email
    pub exp: usize,    // expiration
    pub iat: usize,    // issued at
    pub iss: String,   // issuer
    pub refresh: bool, // is refresh token
}

impl TokenClaims {
    pub fn new(
        sub: String,
        email: String,
        iss: String,
        refresh: bool,
        config: TokenConfig,
    ) -> Self {
        let iat = chrono::Utc::now();
        let exp = iat + chrono::Duration::seconds(config.get_exp(refresh) as i64);
        Self {
            sub,
            email,
            exp: exp.timestamp() as usize,
            iat: iat.timestamp() as usize,
            iss,
            refresh,
        }
    }
    pub fn to_access_token(&self, config: TokenConfig) -> Self {
        Self {
            iat: chrono::Utc::now().timestamp() as usize,
            exp: chrono::Utc::now().timestamp() as usize + config.access_token_exp,
            refresh: false,
            ..self.clone()
        }
    }
    fn gen_header(refresh: bool) -> Header {
        let kid = match refresh {
            true => "refresh_token",
            false => "access_token",
        };
        Header {
            alg: Algorithm::HS512,
            kid: Some(kid.to_string()),
            ..Default::default()
        }
    }

    pub fn sign_token(&mut self, config: TokenConfig) -> Result<String, TokenError> {
        let header = Self::gen_header(self.refresh);
        let key_string = config.get_key(self.refresh);
        let key = key_string.as_bytes();
        match encode(&header, self, &EncodingKey::from_secret(key)) {
            Ok(token) => Ok(token),
            Err(err) => Err(TokenError::InvalidSignToken(err.to_string())),
        }
    }

    pub fn validate_token(
        token: String,
        refresh: bool,
        config: TokenConfig,
    ) -> Result<Self, TokenError> {
        let span = tracing::span!(tracing::Level::INFO, "Token::validate_token");
        let _enter = span.enter();
        let key_string = config.get_key(refresh);
        let key = key_string.as_bytes();
        match jsonwebtoken::decode::<TokenClaims>(
            &token,
            &DecodingKey::from_secret(key),
            &Validation::new(Algorithm::HS512),
        ) {
            Ok(token_data) => {
                if token_data.claims.refresh != refresh {
                    tracing::warn!("Token type does not match");
                    return Err(TokenError::WrongTokenType(
                        "Token type does not match".to_string(),
                    ));
                }

                Ok(token_data.claims)
            }
            Err(err) => {
                tracing::info!("Invalid token: {}", err.to_string());
                Err(TokenError::InvalidToken(err.to_string()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_default_token_claims(
        sub: String,
        email: String,
        iss: String,
        refresh: bool,
        config: TokenConfig,
    ) -> (TokenClaims, usize, usize) {
        let token = TokenClaims::new(sub, email, iss, refresh, config);
        (token.clone(), token.iat, token.exp)
    }

    #[test]
    fn test_token_validate_and_content() {
        let sub = "e95ab377-e32e-54c4-989d-edb163903ac8".to_string();
        let email = "joseph@joestar.com".to_string();
        let iss = "lambda".to_string();
        let token_config = TokenConfig {
            refresh_token_sign: "refresh_token_sign".to_string(),
            access_token_sign: "access_token_sign".to_string(),
            refresh_token_exp: 3600,
            access_token_exp: 3600 * 24 * 7,
        };

        for refresh in vec![true, false].iter() {
            let (mut token_claims, _, _) = init_default_token_claims(
                sub.clone(),
                email.clone(),
                iss.clone(),
                *refresh,
                token_config.clone(),
            );
            let token = match token_claims.sign_token(token_config.clone()) {
                Ok(token) => {
                    assert!(true);
                    token
                }
                Err(_) => {
                    panic!("Failed to sign token")
                }
            };
            let token_claims =
                match TokenClaims::validate_token(token, *refresh, token_config.clone()) {
                    Ok(token_claims) => {
                        assert!(true);
                        token_claims
                    }
                    Err(_) => {
                        panic!("Failed to validate token")
                    }
                };
            assert_eq!(token_claims.sub, sub);
            assert_eq!(token_claims.email, email);
            assert_eq!(token_claims.iss, iss);
            assert_eq!(token_claims.refresh, *refresh);
            match *refresh {
                true => {
                    assert_eq!(token_claims.exp, token_claims.iat + 3600)
                }
                false => {
                    assert_eq!(token_claims.exp, token_claims.iat + 3600 * 24 * 7)
                }
            }
        }
    }
}
