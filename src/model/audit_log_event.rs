
use serde::{Serialize, Deserialize};
use super::{
    AuditLogEventActor, AuditLogEventContext, AuditLogEventDetails, AuditLogEventResource,
};
/**An object representing a single event within an Asana domain.

Every audit log event is comprised of an `event_type`, `actor`, `resource`, and `context`. Some events will include additional metadata about the event under `details`. See our [currently supported list of events](/docs/audit-log-events#supported-audit-log-events) for more details.*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuditLogEvent {
    ///The entity that triggered the event. Will typically be a user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor: Option<AuditLogEventActor>,
    ///The context from which this event originated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<AuditLogEventContext>,
    ///The time the event was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    ///Event specific details. The schema will vary depending on the `event_type`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<AuditLogEventDetails>,
    ///The category that this `event_type` belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_category: Option<String>,
    ///The type of the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    ///Globally unique identifier of the `AuditLogEvent`, as a string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gid: Option<String>,
    ///The primary object that was affected by this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<AuditLogEventResource>,
}
impl std::fmt::Display for AuditLogEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}