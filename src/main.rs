use dotenv::dotenv;
use log::{info, LevelFilter};
use montinger_server::db;

#[tokio::main]
async fn main() {
    dotenv().ok();

    env_logger::Builder::new()
        .filter_level(LevelFilter::Info) // Set the minimum log level to display
        .init();

    info!("Starting server...");

    let _ = db::get_client().await;

    Ok::<(), Box<dyn std::error::Error>>(()).expect("REASON")
}
