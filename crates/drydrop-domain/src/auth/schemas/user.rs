use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(toasty::Model))]
pub struct User {
    #[cfg_attr(feature = "server", key)]
    pub id: String,
    pub name: Option<String>,
    #[cfg_attr(feature = "server", unique)]
    pub email: Option<String>,
    pub email_verified: bool,
    pub image: Option<String>,
    pub created_at: jiff::Timestamp,
    pub updated_at: jiff::Timestamp,
    pub username: Option<String>,
    pub display_username: Option<String>,
    pub two_factor_enabled: bool,
    pub role: Option<String>,
    pub banned: bool,
    pub ban_reason: Option<String>,
    pub ban_expires: Option<jiff::Timestamp>,
    #[cfg_attr(feature = "server", column(type = jsonb))]
    pub metadata: Value,
}
