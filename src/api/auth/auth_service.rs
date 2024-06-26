use argon2::{Argon2, PasswordHash, PasswordVerifier};
use jsonwebtoken::{encode, EncodingKey, Header};
use rocket::serde::json::Json;

use crate::config;

use super::{
    auth_enum::MontingerError,
    auth_model::{Claims, LoginUser, TokenResponse},
    auth_repository,
};

pub async fn login(credentials: LoginUser) -> Result<Json<TokenResponse>, MontingerError> {
    let db_user = auth_repository::get_user_by_email(&credentials.email).await;

    let user = match db_user {
        Ok(user) => user,
        Err(_) => return Err(MontingerError::Unauthorized),
    };

    if let Some(user) = user {
        let argon2 = Argon2::default();
        let is_valid = PasswordHash::new(&user.password)
            .and_then(|hash| argon2.verify_password(credentials.password.as_bytes(), &hash))
            .is_ok();

        if !is_valid {
            return Err(MontingerError::Unauthorized);
        }

        let access_exp_time = config::get_env_i64("ACCESS_EXPIRES_IN").expect("ACCESS_EXPIRES_IN is invalid.");
        let refresh_exp_time = config::get_env_i64("REFRESH_EXPIRES_IN").expect("REFRESH_EXPIRES_IN is invalid.");

        let claim_access = Claims {
            sub: user.email.clone(),
            exp: (chrono::Utc::now() + chrono::Duration::minutes(access_exp_time)).timestamp(),
        };

        let claim_refresh = Claims {
            sub: user.email.clone(),
            exp: (chrono::Utc::now() + chrono::Duration::minutes(refresh_exp_time)).timestamp(),
        };

        let jwt_secret = config::get_env_string("JWT_SECRET").expect("DB_USER is missing.");

        let token_access = encode(
            &Header::default(),
            &claim_access,
            &EncodingKey::from_secret(jwt_secret.as_ref()),
        )
        .map_err(|_| MontingerError::InternalServerError)?;

        let token_refresh = encode(
            &Header::default(),
            &claim_refresh,
            &EncodingKey::from_secret(jwt_secret.as_ref()),
        )
        .map_err(|_| MontingerError::InternalServerError)?;

        return Ok(Json(TokenResponse {
            access_token: token_access,
            refresh_token: token_refresh,
        }));
    }
    Err(MontingerError::Unauthorized)
}
