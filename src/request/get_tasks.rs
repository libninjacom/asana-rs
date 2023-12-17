use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AsanaClient;
/**You should use this struct via [`AsanaClient::get_tasks`].

On request success, this will return a [`GetTasksResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTasksRequest {
    pub assignee: Option<String>,
    pub completed_since: Option<chrono::DateTime<chrono::Utc>>,
    pub limit: Option<i64>,
    pub modified_since: Option<chrono::DateTime<chrono::Utc>>,
    pub offset: Option<String>,
    pub opt_fields: Option<Vec<String>>,
    pub opt_pretty: Option<bool>,
    pub project: Option<String>,
    pub section: Option<String>,
    pub workspace: Option<String>,
}
impl GetTasksRequest {}
impl FluentRequest<'_, GetTasksRequest> {
    pub fn assignee(mut self, assignee: &str) -> Self {
        self.params.assignee = Some(assignee.to_owned());
        self
    }
    pub fn completed_since(
        mut self,
        completed_since: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        self.params.completed_since = Some(completed_since);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.params.limit = Some(limit);
        self
    }
    pub fn modified_since(
        mut self,
        modified_since: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        self.params.modified_since = Some(modified_since);
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
    pub fn project(mut self, project: &str) -> Self {
        self.params.project = Some(project.to_owned());
        self
    }
    pub fn section(mut self, section: &str) -> Self {
        self.params.section = Some(section.to_owned());
        self
    }
    pub fn workspace(mut self, workspace: &str) -> Self {
        self.params.workspace = Some(workspace.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, GetTasksRequest> {
    type Output = httpclient::InMemoryResult<GetTasksResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/tasks";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}