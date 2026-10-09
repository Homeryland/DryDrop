use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(toasty::Model))]
pub struct Session {
    #[cfg_attr(feature = "server", key, auto(uuid(v7)))]
    pub id: Uuid,
    pub expires_at: jiff::Timestamp,
    pub token: String,
    pub created_at: jiff::Timestamp,
    pub updated_at: jiff::Timestamp,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub user_id: Uuid,
    pub impersonated_by: Option<String>,
    pub active_organization_id: Option<String>,
    pub active: bool,
}
