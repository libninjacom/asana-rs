use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AsanaClient;
/**You should use this struct via [`AsanaClient::get_portfolio_membership`].

On request success, this will return a [`GetPortfolioMembershipResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPortfolioMembershipRequest {
    pub opt_fields: Option<Vec<String>>,
    pub opt_pretty: Option<bool>,
    pub portfolio_membership_gid: String,
}
impl GetPortfolioMembershipRequest {}
impl FluentRequest<'_, GetPortfolioMembershipRequest> {
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
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, GetPortfolioMembershipRequest> {
    type Output = httpclient::InMemoryResult<GetPortfolioMembershipResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/portfolio_memberships/{portfolio_membership_gid}",
                portfolio_membership_gid = self.params.portfolio_membership_gid
            );
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}