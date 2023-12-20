use serde::{Serialize, Deserialize};
/**A collection of rich text that will be displayed as a preview to another app.

This is read-only except for a small group of whitelisted apps.*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Preview {
    ///Some fallback text to display if unable to display the full preview.
    pub fallback: String,
    ///Text to display in the footer.
    pub footer: String,
    ///Text to display in the header.
    pub header: String,
    ///Where the header will link to.
    pub header_link: String,
    ///HTML formatted text for the body of the preview.
    pub html_text: String,
    ///Text for the body of the preview.
    pub text: String,
    ///Text to display as the title.
    pub title: String,
    ///Where to title will link to.
    pub title_link: String,
}
impl std::fmt::Display for Preview {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}