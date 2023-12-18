
use serde::{Serialize, Deserialize};
use super::AsanaResource;
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize, Default, Dummy)]
pub struct EnumOption {
    ///A generic Asana Resource, containing a globally unique identifier.
    #[serde(flatten)]
    pub asana_resource: AsanaResource,
    ///The color of the enum option. Defaults to ‘none’.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    ///Whether or not the enum option is a selectable value for the custom field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    ///The name of the enum option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl std::fmt::Display for EnumOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for EnumOption {
    type Target = AsanaResource;
    fn deref(&self) -> &Self::Target {
        &self.asana_resource
    }
}
impl std::ops::DerefMut for EnumOption {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.asana_resource
    }
}