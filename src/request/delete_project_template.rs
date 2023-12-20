use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AsanaClient;
/**You should use this struct via [`AsanaClient::delete_project_template`].

On request success, this will return a [`DeleteProjectTemplateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteProjectTemplateRequest {
    pub opt_pretty: Option<bool>,
    pub project_template_gid: String,
}
impl DeleteProjectTemplateRequest {}
impl FluentRequest<'_, DeleteProjectTemplateRequest> {
    pub fn opt_pretty(mut self, opt_pretty: bool) -> Self {
        self.params.opt_pretty = Some(opt_pretty);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, DeleteProjectTemplateRequest> {
    type Output = httpclient::InMemoryResult<DeleteProjectTemplateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/project_templates/{project_template_gid}", project_template_gid = self
                .params.project_template_gid
            );
            let mut r = self.client.client.delete(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}