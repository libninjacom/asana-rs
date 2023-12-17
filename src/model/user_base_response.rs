
use serde::{Serialize, Deserialize};
use super::UserBase;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserBaseResponse {
    #[serde(flatten)]
    pub user_base: UserBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<serde_json::Value>,
}
impl std::fmt::Display for UserBaseResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for UserBaseResponse {
    type Target = UserBase;
    fn deref(&self) -> &Self::Target {
        &self.user_base
    }
}
impl std::ops::DerefMut for UserBaseResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.user_base
    }
}