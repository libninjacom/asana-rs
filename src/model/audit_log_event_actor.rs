
use serde::{Serialize, Deserialize};
use fake::Dummy;
///The entity that triggered the event. Will typically be a user.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Dummy)]
pub struct AuditLogEventActor {
    /**The type of actor.
Can be one of `user`, `asana`, `asana_support`, `anonymous`, or `external_administrator`.*/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_type: Option<String>,
    ///The email of the actor, if it is a user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    ///Globally unique identifier of the actor, if it is a user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gid: Option<String>,
    ///The name of the actor, if it is a user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl std::fmt::Display for AuditLogEventActor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}