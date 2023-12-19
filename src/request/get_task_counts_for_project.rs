use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AsanaClient;
/**You should use this struct via [`AsanaClient::get_task_counts_for_project`].

On request success, this will return a [`GetTaskCountsForProjectResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTaskCountsForProjectRequest {
    pub opt_fields: Option<Vec<String>>,
    pub opt_pretty: Option<bool>,
    pub project_gid: String,
}
impl GetTaskCountsForProjectRequest {}
impl FluentRequest<'_, GetTaskCountsForProjectRequest> {
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
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, GetTaskCountsForProjectRequest> {
    type Output = httpclient::InMemoryResult<GetTaskCountsForProjectResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/projects/{project_gid}/task_counts", project_gid = self.params
                .project_gid
            );
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
