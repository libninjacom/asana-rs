use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AsanaClient;
/**You should use this struct via [`AsanaClient::delete_membership`].

On request success, this will return a [`DeleteMembershipResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteMembershipRequest {
    pub membership_gid: String,
    pub opt_pretty: Option<bool>,
}
impl DeleteMembershipRequest {}
impl FluentRequest<'_, DeleteMembershipRequest> {
    pub fn opt_pretty(mut self, opt_pretty: bool) -> Self {
        self.params.opt_pretty = Some(opt_pretty);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, DeleteMembershipRequest> {
    type Output = httpclient::InMemoryResult<DeleteMembershipResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/memberships/{membership_gid}", membership_gid = self.params
                .membership_gid
            );
            let mut r = self.client.client.delete(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
