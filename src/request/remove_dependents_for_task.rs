use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AsanaClient;
/**You should use this struct via [`AsanaClient::remove_dependents_for_task`].

On request success, this will return a [`RemoveDependentsForTaskResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveDependentsForTaskRequest {
    pub data: Option<ModifyDependentsRequest>,
    pub opt_pretty: Option<bool>,
    pub task_gid: String,
}
impl RemoveDependentsForTaskRequest {}
impl FluentRequest<'_, RemoveDependentsForTaskRequest> {
    pub fn data(mut self, data: ModifyDependentsRequest) -> Self {
        self.params.data = Some(data);
        self
    }
    pub fn opt_pretty(mut self, opt_pretty: bool) -> Self {
        self.params.opt_pretty = Some(opt_pretty);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, RemoveDependentsForTaskRequest> {
    type Output = httpclient::InMemoryResult<RemoveDependentsForTaskResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/tasks/{task_gid}/removeDependents", task_gid = self.params.task_gid
            );
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.data {
                r = r.json(json!({ "data" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.opt_pretty {
                r = r.query("opt_pretty", &unwrapped.to_string());
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}