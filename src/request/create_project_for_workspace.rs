use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AsanaClient;
/**You should use this struct via [`AsanaClient::create_project_for_workspace`].

On request success, this will return a [`CreateProjectForWorkspaceResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProjectForWorkspaceRequest {
    pub data: ProjectRequest,
    pub opt_fields: Option<Vec<String>>,
    pub opt_pretty: Option<bool>,
    pub workspace_gid: String,
}
impl CreateProjectForWorkspaceRequest {}
impl FluentRequest<'_, CreateProjectForWorkspaceRequest> {
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
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, CreateProjectForWorkspaceRequest> {
    type Output = httpclient::InMemoryResult<CreateProjectForWorkspaceResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/workspaces/{workspace_gid}/projects", workspace_gid = self.params
                .workspace_gid
            );
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "data" : self.params.data }));
            if let Some(ref unwrapped) = self.params.opt_fields {
                for item in unwrapped {
                    r = r.query("opt_fields[]", &item.to_string());
                }
            }
            if let Some(ref unwrapped) = self.params.opt_pretty {
                r = r.query("opt_pretty", &unwrapped.to_string());
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}
