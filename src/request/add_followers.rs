use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::AsanaClient;
/**You should use this struct via [`AsanaClient::add_followers`].

On request success, this will return a [`AddFollowersResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddFollowersRequest {
    pub data: Option<TaskAddFollowersRequest>,
    pub goal_gid: String,
    pub opt_fields: Option<Vec<String>>,
    pub opt_pretty: Option<bool>,
}
impl AddFollowersRequest {}
impl FluentRequest<'_, AddFollowersRequest> {
    pub fn data(mut self, data: TaskAddFollowersRequest) -> Self {
        self.params.data = Some(data);
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
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, AddFollowersRequest> {
    type Output = httpclient::InMemoryResult<AddFollowersResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/goals/{goal_gid}/addFollowers", goal_gid = self.params.goal_gid
            );
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.data {
                r = r.json(json!({ "data" : unwrapped }));
            }
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