pub mod auth_controller;
pub mod auth_model;
pub mod auth_service;

use axum::{routing::get, Router};

pub fn router() -> Router {
    Router::new().route("/login", get(auth_controller::login))
}
