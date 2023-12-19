use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AsanaClient;
/**You should use this struct via [`AsanaClient::get_tasks_for_user_task_list`].

On request success, this will return a [`GetTasksForUserTaskListResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTasksForUserTaskListRequest {
    pub completed_since: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<String>,
    pub opt_fields: Option<Vec<String>>,
    pub opt_pretty: Option<bool>,
    pub user_task_list_gid: String,
}
impl GetTasksForUserTaskListRequest {}
impl FluentRequest<'_, GetTasksForUserTaskListRequest> {
    pub fn completed_since(mut self, completed_since: &str) -> Self {
        self.params.completed_since = Some(completed_since.to_owned());
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
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, GetTasksForUserTaskListRequest> {
    type Output = httpclient::InMemoryResult<GetTasksForUserTaskListResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/user_task_lists/{user_task_list_gid}/tasks", user_task_list_gid = self
                .params.user_task_list_gid
            );
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
