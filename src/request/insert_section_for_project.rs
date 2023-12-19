use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AsanaClient;
/**You should use this struct via [`AsanaClient::insert_section_for_project`].

On request success, this will return a [`InsertSectionForProjectResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertSectionForProjectRequest {
    pub data: ProjectSectionInsertRequest,
    pub opt_pretty: Option<bool>,
    pub project_gid: String,
}
impl InsertSectionForProjectRequest {}
impl FluentRequest<'_, InsertSectionForProjectRequest> {
    pub fn opt_pretty(mut self, opt_pretty: bool) -> Self {
        self.params.opt_pretty = Some(opt_pretty);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, InsertSectionForProjectRequest> {
    type Output = httpclient::InMemoryResult<InsertSectionForProjectResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/projects/{project_gid}/sections/insert", project_gid = self.params
                .project_gid
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
