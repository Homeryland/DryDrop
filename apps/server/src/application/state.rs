use snafu::ResultExt;

use crate::{
    application::error::{AppResult, DBSnafu},
    infrastructure::config::Config,
};

#[derive(Clone)]
pub struct AppState {
    pub mode: String,
    pub web_base_url: String,
    pub server_base_url: String,
    pub jwt_secret: String,
    pub db: toasty::Db,
}

impl AppState {
    pub async fn new(config: Config) -> AppResult<Self> {
        let mode = config.mode;
        let web_base_url = format!("http://{}:{}", config.web.host, config.web.port);
        let server_base_url = format!("http://{}:{}", config.server.host, config.server.port);
        let db_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            config.postgres.user,
            config.postgres.password,
            config.postgres.host,
            config.postgres.port,
            config.postgres.db
        );
        let db = toasty::Db::builder()
            .connect(&db_url)
            .await
            .context(DBSnafu)?;
        db.push_schema().await.context(DBSnafu)?;
        let jwt_secret = config.jwt_secret;
        Ok(Self {
            mode,
            web_base_url,
            server_base_url,
            jwt_secret,
            db,
        })
    }
}
