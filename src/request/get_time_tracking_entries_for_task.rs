use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AsanaClient;
/**You should use this struct via [`AsanaClient::get_time_tracking_entries_for_task`].

On request success, this will return a [`GetTimeTrackingEntriesForTaskResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTimeTrackingEntriesForTaskRequest {
    pub limit: Option<i64>,
    pub offset: Option<String>,
    pub opt_fields: Option<Vec<String>>,
    pub opt_pretty: Option<bool>,
    pub task_gid: String,
}
impl GetTimeTrackingEntriesForTaskRequest {}
impl FluentRequest<'_, GetTimeTrackingEntriesForTaskRequest> {
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
for FluentRequest<'a, GetTimeTrackingEntriesForTaskRequest> {
    type Output = httpclient::InMemoryResult<GetTimeTrackingEntriesForTaskResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/tasks/{task_gid}/time_tracking_entries", task_gid = self.params
                .task_gid
            );
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
