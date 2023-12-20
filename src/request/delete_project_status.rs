use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AsanaClient;
/**You should use this struct via [`AsanaClient::delete_project_status`].

On request success, this will return a [`DeleteProjectStatusResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteProjectStatusRequest {
    pub opt_pretty: Option<bool>,
    pub project_status_gid: String,
}
impl DeleteProjectStatusRequest {}
impl FluentRequest<'_, DeleteProjectStatusRequest> {
    pub fn opt_pretty(mut self, opt_pretty: bool) -> Self {
        self.params.opt_pretty = Some(opt_pretty);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, DeleteProjectStatusRequest> {
    type Output = httpclient::InMemoryResult<DeleteProjectStatusResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/project_statuses/{project_status_gid}", project_status_gid = self
                .params.project_status_gid
            );
            let mut r = self.client.client.delete(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}