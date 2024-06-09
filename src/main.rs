use dotenv::dotenv;
use log::{info, LevelFilter};
use montinger_server::{db, grpc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>  {
    dotenv().ok();

    env_logger::Builder::new()
        .filter_level(LevelFilter::Info) // Set the minimum log level to display
        .init();

    info!("Initializing...");

    let _ = db::get_client().await;
    grpc::start_server().await?;

    Ok(())
}
