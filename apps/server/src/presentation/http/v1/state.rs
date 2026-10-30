use drydrop::{config::Config, infra::PgClient};
use snafu::ResultExt;

use crate::application::error::{AppResult, DBSnafu};

#[derive(Clone)]
pub struct AppState {
    pub web_base_url: String,
    pub server_base_url: String,
    pub jwt_secret: String,
    pub db: PgClient,
}

impl AppState {
    pub async fn new(config: Config) -> AppResult<Self> {
        let web_base_url = format!("http://{}:{}", config.web.host, config.web.port);
        let server_base_url = format!("http://{}:{}", config.server.host, config.server.port);
        let jwt_secret = config.jwt_secret;
        let db = PgClient::new(&config.postgres).await.context(DBSnafu)?;
        Ok(Self {
            web_base_url,
            server_base_url,
            jwt_secret,
            db,
        })
    }
}
