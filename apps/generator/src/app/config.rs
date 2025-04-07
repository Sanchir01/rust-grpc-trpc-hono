use serde::Deserialize;
use tokio::fs;
use toml;

#[derive(Debug, Deserialize)]
pub struct GRPClientConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub grpc_server: Option<GRPCServerConfig>,
}

impl Config {
    pub async fn new() -> Self {
        let config_path = if cfg!(debug_assertions) {
            println!("Dev mode");
            "apps/generator/config/dev.toml"
        } else {
            println!("Prod mode");
            "apps/generator/config/prod.toml"
        };

        let content = fs::read_to_string(config_path).await.unwrap_or_else(|err| {
            println!("Error reading config file: {:?}", err);
            String::new()
        });

        toml::from_str(&content).unwrap_or_else(|err| {
            println!("Error parsing config file: {:?}", err);
            Config { grpc_server: None }
        })
    }

    pub fn get_grpc_config(&self) -> Option<&GRPCServerConfig> {
        self.grpc_server.as_ref()
    }
}
