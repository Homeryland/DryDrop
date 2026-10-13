use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[cfg(feature = "server")]
pub struct PostgresConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub db: String,
}
