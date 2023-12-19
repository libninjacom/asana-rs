use serde::{Serialize, Deserialize};
///The context from which this event originated.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuditLogEventContext {
    /**The authentication method used in the context of an API request.
Only present if the `context_type` is `api`. Can be one of `cookie`, `oauth`, `personal_access_token`, or `service_account`.*/
    pub api_authentication_method: String,
    ///The IP address of the client that initiated the event, if applicable.
    pub client_ip_address: String,
    /**The type of context.
Can be one of `web`, `desktop`, `mobile`, `asana_support`, `asana`, `email`, or `api`.*/
    pub context_type: String,
    /**The name of the OAuth App that initiated the event.
Only present if the `api_authentication_method` is `oauth`.*/
    pub oauth_app_name: String,
    ///The user agent of the client that initiated the event, if applicable.
    pub user_agent: String,
}
impl std::fmt::Display for AuditLogEventContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
