pub mod auth;

use crate::{config, db, middleware::LoggingMiddleware};
use auth::auth_controller;
use log::info;
use rocket::routes;
use rocket_cors::CorsOptions;

pub async fn initialize() -> Result<(), Box<dyn std::error::Error>> {
    let rest_port = config::get_env_string("REST_PORT").expect("REST_PORT is missing.");

    info!("Initializing REST API on [::1]:{}...", rest_port);

    let cors = CorsOptions::default().to_cors().unwrap();
    let config = rocket::Config {
        port: rest_port.parse().unwrap(),
        address: "::1".parse().unwrap(),
        log_level: rocket::config::LogLevel::Normal,
        ..Default::default()
    };

    rocket::custom(config)
        .mount("/", routes![auth_controller::login])
        .manage(db::get_client().await)
        .attach(cors)
        .attach(LoggingMiddleware)
        .launch()
        .await?;

    Ok(())
}
