use log::info;
use mongodb::{options::ClientOptions, Client};
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};
use tokio::sync::OnceCell;

use crate::config;

static CLIENT: Lazy<OnceCell<Arc<Mutex<Client>>>> = Lazy::new(|| OnceCell::new());

pub async fn get_client() -> mongodb::error::Result<Arc<Mutex<Client>>> {
    CLIENT
        .get_or_try_init(|| async {
            info!("Initializing database connection...");

            let db_user = config::get_env_string("DB_USER").expect("DB_USER is missing.");
            let db_pass = config::get_env_string("DB_PASS").expect("DB_PASS is missing.");
            let db_host = config::get_env_string("DB_HOST").expect("DB_HOST is missing.");
            let db_port = config::get_env_string("DB_PORT").expect("DB_PORT is missing.");
            let database = config::get_env_string("DB").expect("DB is missing.");

            let connection_string = format!(
                "mongodb://{}:{}@{}:{}/?authSource={}",
                db_user, db_pass, db_host, db_port, database
            );

            let options = ClientOptions::parse(connection_string).await?;
            let client = Client::with_options(options)?;
            Ok::<_, mongodb::error::Error>(Arc::new(Mutex::new(client)))
        })
        .await
        .cloned() // Clone the Arc
}
