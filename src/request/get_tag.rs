use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AsanaClient;
/**You should use this struct via [`AsanaClient::get_tag`].

On request success, this will return a [`GetTagResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTagRequest {
    pub opt_fields: Option<Vec<String>>,
    pub opt_pretty: Option<bool>,
    pub tag_gid: String,
}
impl GetTagRequest {}
impl FluentRequest<'_, GetTagRequest> {
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
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, GetTagRequest> {
    type Output = httpclient::InMemoryResult<GetTagResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!("/tags/{tag_gid}", tag_gid = self.params.tag_gid);
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}