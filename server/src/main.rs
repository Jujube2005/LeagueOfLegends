use std::sync::Arc;

use server::{
    config::config_loader,
    infrastructure::{database::postgresql_connection, http::http_serv::start},
};
use tracing::{error, info};

use std::io::{self, Write};

#[tokio::main]
async fn main() {
    println!(">>> SERVER STARTING UP...");
    io::stdout().flush().unwrap();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    println!(">>> LOADING CONFIG...");
    io::stdout().flush().unwrap();
    
    let dotenvy_env = match config_loader::load() {
        Ok(env) => {
            println!(">>> CONFIG LOADED");
            io::stdout().flush().unwrap();
            env
        },
        Err(e) => {
            eprintln!(">>> FATAL: FAILED TO LOAD CONFIG: {e:?}");
            error!("Failed to load ENV: {}", e);
            std::process::exit(1);
        }
    };

    println!(">>> CONNECTING TO DATABASE...");
    io::stdout().flush().unwrap();
    
    let postgres_pool = match postgresql_connection::establish_connection(&dotenvy_env.database.url)
    {
        Ok(pool) => {
            println!(">>> DATABASE CONNECTED");
            io::stdout().flush().unwrap();
            pool
        },
        Err(err) => {
            eprintln!(">>> FATAL: DATABASE CONNECTION FAILED: {err:?}");
            error!("Fail to connect: {}", err);
            std::process::exit(1)
        }
    };

    println!(">>> STARTING HTTP SERVER ON PORT {}...", dotenvy_env.server.port);
    io::stdout().flush().unwrap();
    
    if let Err(e) = start(Arc::new(dotenvy_env), Arc::new(postgres_pool)).await {
        eprintln!(">>> FATAL: SERVER FAILED TO START: {e:?}");
        panic!("Failed to start server: {:?}", e);
    }
}
