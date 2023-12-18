
use serde::{Serialize, Deserialize};
use fake::Dummy;
/**A collection of rich text that will be displayed as a preview to another app.

This is read-only except for a small group of whitelisted apps.*/
#[derive(Debug, Clone, Serialize, Deserialize, Default, Dummy)]
pub struct Preview {
    ///Some fallback text to display if unable to display the full preview.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback: Option<String>,
    ///Text to display in the footer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,
    ///Text to display in the header.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,
    ///Where the header will link to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_link: Option<String>,
    ///HTML formatted text for the body of the preview.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_text: Option<String>,
    ///Text for the body of the preview.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    ///Text to display as the title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    ///Where to title will link to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_link: Option<String>,
}
impl std::fmt::Display for Preview {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}