use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(toasty::Model))]
pub struct Session {
    #[cfg_attr(feature = "server", key)]
    pub id: String,
    pub expires_at: jiff::Timestamp,
    pub token: String,
    pub created_at: jiff::Timestamp,
    pub updated_at: jiff::Timestamp,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub user_id: String,
    pub impersonated_by: Option<String>,
    pub active_organization_id: Option<String>,
    pub active: bool,
}
