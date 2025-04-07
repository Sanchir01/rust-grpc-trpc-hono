use std::env;

use clickhouse::Client;

use super::config::Config;

pub async fn init_db(config: &Config) -> Result<Client, Box<dyn std::error::Error>> {
    let clickhouse_config = config
        .get_database_config()
        .ok_or_else(|| "ClickHouse configuration not found")?;
    let password = env::var("CLICKHOUSE_PASSWORD")
        .map_err(|e| format!("Failed to get CLICKHOUSE_PASSWORD from env: {}", e))?;

    let addr = format!(
        "http://{}:{}",
        clickhouse_config.host, clickhouse_config.port
    );

    println!("Connecting to ClickHouse at: {}", addr);

    
    let client = Client::default()
        .with_url(&addr)
        .with_user(&clickhouse_config.username)
        .with_password(password)
        .with_database(&clickhouse_config.database);

   
    match client.query("SELECT 1").execute().await {
        Ok(_) => println!("Successfully connected to ClickHouse!"),
        Err(e) => println!(
            "Warning: Failed to connect to ClickHouse: {}. Will try to continue anyway.",
            e
        ),
    }

    Ok(client)
}
