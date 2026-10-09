use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(toasty::Model))]
pub struct Organization {
    #[cfg_attr(feature = "server", key, auto(uuid(v7)))]
    pub id: Option<Uuid>,
    pub name: String,
    pub slug: String,
    pub logo: Option<String>,
    #[cfg_attr(feature = "server", column(type = jsonb))]
    pub metadata: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(toasty::Model))]
pub struct Member {
    #[cfg_attr(feature = "server", key, auto(uuid(v7)))]
    pub organization_id: Uuid,
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
