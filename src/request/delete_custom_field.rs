use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AsanaClient;
/**You should use this struct via [`AsanaClient::delete_custom_field`].

On request success, this will return a [`DeleteCustomFieldResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteCustomFieldRequest {
    pub custom_field_gid: String,
    pub opt_pretty: Option<bool>,
}
impl DeleteCustomFieldRequest {}
impl FluentRequest<'_, DeleteCustomFieldRequest> {
    pub fn opt_pretty(mut self, opt_pretty: bool) -> Self {
        self.params.opt_pretty = Some(opt_pretty);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, DeleteCustomFieldRequest> {
    type Output = httpclient::InMemoryResult<DeleteCustomFieldResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/custom_fields/{custom_field_gid}", custom_field_gid = self.params
                .custom_field_gid
            );
            let mut r = self.client.client.delete(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
