use rocket::serde::json::Json;

use crate::api::auth::auth_enum::MontingerError;

use super::{monitors_model::MonitorResponse, monitors_repository};

pub async fn get_all() -> Result<Json<Vec<MonitorResponse>>, MontingerError> {
  monitors_repository::get_all()
    .await
    .map(|monitors| Json(monitors.into_iter().map(MonitorResponse::from).collect()))
}