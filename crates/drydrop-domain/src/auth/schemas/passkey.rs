use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(toasty::Model))]
pub struct Passkey {
    #[cfg_attr(feature = "server", key, auto(uuid(v7)))]
    pub id: Uuid,
    pub name: String,
    pub public_key: String,
    pub user_id: Uuid,
    pub credential_id: String,
    pub counter: u64,
    pub device_type: String,
    pub backed_up: bool,
    pub transports: Option<String>,
    pub created_at: jiff::Timestamp,
}
