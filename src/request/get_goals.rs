use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AsanaClient;
/**You should use this struct via [`AsanaClient::get_goals`].

On request success, this will return a [`GetGoalsResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetGoalsRequest {
    pub is_workspace_level: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<String>,
    pub opt_fields: Option<Vec<String>>,
    pub opt_pretty: Option<bool>,
    pub portfolio: Option<String>,
    pub project: Option<String>,
    pub task: Option<String>,
    pub team: Option<String>,
    pub time_periods: Option<Vec<String>>,
    pub workspace: Option<String>,
}
impl GetGoalsRequest {}
impl FluentRequest<'_, GetGoalsRequest> {
    pub fn is_workspace_level(mut self, is_workspace_level: bool) -> Self {
        self.params.is_workspace_level = Some(is_workspace_level);
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
    pub fn portfolio(mut self, portfolio: &str) -> Self {
        self.params.portfolio = Some(portfolio.to_owned());
        self
    }
    pub fn project(mut self, project: &str) -> Self {
        self.params.project = Some(project.to_owned());
        self
    }
    pub fn task(mut self, task: &str) -> Self {
        self.params.task = Some(task.to_owned());
        self
    }
    pub fn team(mut self, team: &str) -> Self {
        self.params.team = Some(team.to_owned());
        self
    }
    pub fn time_periods(
        mut self,
        time_periods: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .params
            .time_periods = Some(
            time_periods.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn workspace(mut self, workspace: &str) -> Self {
        self.params.workspace = Some(workspace.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, GetGoalsRequest> {
    type Output = httpclient::InMemoryResult<GetGoalsResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/goals";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}