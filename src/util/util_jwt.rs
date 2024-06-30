use jsonwebtoken::{encode, DecodingKey, EncodingKey, Header};

use crate::{
    api::auth::{auth_enum::MontingerError, auth_model::Claims},
    config,
};

pub fn encode_access_jwt(claims: &Claims) -> Result<String, MontingerError> {
    let jwt_secret = config::get_env_string("JWT_ACCESS_SECRET").expect("JWT_ACCESS_SECRET is missing.");
    encode_jwt(claims, jwt_secret.as_str())
}

pub fn encode_refresh_jwt(claims: &Claims) -> Result<String, MontingerError> {
    let jwt_secret = config::get_env_string("JWT_REFRESH_SECRET").expect("JWT_REFRESH_SECRET is missing.");
    encode_jwt(claims, jwt_secret.as_str())
}

fn encode_jwt(claims: &Claims, jwt_secret: &str) -> Result<String, MontingerError> {
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
    .map_err(|_| MontingerError::InternalServerError)
}

pub fn decode_access_jwt(token: &str) -> Result<Claims, MontingerError> {
    let jwt_secret = config::get_env_string("JWT_ACCESS_SECRET").expect("JWT_ACCESS_SECRET is missing.");
    decode_jwt(token, jwt_secret.as_str())
}

pub fn decode_refresh_jwt(token: &str) -> Result<Claims, MontingerError> {
    let jwt_secret = config::get_env_string("JWT_REFRESH_SECRET").expect("JWT_REFRESH_SECRET is missing.");
    decode_jwt(token, jwt_secret.as_str())
}

fn decode_jwt(token: &str, jwt_secret: &str) -> Result<Claims, MontingerError> {
    jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &jsonwebtoken::Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|_| MontingerError::Unauthorized)
}