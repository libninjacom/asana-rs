use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AsanaClient;
/**You should use this struct via [`AsanaClient::get_goal_relationship`].

On request success, this will return a [`GetGoalRelationshipResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetGoalRelationshipRequest {
    pub goal_relationship_gid: String,
    pub opt_fields: Option<Vec<String>>,
    pub opt_pretty: Option<bool>,
}
impl GetGoalRelationshipRequest {}
impl FluentRequest<'_, GetGoalRelationshipRequest> {
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
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, GetGoalRelationshipRequest> {
    type Output = httpclient::InMemoryResult<GetGoalRelationshipResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/goal_relationships/{goal_relationship_gid}", goal_relationship_gid =
                self.params.goal_relationship_gid
            );
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
