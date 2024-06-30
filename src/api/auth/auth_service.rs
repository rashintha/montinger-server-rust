use argon2::{Argon2, PasswordHash, PasswordVerifier};
use rocket::serde::json::Json;

use crate::{
    config,
    util::util_jwt::{decode_refresh_jwt, encode_access_jwt, encode_refresh_jwt},
};

use super::{
    auth_enum::MontingerError,
    auth_model::{Claims, LoginUser, RefreshRequest, TokenResponse},
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

        let access_exp_time =
            config::get_env_i64("ACCESS_EXPIRES_IN").expect("ACCESS_EXPIRES_IN is invalid.");
        let refresh_exp_time =
            config::get_env_i64("REFRESH_EXPIRES_IN").expect("REFRESH_EXPIRES_IN is invalid.");

        let claim_access = Claims {
            sub: user.email.clone(),
            exp: (chrono::Utc::now() + chrono::Duration::minutes(access_exp_time)).timestamp(),
        };

        let claim_refresh = Claims {
            sub: user.email.clone(),
            exp: (chrono::Utc::now() + chrono::Duration::minutes(refresh_exp_time)).timestamp(),
        };

        let token_access = encode_access_jwt(&claim_access)?;
        let token_refresh = encode_refresh_jwt(&claim_refresh)?;

        return Ok(Json(TokenResponse {
            access_token: token_access,
            refresh_token: token_refresh,
        }));
    }
    Err(MontingerError::Unauthorized)
}

pub async fn refresh(
    refresh_request: RefreshRequest,
) -> Result<Json<TokenResponse>, MontingerError> {
    let claims = decode_refresh_jwt(refresh_request.refresh_token.as_str())?;

    let access_exp_time =
        config::get_env_i64("ACCESS_EXPIRES_IN").expect("ACCESS_EXPIRES_IN is invalid.");
    let refresh_exp_time =
        config::get_env_i64("REFRESH_EXPIRES_IN").expect("REFRESH_EXPIRES_IN is invalid.");

    if claims.exp < chrono::Utc::now().timestamp() {
        return Err(MontingerError::Unauthorized);
    }

    let claim_access = Claims {
        sub: claims.sub.clone(),
        exp: (chrono::Utc::now() + chrono::Duration::minutes(access_exp_time)).timestamp(),
    };

    let claim_refresh = Claims {
        sub: claims.sub.clone(),
        exp: (chrono::Utc::now() + chrono::Duration::minutes(refresh_exp_time)).timestamp(),
    };

    let token_access = encode_access_jwt(&claim_access)?;
    let token_refresh = encode_refresh_jwt(&claim_refresh)?;

    Ok(Json(TokenResponse {
        access_token: token_access,
        refresh_token: token_refresh,
    }))
}
