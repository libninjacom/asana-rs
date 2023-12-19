use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AsanaClient;
/**You should use this struct via [`AsanaClient::add_custom_field_setting_for_portfolio`].

On request success, this will return a [`AddCustomFieldSettingForPortfolioResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddCustomFieldSettingForPortfolioRequest {
    pub data: AddCustomFieldSettingRequest,
    pub opt_pretty: Option<bool>,
    pub portfolio_gid: String,
}
impl AddCustomFieldSettingForPortfolioRequest {}
impl FluentRequest<'_, AddCustomFieldSettingForPortfolioRequest> {
    pub fn opt_pretty(mut self, opt_pretty: bool) -> Self {
        self.params.opt_pretty = Some(opt_pretty);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, AddCustomFieldSettingForPortfolioRequest> {
    type Output = httpclient::InMemoryResult<AddCustomFieldSettingForPortfolioResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/portfolios/{portfolio_gid}/addCustomFieldSetting", portfolio_gid = self
                .params.portfolio_gid
            );
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "data" : self.params.data }));
            if let Some(ref unwrapped) = self.params.opt_pretty {
                r = r.query("opt_pretty", &unwrapped.to_string());
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
