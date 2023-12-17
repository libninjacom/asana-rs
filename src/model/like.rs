
use serde::{Serialize, Deserialize};
use super::UserCompact;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Like {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserCompact>,
}
impl std::fmt::Display for Like {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}