use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(toasty::Model))]
pub struct Account {
    #[cfg_attr(feature = "server", key, auto(uuid(v7)))]
    pub id: Uuid,
    pub account_id: String,
    pub provider_id: String,
    pub user_id: Uuid,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub id_token: Option<String>,
    pub access_token_expires_at: Option<jiff::Timestamp>,
    pub refresh_token_expires_at: Option<jiff::Timestamp>,
    pub scope: Option<String>,
    pub password: Option<String>,
    pub created_at: jiff::Timestamp,
    pub updated_at: jiff::Timestamp,
}
