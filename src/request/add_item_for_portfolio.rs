use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AsanaClient;
/**You should use this struct via [`AsanaClient::add_item_for_portfolio`].

On request success, this will return a [`AddItemForPortfolioResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddItemForPortfolioRequest {
    pub data: Option<PortfolioAddItemRequest>,
    pub opt_pretty: Option<bool>,
    pub portfolio_gid: String,
}
impl AddItemForPortfolioRequest {}
impl FluentRequest<'_, AddItemForPortfolioRequest> {
    pub fn data(mut self, data: PortfolioAddItemRequest) -> Self {
        self.params.data = Some(data);
        self
    }
    pub fn opt_pretty(mut self, opt_pretty: bool) -> Self {
        self.params.opt_pretty = Some(opt_pretty);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, AddItemForPortfolioRequest> {
    type Output = httpclient::InMemoryResult<AddItemForPortfolioResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/portfolios/{portfolio_gid}/addItem", portfolio_gid = self.params
                .portfolio_gid
            );
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.data {
                r = r.json(json!({ "data" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.opt_pretty {
                r = r.query("opt_pretty", &unwrapped.to_string());
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}