use drydrop_config::postgres::PostgresConfig;
use toasty::Db;

#[derive(Clone)]
pub struct PgClient(Db);

impl PgClient {
    pub async fn new(config: &PostgresConfig) -> toasty::Result<Self> {
        let db_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            config.user, config.password, config.host, config.port, config.db
        );
        let db = toasty::Db::builder()
            .models(toasty::models!(
                drydrop_domain::auth::schemas::account::Account,
                drydrop_domain::auth::schemas::organization::Organization,
                drydrop_domain::auth::schemas::organization::Invitation,
                drydrop_domain::auth::schemas::organization::Member,
                drydrop_domain::auth::schemas::passkey::Passkey,
                drydrop_domain::auth::schemas::session::Session,
                drydrop_domain::auth::schemas::two_factor::TwoFactor,
                drydrop_domain::auth::schemas::user::User,
                drydrop_domain::auth::schemas::verification::Verification,
            ))
            .connect(&db_url)
            .await?;
        db.push_schema().await?;
        Ok(PgClient(db))
    }
    pub fn inner(&self) -> &Db {
        &self.0
    }
}
