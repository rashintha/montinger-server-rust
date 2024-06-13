use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}