use std::path::Path;

use dotenvy::from_path;
use serde::Deserialize;
use snafu::{ResultExt, Whatever};

use crate::{postgres::PostgresConfig, server::ServerConfig, web::WebConfig};

#[derive(Debug, Deserialize, Clone)]
#[cfg(feature = "server")]
pub struct Config {
    pub jwt_secret: String,
    pub owner_email: String,
    pub web: WebConfig,
    pub server: ServerConfig,
    pub postgres: PostgresConfig,
}

impl Config {
    pub fn load() -> Result<Self, Whatever> {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../config/.env");
        from_path(&path).with_whatever_context(|e| {
            format!("Could not find env file {}: {e}", path.display())
        })?;
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
        Ok(config)
    }
}
