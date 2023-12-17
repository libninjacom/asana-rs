use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AsanaClient;
/**You should use this struct via [`AsanaClient::delete_status`].

On request success, this will return a [`DeleteStatusResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteStatusRequest {
    pub opt_pretty: Option<bool>,
    pub status_update_gid: String,
}
impl DeleteStatusRequest {}
impl FluentRequest<'_, DeleteStatusRequest> {
    pub fn opt_pretty(mut self, opt_pretty: bool) -> Self {
        self.params.opt_pretty = Some(opt_pretty);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, DeleteStatusRequest> {
    type Output = httpclient::InMemoryResult<DeleteStatusResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/status_updates/{status_update_gid}", status_update_gid = self.params
                .status_update_gid
            );
            let mut r = self.client.client.delete(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}