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

        let claims = Claims {
            sub: user.email.clone(),
            exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp(),
        };

        let db_user = config::get_env_string("JWT_SECRET").expect("DB_USER is missing.");

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(db_user.as_ref()),
        )
        .map_err(|_| MontingerError::InternalServerError)?;

        return Ok(Json(TokenResponse {
            access_token: token,
        }));
    }
    Err(MontingerError::Unauthorized)
}
