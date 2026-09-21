use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(toasty::Model))]
pub struct Organization {
    #[cfg_attr(feature = "server", key)]
    pub id: Option<String>,
    pub name: String,
    pub slug: String,
    pub logo: Option<String>,
    #[cfg_attr(feature = "server", column(type = jsonb))]
    pub metadata: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(toasty::Model))]
pub struct Member {
    #[cfg_attr(feature = "server", key)]
    pub organization_id: String,
    pub user_id: String,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(toasty::Embed))]
pub enum InvitationStatus {
    Pending,
    Accepted,
    Rejected,
    Canceled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(toasty::Model))]
pub struct Invitation {
    #[cfg_attr(feature = "server", key)]
    pub id: String,
    pub organization_id: String,
    pub email: String,
    pub role: String,
    pub status: InvitationStatus,
    pub inviter_id: String,
    pub expires_at: jiff::Timestamp,
    pub created_at: jiff::Timestamp,
}
