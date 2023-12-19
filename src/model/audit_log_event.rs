use serde::{Serialize, Deserialize};
use super::{
    AuditLogEventActor, AuditLogEventContext, AuditLogEventDetails, AuditLogEventResource,
};
/**An object representing a single event within an Asana domain.

Every audit log event is comprised of an `event_type`, `actor`, `resource`, and `context`. Some events will include additional metadata about the event under `details`. See our [currently supported list of events](/docs/audit-log-events#supported-audit-log-events) for more details.*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuditLogEvent {
    ///The entity that triggered the event. Will typically be a user.
    pub actor: AuditLogEventActor,
    ///The context from which this event originated.
    pub context: AuditLogEventContext,
    ///The time the event was created.
    pub created_at: chrono::DateTime<chrono::Utc>,
    ///Event specific details. The schema will vary depending on the `event_type`.
    pub details: AuditLogEventDetails,
    ///The category that this `event_type` belongs to.
    pub event_category: String,
    ///The type of the event.
    pub event_type: String,
    ///Globally unique identifier of the `AuditLogEvent`, as a string.
    pub gid: String,
    ///The primary object that was affected by this event.
    pub resource: AuditLogEventResource,
}
impl std::fmt::Display for AuditLogEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
