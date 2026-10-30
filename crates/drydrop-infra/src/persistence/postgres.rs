use drydrop_config::postgres::PostgresConfig;
use toasty::Db;
use tracing::warn;

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
        if let Err(e) = db.push_schema().await {
            // `push_schema` re-creates every table/type from the models, so it fails
            // with `already exists` whenever the database already has the schema
            // (e.g. watchexec restarts between `start` wipes). The database is either
            // fresh (push succeeds) or fully populated (data dirs are wiped on start),
            // so treat `already exists` as up-to-date instead of crashing.
            if e.to_string().contains("already exists") {
                warn!("database schema already exists, skipping push_schema");
            } else {
                return Err(e);
            }
        }
        Ok(PgClient(db))
    }
    pub fn inner(&self) -> &Db {
        &self.0
    }
}
