use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(toasty::Model))]
pub struct TwoFactor {
    #[cfg_attr(feature = "server", key)]
    pub id: String,
    pub secret: String,
    pub backup_codes: Option<String>,
    pub user_id: String,
    pub created_at: jiff::Timestamp,
    pub updated_at: jiff::Timestamp,
}
