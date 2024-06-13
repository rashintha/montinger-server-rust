use axum::Json;

use super::auth_model::LoginUser;

pub async fn login(Json(login): Json<LoginUser>) {
    // Handle login

    println!("Login: {:?} {:?}", login.email, login.password);
}
