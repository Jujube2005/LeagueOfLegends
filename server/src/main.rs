use std::sync::Arc;

use server::{config::config_loader::load, infrastructure::{database::postgresql_connection, http::http_serv::start}};
use tracing::{Level, info, error};


#[tokio::main]
async fn main() {
   
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let dotenvy_env = match load() {
        Ok(env) => env,
        Err(e) => {
            tracing::error!("Failed to load .env file: {}", e);
            std::process::exit(1);
        }
    };

    info!("YESSSSS");         

    let postgres_pool = match postgresql_connection::establish_connection(&dotenvy_env.database.url)
    {
        Ok(pool) => pool,
        Err(e) => {
            error!("Failed to establish connection to Postgres: {}", e);
            std::process::exit(1);
        }
    };

    info!("Connect to the PostgreSQL database successfully.");

// Use MissionEntity from server::domain::entities::missions; no local definition needed here.



    start(Arc::new(dotenvy_env), Arc::new(postgres_pool)).await.expect("Failed to start the server");


}