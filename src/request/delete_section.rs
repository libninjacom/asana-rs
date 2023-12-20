use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AsanaClient;
/**You should use this struct via [`AsanaClient::delete_section`].

On request success, this will return a [`DeleteSectionResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSectionRequest {
    pub opt_pretty: Option<bool>,
    pub section_gid: String,
}
impl DeleteSectionRequest {}
impl FluentRequest<'_, DeleteSectionRequest> {
    pub fn opt_pretty(mut self, opt_pretty: bool) -> Self {
        self.params.opt_pretty = Some(opt_pretty);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, DeleteSectionRequest> {
    type Output = httpclient::InMemoryResult<DeleteSectionResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/sections/{section_gid}", section_gid = self.params.section_gid
            );
            let mut r = self.client.client.delete(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}