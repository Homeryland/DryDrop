use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[cfg(feature = "server")]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}
