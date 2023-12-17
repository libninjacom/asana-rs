
use serde::{Serialize, Deserialize};
use super::{
    AuditLogEventActor, AuditLogEventContext, AuditLogEventDetails, AuditLogEventResource,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuditLogEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor: Option<AuditLogEventActor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<AuditLogEventContext>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<AuditLogEventDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<AuditLogEventResource>,
}
impl std::fmt::Display for AuditLogEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}