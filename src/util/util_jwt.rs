use jsonwebtoken::{encode, DecodingKey, EncodingKey, Header};

use crate::{
    api::auth::{auth_enum::MontingerError, auth_model::Claims},
    config,
};

pub fn encode_jwt(claims: &Claims) -> Result<String, MontingerError> {
    let jwt_secret = config::get_env_string("JWT_SECRET").expect("JWT_SECRET is missing.");

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
    .map_err(|_| MontingerError::InternalServerError)
}


pub fn decode_jwt(token: &str) -> Result<Claims, MontingerError> {
    let jwt_secret = config::get_env_string("JWT_SECRET").expect("JWT_SECRET is missing.");

    jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &jsonwebtoken::Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|_| MontingerError::Unauthorized)
}