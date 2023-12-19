use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AsanaClient;
/**You should use this struct via [`AsanaClient::get_portfolio_memberships`].

On request success, this will return a [`GetPortfolioMembershipsResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPortfolioMembershipsRequest {
    pub limit: Option<i64>,
    pub offset: Option<String>,
    pub opt_fields: Option<Vec<String>>,
    pub opt_pretty: Option<bool>,
    pub portfolio: Option<String>,
    pub user: Option<String>,
    pub workspace: Option<String>,
}
impl GetPortfolioMembershipsRequest {}
impl FluentRequest<'_, GetPortfolioMembershipsRequest> {
    pub fn limit(mut self, limit: i64) -> Self {
        self.params.limit = Some(limit);
        self
    }
    pub fn offset(mut self, offset: &str) -> Self {
        self.params.offset = Some(offset.to_owned());
        self
    }
    pub fn opt_fields(
        mut self,
        opt_fields: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .params
            .opt_fields = Some(
            opt_fields.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn opt_pretty(mut self, opt_pretty: bool) -> Self {
        self.params.opt_pretty = Some(opt_pretty);
        self
    }
    pub fn portfolio(mut self, portfolio: &str) -> Self {
        self.params.portfolio = Some(portfolio.to_owned());
        self
    }
    pub fn user(mut self, user: &str) -> Self {
        self.params.user = Some(user.to_owned());
        self
    }
    pub fn workspace(mut self, workspace: &str) -> Self {
        self.params.workspace = Some(workspace.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, GetPortfolioMembershipsRequest> {
    type Output = httpclient::InMemoryResult<GetPortfolioMembershipsResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/portfolio_memberships";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
