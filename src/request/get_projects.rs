use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AsanaClient;
/**You should use this struct via [`AsanaClient::get_projects`].

On request success, this will return a [`GetProjectsResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetProjectsRequest {
    pub archived: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<String>,
    pub opt_fields: Option<Vec<String>>,
    pub opt_pretty: Option<bool>,
    pub team: Option<String>,
    pub workspace: Option<String>,
}
impl GetProjectsRequest {}
impl FluentRequest<'_, GetProjectsRequest> {
    pub fn archived(mut self, archived: bool) -> Self {
        self.params.archived = Some(archived);
        self
    }
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
    pub fn team(mut self, team: &str) -> Self {
        self.params.team = Some(team.to_owned());
        self
    }
    pub fn workspace(mut self, workspace: &str) -> Self {
        self.params.workspace = Some(workspace.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, GetProjectsRequest> {
    type Output = httpclient::InMemoryResult<GetProjectsResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/projects";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}