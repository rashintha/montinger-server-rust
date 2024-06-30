use rocket::{post, get, serde::json::Json};

use super::{
    auth_enum::MontingerError,
    auth_model::{LoginUser, RefreshRequest, TokenResponse},
    auth_service,
};

#[post("/auth/login", data = "<credentials>")]
pub async fn login(credentials: Json<LoginUser>) -> Result<Json<TokenResponse>, MontingerError> {
    auth_service::login(credentials.into_inner()).await
}

#[post("/auth/refresh", data = "<refresh_request>")]
pub async fn refresh(refresh_request: Json<RefreshRequest>) -> Result<Json<TokenResponse>, MontingerError> {
    auth_service::refresh(refresh_request.into_inner()).await
}

#[get("/error/unauthorized")]
pub fn unauthorized_error() -> MontingerError {
    MontingerError::Unauthorized
}

#[get("/auth/check")]
pub fn auth_check() -> Result<(), MontingerError> {
    Ok(())
}