use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AsanaClient;
/**You should use this struct via [`AsanaClient::remove_item_for_portfolio`].

On request success, this will return a [`RemoveItemForPortfolioResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveItemForPortfolioRequest {
    pub data: Option<PortfolioRemoveItemRequest>,
    pub opt_pretty: Option<bool>,
    pub portfolio_gid: String,
}
impl RemoveItemForPortfolioRequest {}
impl FluentRequest<'_, RemoveItemForPortfolioRequest> {
    pub fn data(mut self, data: PortfolioRemoveItemRequest) -> Self {
        self.params.data = Some(data);
        self
    }
    pub fn opt_pretty(mut self, opt_pretty: bool) -> Self {
        self.params.opt_pretty = Some(opt_pretty);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, RemoveItemForPortfolioRequest> {
    type Output = httpclient::InMemoryResult<RemoveItemForPortfolioResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/portfolios/{portfolio_gid}/removeItem", portfolio_gid = self.params
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