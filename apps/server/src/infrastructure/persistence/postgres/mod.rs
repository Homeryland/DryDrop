#[derive(Debug, Clone)]
pub struct PGClient {
    pub db: toasty::Db,
}

impl PGClient {
    pub async fn new(pg_url: &str) -> toasty::Result<Self> {
        let db = toasty::Db::builder()
            .models(toasty::models!(
                drydrop::auth::models::Account,
                drydrop::auth::models::Organization,
                drydrop::auth::models::Invitation,
                drydrop::auth::models::Member,
                drydrop::auth::models::Passkey,
                drydrop::auth::models::Session,
                drydrop::auth::models::TwoFactor,
                drydrop::auth::models::User,
                drydrop::auth::models::Verification,
            ))
            .connect(pg_url)
            .await?;
        db.push_schema().await?;
        Ok(PGClient { db })
    }
}
