
use serde::{Serialize, Deserialize};
use super::{AsanaResource, PortfolioCompact, UserCompact};
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize, Default, Dummy)]
pub struct PortfolioMembershipCompact {
    ///A generic Asana Resource, containing a globally unique identifier.
    #[serde(flatten)]
    pub asana_resource: AsanaResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portfolio: Option<PortfolioCompact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserCompact>,
}
impl std::fmt::Display for PortfolioMembershipCompact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for PortfolioMembershipCompact {
    type Target = AsanaResource;
    fn deref(&self) -> &Self::Target {
        &self.asana_resource
    }
}
impl std::ops::DerefMut for PortfolioMembershipCompact {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.asana_resource
    }
}