use rocket::{get, serde::json::Json};

use crate::api::auth::auth_enum::MontingerError;

use super::{monitors_model::MonitorResponse, monitors_service};

#[get("/monitors")]
pub async fn get_all() -> Result<Json<Vec<MonitorResponse>>, MontingerError> {
    monitors_service::get_all().await
}
