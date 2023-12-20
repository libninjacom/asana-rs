use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AsanaClient;
/**You should use this struct via [`AsanaClient::add_task_for_section`].

On request success, this will return a [`AddTaskForSectionResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddTaskForSectionRequest {
    pub data: SectionTaskInsertRequest,
    pub opt_pretty: Option<bool>,
    pub section_gid: String,
}
impl AddTaskForSectionRequest {}
impl FluentRequest<'_, AddTaskForSectionRequest> {
    pub fn opt_pretty(mut self, opt_pretty: bool) -> Self {
        self.params.opt_pretty = Some(opt_pretty);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, AddTaskForSectionRequest> {
    type Output = httpclient::InMemoryResult<AddTaskForSectionResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/sections/{section_gid}/addTask", section_gid = self.params.section_gid
            );
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "data" : self.params.data }));
            if let Some(ref unwrapped) = self.params.opt_pretty {
                r = r.query("opt_pretty", &unwrapped.to_string());
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}