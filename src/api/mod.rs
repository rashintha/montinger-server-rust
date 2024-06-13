pub mod auth;

use crate::config;
use log::info;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

pub async fn initialize() -> Result<(), Box<dyn std::error::Error>> {
    let rest_port = config::get_env_string("REST_PORT").expect("REST_PORT is missing.");

    info!("Initializing REST API on [::1]:{}...", rest_port);

    let app = axum::Router::new().nest("/auth", auth::router()).layer(
        TraceLayer::new_for_http()
            .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
            .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
    );

    let listener = tokio::net::TcpListener::bind(format!("[::1]:{}", rest_port)).await?;
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
