use tokio::join;

use dotenv::dotenv;
use log::info;
use montinger_server::{api, db, grpc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    info!("Initializing...");

    let _ = db::get_client().await;
    let _ = join!(grpc::start_server(), api::initialize());

    Ok(())
}
