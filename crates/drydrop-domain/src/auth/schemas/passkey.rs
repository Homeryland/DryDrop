use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(toasty::Model))]
pub struct Passkey {
    #[cfg_attr(feature = "server", key)]
    pub id: String,
    pub name: String,
    pub public_key: String,
    pub user_id: String,
    pub credential_id: String,
    pub counter: u64,
    pub device_type: String,
    pub backed_up: bool,
    pub transports: Option<String>,
    pub created_at: jiff::Timestamp,
}
