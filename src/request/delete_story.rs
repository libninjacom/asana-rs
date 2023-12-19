use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AsanaClient;
/**You should use this struct via [`AsanaClient::delete_story`].

On request success, this will return a [`DeleteStoryResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteStoryRequest {
    pub opt_pretty: Option<bool>,
    pub story_gid: String,
}
impl DeleteStoryRequest {}
impl FluentRequest<'_, DeleteStoryRequest> {
    pub fn opt_pretty(mut self, opt_pretty: bool) -> Self {
        self.params.opt_pretty = Some(opt_pretty);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, DeleteStoryRequest> {
    type Output = httpclient::InMemoryResult<DeleteStoryResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/stories/{story_gid}", story_gid = self.params.story_gid
            );
            let mut r = self.client.client.delete(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
