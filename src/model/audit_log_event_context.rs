
use serde::{Serialize, Deserialize};
///The context from which this event originated.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuditLogEventContext {
    /**The authentication method used in the context of an API request.
Only present if the `context_type` is `api`. Can be one of `cookie`, `oauth`, `personal_access_token`, or `service_account`.*/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_authentication_method: Option<String>,
    ///The IP address of the client that initiated the event, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_ip_address: Option<String>,
    /**The type of context.
Can be one of `web`, `desktop`, `mobile`, `asana_support`, `asana`, `email`, or `api`.*/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_type: Option<String>,
    /**The name of the OAuth App that initiated the event.
Only present if the `api_authentication_method` is `oauth`.*/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_app_name: Option<String>,
    ///The user agent of the client that initiated the event, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}
impl std::fmt::Display for AuditLogEventContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}