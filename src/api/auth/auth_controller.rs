use rocket::{post, serde::json::Json};

use super::{
    auth_enum::MontingerError,
    auth_model::{LoginUser, TokenResponse},
    auth_service,
};

#[post("/auth/login", data = "<credentials>")]
pub async fn login(credentials: Json<LoginUser>) -> Result<Json<TokenResponse>, MontingerError> {
    auth_service::login(credentials.into_inner()).await
}
