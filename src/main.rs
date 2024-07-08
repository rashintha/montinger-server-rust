use tokio::{join, spawn};

use dotenv::dotenv;
use log::info;
use montinger_server::{api, cron, db, grpc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    info!("Initializing...");

    let _ = db::get_client().await;

    // let (_, _) = join!(spawn(cron::initialize()), spawn(cron::run_cron_jobs()));
    let (_, _, _, rocket_result) = join!(
        grpc::start_server(),
        spawn(cron::initialize()),
        spawn(cron::run_cron_jobs()),
        api::initialize()
    );

    rocket_result?;

    Ok(())
}
