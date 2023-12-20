use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AsanaClient;
/**You should use this struct via [`AsanaClient::remove_supporting_relationship`].

On request success, this will return a [`RemoveSupportingRelationshipResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveSupportingRelationshipRequest {
    pub data: GoalRemoveSupportingRelationshipRequest,
    pub goal_gid: String,
    pub opt_pretty: Option<bool>,
}
impl RemoveSupportingRelationshipRequest {}
impl FluentRequest<'_, RemoveSupportingRelationshipRequest> {
    pub fn opt_pretty(mut self, opt_pretty: bool) -> Self {
        self.params.opt_pretty = Some(opt_pretty);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, RemoveSupportingRelationshipRequest> {
    type Output = httpclient::InMemoryResult<RemoveSupportingRelationshipResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/goals/{goal_gid}/removeSupportingRelationship", goal_gid = self.params
                .goal_gid
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