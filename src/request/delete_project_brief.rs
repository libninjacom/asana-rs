use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AsanaClient;
/**You should use this struct via [`AsanaClient::delete_project_brief`].

On request success, this will return a [`DeleteProjectBriefResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteProjectBriefRequest {
    pub opt_pretty: Option<bool>,
    pub project_brief_gid: String,
}
impl DeleteProjectBriefRequest {}
impl FluentRequest<'_, DeleteProjectBriefRequest> {
    pub fn opt_pretty(mut self, opt_pretty: bool) -> Self {
        self.params.opt_pretty = Some(opt_pretty);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, DeleteProjectBriefRequest> {
    type Output = httpclient::InMemoryResult<DeleteProjectBriefResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/project_briefs/{project_brief_gid}", project_brief_gid = self.params
                .project_brief_gid
            );
            let mut r = self.client.client.delete(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
