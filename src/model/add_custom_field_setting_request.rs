
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AddCustomFieldSettingRequest {
    ///The custom field to associate with this container.
    pub custom_field: String,
    ///A gid of a Custom Field Setting on this container, after which the new Custom Field Setting will be added.  `insert_before` and `insert_after` parameters cannot both be specified.
    pub insert_after: String,
    ///A gid of a Custom Field Setting on this container, before which the new Custom Field Setting will be added.  `insert_before` and `insert_after` parameters cannot both be specified.
    pub insert_before: String,
    ///Whether this field should be considered important to this container (for instance, to display in the list view of items in the container).
    pub is_important: bool,
}
impl std::fmt::Display for AddCustomFieldSettingRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}