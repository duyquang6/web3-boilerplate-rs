use std::sync::LazyLock;

use config::{Environment, File};
use serde::Deserialize;

/// AppConfig define config
#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub database: Database,
    pub eth_rpc_url: String,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub max_connections: u32,
    pub url: String,
}

pub static CONFIG: LazyLock<AppConfig> = LazyLock::new(|| {
    let cfg = config::Config::builder()
        .add_source(File::with_name("config/local")) // `config/local.toml`
        .set_override(
            "database.url",
            std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://postgres:123abc@localhost".to_string()),
        )
        .unwrap()
        .add_source(Environment::with_prefix("APP").separator("__")) // APP__DATABASE__URL
        .build()
        .expect("parse config error");

    cfg.try_deserialize().expect("deserialize failed")
});
