use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AsanaClient;
/**You should use this struct via [`AsanaClient::typeahead_for_workspace`].

On request success, this will return a [`TypeaheadForWorkspaceResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeaheadForWorkspaceRequest {
    pub count: Option<i64>,
    pub opt_fields: Option<Vec<String>>,
    pub opt_pretty: Option<bool>,
    pub query: Option<String>,
    pub resource_type: String,
    pub type_: Option<String>,
    pub workspace_gid: String,
}
impl TypeaheadForWorkspaceRequest {}
impl FluentRequest<'_, TypeaheadForWorkspaceRequest> {
    pub fn count(mut self, count: i64) -> Self {
        self.params.count = Some(count);
        self
    }
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
    pub fn query(mut self, query: &str) -> Self {
        self.params.query = Some(query.to_owned());
        self
    }
    pub fn type_(mut self, type_: &str) -> Self {
        self.params.type_ = Some(type_.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, TypeaheadForWorkspaceRequest> {
    type Output = httpclient::InMemoryResult<TypeaheadForWorkspaceResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/workspaces/{workspace_gid}/typeahead", workspace_gid = self.params
                .workspace_gid
            );
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}