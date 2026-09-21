use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(toasty::Model))]
pub struct Verification {
    #[cfg_attr(feature = "server", key)]
    pub id: String,
    pub identifier: String,
    pub value: String,
    pub expires_at: jiff::Timestamp,
    pub created_at: jiff::Timestamp,
    pub updated_at: jiff::Timestamp,
}
