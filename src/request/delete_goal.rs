use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AsanaClient;
/**You should use this struct via [`AsanaClient::delete_goal`].

On request success, this will return a [`DeleteGoalResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteGoalRequest {
    pub goal_gid: String,
    pub opt_pretty: Option<bool>,
}
impl DeleteGoalRequest {}
impl FluentRequest<'_, DeleteGoalRequest> {
    pub fn opt_pretty(mut self, opt_pretty: bool) -> Self {
        self.params.opt_pretty = Some(opt_pretty);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, DeleteGoalRequest> {
    type Output = httpclient::InMemoryResult<DeleteGoalResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!("/goals/{goal_gid}", goal_gid = self.params.goal_gid);
            let mut r = self.client.client.delete(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
