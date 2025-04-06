use app::{
    config::Config, db::init_db, handlers::Handlers, repositories::Repositories, services::Services,
};
use clickhouse::Client;
use dotenvy::dotenv;
use feature::post::handler::MyIngesterHandlers;
use server::grpc::servergrpc::init_grpc_server;
use std::{error::Error, net::SocketAddr, sync::Arc};
mod app;
mod feature;
mod server;
#[cfg(not(target_os = "windows"))]
use jemallocator::Jemalloc as GlobalAlloc;

#[cfg(target_os = "windows")]
use mimalloc::MiMalloc as GlobalAlloc;

#[global_allocator]
static GLOBAL: GlobalAlloc = GlobalAlloc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok().expect("Could not load .env file");
    let config = Config::new().await;
    //todo:delete to prod
    println!("Config: {:?}", config);

    let client = init_db(&config).await?;

    run_migrations(&client).await?;

    let repo = Arc::new(Repositories::new_repositories(client));
    let in_memory_services = Arc::new(Services::new_services(repo));
    let handlers = Arc::new(Handlers::new_handlers(in_memory_services));

    let _ = init_grpc_server(config, handlers.post_handler.clone()).await?;
    Ok(())
}

async fn run_migrations(client: &Client) -> Result<(), Box<dyn Error>> {
    println!("Attempting to connect and run migrations...");

    let create_table_sql = r#"
        CREATE TABLE IF NOT EXISTS posts (
            chat_id Int64,
            message_id UInt32,
            timestamp DateTime64(3, 'UTC'),
            post_timestamp DateTime64(3, 'UTC'),
            text String
        ) ENGINE = MergeTree()
        ORDER BY (chat_id, message_id, timestamp);
    "#;

    match client.query(create_table_sql).execute().await {
        Ok(_) => {
            println!("Migrations executed successfully.");
            Ok(())
        }
        Err(e) => {
            eprintln!("Error executing migration query: {}", e);
            Err(Box::new(e))
        }
    }
}
