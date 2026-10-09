use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(toasty::Model))]
pub struct TwoFactor {
    #[cfg_attr(feature = "server", key, auto(uuid(v7)))]
    pub id: Uuid,
    pub secret: String,
    pub backup_codes: Option<String>,
    pub user_id: Uuid,
    pub created_at: jiff::Timestamp,
    pub updated_at: jiff::Timestamp,
}
