use serde::{Serialize, Deserialize};
///The entity that triggered the event. Will typically be a user.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuditLogEventActor {
    /**The type of actor.
Can be one of `user`, `asana`, `asana_support`, `anonymous`, or `external_administrator`.*/
    pub actor_type: String,
    ///The email of the actor, if it is a user.
    pub email: String,
    ///Globally unique identifier of the actor, if it is a user.
    pub gid: String,
    ///The name of the actor, if it is a user.
    pub name: String,
}
impl std::fmt::Display for AuditLogEventActor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}