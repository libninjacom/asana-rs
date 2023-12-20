use serde::{Serialize, Deserialize};
///Event specific details. The schema will vary depending on the `event_type`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuditLogEventDetails {}
impl std::fmt::Display for AuditLogEventDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}