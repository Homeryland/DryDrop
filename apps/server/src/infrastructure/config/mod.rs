use std::path::Path;

use dotenvy::from_path;
use serde::Deserialize;
use snafu::{ResultExt, Whatever};
use tracing::info;

use crate::infrastructure::config::{
    postgres::PostgresConfig, server::ServerConfig, web::WebConfig,
};

pub mod postgres;
pub mod server;
pub mod web;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub mode: String,
    pub jwt_secret: String,
    pub owner_email: String,
    pub web: WebConfig,
    pub server: ServerConfig,
    pub postgres: PostgresConfig,
}

impl Config {
    pub fn load() -> Result<Self, Whatever> {
        let mode = std::env::var("RUST_ENV").unwrap_or_else(|_| "dev".into());
        from_path(Path::new(&format!("../../config/.env.{}", mode)))
            .with_whatever_context(|path| format!("Could not find {}", path))?;
        info!("Loaded {} environment variables successfully", mode);
        let config = config::Config::builder()
            .add_source(
                config::Environment::default()
                    .separator("__")
                    .try_parsing(true),
            )
            .build()
            .whatever_context("Could not build config")?;
        let config: Config = config
            .try_deserialize()
            .whatever_context("Could not deserialize config")?;
        info!("Owner email: {}", &config.owner_email);
        Ok(config)
    }
}
