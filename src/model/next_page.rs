
use serde::{Serialize, Deserialize};
use fake::Dummy;
///*Conditional*. This property is only present when a limit query parameter is provided in the request. When making a paginated request, the API will return a number of results as specified by the limit parameter. If more results exist, then the response will contain a next_page attribute, which will include an offset, a relative path attribute, and a full uri attribute. If there are no more pages available, next_page will be null and no offset will be provided. Note that an offset token will expire after some time, as data may have changed.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Dummy)]
pub struct NextPage {
    ///Pagination offset for the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<String>,
    ///A relative path containing the query parameters to fetch for next_page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    ///A full uri containing the query parameters to fetch for next_page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
impl std::fmt::Display for NextPage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}