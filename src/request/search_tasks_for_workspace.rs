use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AsanaClient;
/**You should use this struct via [`AsanaClient::search_tasks_for_workspace`].

On request success, this will return a [`SearchTasksForWorkspaceResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchTasksForWorkspaceRequest {
    pub assigned_by_any: Option<String>,
    pub assigned_by_not: Option<String>,
    pub assignee_any: Option<String>,
    pub assignee_not: Option<String>,
    pub commented_on_by_not: Option<String>,
    pub completed: Option<bool>,
    pub completed_at_after: Option<chrono::DateTime<chrono::Utc>>,
    pub completed_at_before: Option<chrono::DateTime<chrono::Utc>>,
    pub completed_on: Option<chrono::NaiveDate>,
    pub completed_on_after: Option<chrono::NaiveDate>,
    pub completed_on_before: Option<chrono::NaiveDate>,
    pub created_at_after: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at_before: Option<chrono::DateTime<chrono::Utc>>,
    pub created_by_any: Option<String>,
    pub created_by_not: Option<String>,
    pub created_on: Option<chrono::NaiveDate>,
    pub created_on_after: Option<chrono::NaiveDate>,
    pub created_on_before: Option<chrono::NaiveDate>,
    pub due_at_after: Option<chrono::DateTime<chrono::Utc>>,
    pub due_at_before: Option<chrono::DateTime<chrono::Utc>>,
    pub due_on: Option<chrono::NaiveDate>,
    pub due_on_after: Option<chrono::NaiveDate>,
    pub due_on_before: Option<chrono::NaiveDate>,
    pub followers_not: Option<String>,
    pub has_attachment: Option<bool>,
    pub is_blocked: Option<bool>,
    pub is_blocking: Option<bool>,
    pub is_subtask: Option<bool>,
    pub liked_by_not: Option<String>,
    pub modified_at_after: Option<chrono::DateTime<chrono::Utc>>,
    pub modified_at_before: Option<chrono::DateTime<chrono::Utc>>,
    pub modified_on: Option<chrono::NaiveDate>,
    pub modified_on_after: Option<chrono::NaiveDate>,
    pub modified_on_before: Option<chrono::NaiveDate>,
    pub opt_fields: Option<Vec<String>>,
    pub opt_pretty: Option<bool>,
    pub portfolios_any: Option<String>,
    pub projects_all: Option<String>,
    pub projects_any: Option<String>,
    pub projects_not: Option<String>,
    pub resource_subtype: Option<String>,
    pub sections_all: Option<String>,
    pub sections_any: Option<String>,
    pub sections_not: Option<String>,
    pub sort_ascending: Option<bool>,
    pub sort_by: Option<String>,
    pub start_on: Option<chrono::NaiveDate>,
    pub start_on_after: Option<chrono::NaiveDate>,
    pub start_on_before: Option<chrono::NaiveDate>,
    pub tags_all: Option<String>,
    pub tags_any: Option<String>,
    pub tags_not: Option<String>,
    pub teams_any: Option<String>,
    pub text: Option<String>,
    pub workspace_gid: String,
}
impl SearchTasksForWorkspaceRequest {}
impl FluentRequest<'_, SearchTasksForWorkspaceRequest> {
    pub fn assigned_by_any(mut self, assigned_by_any: &str) -> Self {
        self.params.assigned_by_any = Some(assigned_by_any.to_owned());
        self
    }
    pub fn assigned_by_not(mut self, assigned_by_not: &str) -> Self {
        self.params.assigned_by_not = Some(assigned_by_not.to_owned());
        self
    }
    pub fn assignee_any(mut self, assignee_any: &str) -> Self {
        self.params.assignee_any = Some(assignee_any.to_owned());
        self
    }
    pub fn assignee_not(mut self, assignee_not: &str) -> Self {
        self.params.assignee_not = Some(assignee_not.to_owned());
        self
    }
    pub fn commented_on_by_not(mut self, commented_on_by_not: &str) -> Self {
        self.params.commented_on_by_not = Some(commented_on_by_not.to_owned());
        self
    }
    pub fn completed(mut self, completed: bool) -> Self {
        self.params.completed = Some(completed);
        self
    }
    pub fn completed_at_after(
        mut self,
        completed_at_after: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        self.params.completed_at_after = Some(completed_at_after);
        self
    }
    pub fn completed_at_before(
        mut self,
        completed_at_before: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        self.params.completed_at_before = Some(completed_at_before);
        self
    }
    pub fn completed_on(mut self, completed_on: chrono::NaiveDate) -> Self {
        self.params.completed_on = Some(completed_on);
        self
    }
    pub fn completed_on_after(mut self, completed_on_after: chrono::NaiveDate) -> Self {
        self.params.completed_on_after = Some(completed_on_after);
        self
    }
    pub fn completed_on_before(
        mut self,
        completed_on_before: chrono::NaiveDate,
    ) -> Self {
        self.params.completed_on_before = Some(completed_on_before);
        self
    }
    pub fn created_at_after(
        mut self,
        created_at_after: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        self.params.created_at_after = Some(created_at_after);
        self
    }
    pub fn created_at_before(
        mut self,
        created_at_before: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        self.params.created_at_before = Some(created_at_before);
        self
    }
    pub fn created_by_any(mut self, created_by_any: &str) -> Self {
        self.params.created_by_any = Some(created_by_any.to_owned());
        self
    }
    pub fn created_by_not(mut self, created_by_not: &str) -> Self {
        self.params.created_by_not = Some(created_by_not.to_owned());
        self
    }
    pub fn created_on(mut self, created_on: chrono::NaiveDate) -> Self {
        self.params.created_on = Some(created_on);
        self
    }
    pub fn created_on_after(mut self, created_on_after: chrono::NaiveDate) -> Self {
        self.params.created_on_after = Some(created_on_after);
        self
    }
    pub fn created_on_before(mut self, created_on_before: chrono::NaiveDate) -> Self {
        self.params.created_on_before = Some(created_on_before);
        self
    }
    pub fn due_at_after(mut self, due_at_after: chrono::DateTime<chrono::Utc>) -> Self {
        self.params.due_at_after = Some(due_at_after);
        self
    }
    pub fn due_at_before(
        mut self,
        due_at_before: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        self.params.due_at_before = Some(due_at_before);
        self
    }
    pub fn due_on(mut self, due_on: chrono::NaiveDate) -> Self {
        self.params.due_on = Some(due_on);
        self
    }
    pub fn due_on_after(mut self, due_on_after: chrono::NaiveDate) -> Self {
        self.params.due_on_after = Some(due_on_after);
        self
    }
    pub fn due_on_before(mut self, due_on_before: chrono::NaiveDate) -> Self {
        self.params.due_on_before = Some(due_on_before);
        self
    }
    pub fn followers_not(mut self, followers_not: &str) -> Self {
        self.params.followers_not = Some(followers_not.to_owned());
        self
    }
    pub fn has_attachment(mut self, has_attachment: bool) -> Self {
        self.params.has_attachment = Some(has_attachment);
        self
    }
    pub fn is_blocked(mut self, is_blocked: bool) -> Self {
        self.params.is_blocked = Some(is_blocked);
        self
    }
    pub fn is_blocking(mut self, is_blocking: bool) -> Self {
        self.params.is_blocking = Some(is_blocking);
        self
    }
    pub fn is_subtask(mut self, is_subtask: bool) -> Self {
        self.params.is_subtask = Some(is_subtask);
        self
    }
    pub fn liked_by_not(mut self, liked_by_not: &str) -> Self {
        self.params.liked_by_not = Some(liked_by_not.to_owned());
        self
    }
    pub fn modified_at_after(
        mut self,
        modified_at_after: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        self.params.modified_at_after = Some(modified_at_after);
        self
    }
    pub fn modified_at_before(
        mut self,
        modified_at_before: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        self.params.modified_at_before = Some(modified_at_before);
        self
    }
    pub fn modified_on(mut self, modified_on: chrono::NaiveDate) -> Self {
        self.params.modified_on = Some(modified_on);
        self
    }
    pub fn modified_on_after(mut self, modified_on_after: chrono::NaiveDate) -> Self {
        self.params.modified_on_after = Some(modified_on_after);
        self
    }
    pub fn modified_on_before(mut self, modified_on_before: chrono::NaiveDate) -> Self {
        self.params.modified_on_before = Some(modified_on_before);
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
    pub fn portfolios_any(mut self, portfolios_any: &str) -> Self {
        self.params.portfolios_any = Some(portfolios_any.to_owned());
        self
    }
    pub fn projects_all(mut self, projects_all: &str) -> Self {
        self.params.projects_all = Some(projects_all.to_owned());
        self
    }
    pub fn projects_any(mut self, projects_any: &str) -> Self {
        self.params.projects_any = Some(projects_any.to_owned());
        self
    }
    pub fn projects_not(mut self, projects_not: &str) -> Self {
        self.params.projects_not = Some(projects_not.to_owned());
        self
    }
    pub fn resource_subtype(mut self, resource_subtype: &str) -> Self {
        self.params.resource_subtype = Some(resource_subtype.to_owned());
        self
    }
    pub fn sections_all(mut self, sections_all: &str) -> Self {
        self.params.sections_all = Some(sections_all.to_owned());
        self
    }
    pub fn sections_any(mut self, sections_any: &str) -> Self {
        self.params.sections_any = Some(sections_any.to_owned());
        self
    }
    pub fn sections_not(mut self, sections_not: &str) -> Self {
        self.params.sections_not = Some(sections_not.to_owned());
        self
    }
    pub fn sort_ascending(mut self, sort_ascending: bool) -> Self {
        self.params.sort_ascending = Some(sort_ascending);
        self
    }
    pub fn sort_by(mut self, sort_by: &str) -> Self {
        self.params.sort_by = Some(sort_by.to_owned());
        self
    }
    pub fn start_on(mut self, start_on: chrono::NaiveDate) -> Self {
        self.params.start_on = Some(start_on);
        self
    }
    pub fn start_on_after(mut self, start_on_after: chrono::NaiveDate) -> Self {
        self.params.start_on_after = Some(start_on_after);
        self
    }
    pub fn start_on_before(mut self, start_on_before: chrono::NaiveDate) -> Self {
        self.params.start_on_before = Some(start_on_before);
        self
    }
    pub fn tags_all(mut self, tags_all: &str) -> Self {
        self.params.tags_all = Some(tags_all.to_owned());
        self
    }
    pub fn tags_any(mut self, tags_any: &str) -> Self {
        self.params.tags_any = Some(tags_any.to_owned());
        self
    }
    pub fn tags_not(mut self, tags_not: &str) -> Self {
        self.params.tags_not = Some(tags_not.to_owned());
        self
    }
    pub fn teams_any(mut self, teams_any: &str) -> Self {
        self.params.teams_any = Some(teams_any.to_owned());
        self
    }
    pub fn text(mut self, text: &str) -> Self {
        self.params.text = Some(text.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, SearchTasksForWorkspaceRequest> {
    type Output = httpclient::InMemoryResult<SearchTasksForWorkspaceResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/workspaces/{workspace_gid}/tasks/search", workspace_gid = self.params
                .workspace_gid
            );
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}